#include <stddef.h>
#include <stdint.h>
#include <riscv_vector.h>

#ifdef __cplusplus
extern "C" {
#endif
void* memset(void* dest, int ch, size_t count) {
    unsigned char ch_uc = (unsigned char)ch;
    unsigned char* dest_uc = (unsigned char*)dest;
    for (int i = 0; i < count; i++) {
        *(dest_uc + i) = ch_uc;
    }

    return dest_uc;
}

// Define a simple stdlib replacement of memcpy
void* memcpy(void* dest, const void* src, size_t count) {
    unsigned char* dest_uc = (unsigned char*) dest;
    const unsigned char* src_uc = (const unsigned char*) src;
    while(count > 0) {
        *dest_uc = *src_uc;
        dest_uc++;
        src_uc++;
        count--;
    }
    return dest;
}
#ifdef __cplusplus
}
#endif

#define ASM_PREG(val) "r"(val)
#define CAPABILITY_IF_SUPPORTED

// Patch over differences between GCC, clang, and CHERI-clang
#if defined(__llvm__)
// Clang intrinsics are correct for segmented loads and supports fractional LMUL.
// Clang 14+ has the correct intrinsics for bytemask loads,
// and Clang has been tested with wholereg ASM

    #if __clang_major__ >= 14
        #define ENABLE_BYTEMASKLOAD 1
    #else
        #define ENABLE_BYTEMASKLOAD 0
    #endif

    #if __has_feature(capabilities)
        #if __has_feature(pure_capabilities)
            // Replace the ASM pointer register function to use capability register
            #undef ASM_PREG
            #define ASM_PREG(val) "C"(val)
        #endif

        #undef CAPABILITY_IF_SUPPORTED
        #define CAPABILITY_IF_SUPPORTED __capability

        // Enable everything
        #define ENABLE_UNIT 1
        #define ENABLE_STRIDED 1
        #define ENABLE_INDEXED 1
        #define ENABLE_MASKED 1
        #define ENABLE_SEGMENTED 1
        #define ENABLE_FRAC_LMUL 1

        // Use ASM for everything
        #define USE_ASM_FOR_UNIT 1
        #define USE_ASM_FOR_STRIDED 1
        #define USE_ASM_FOR_INDEXED 1
        #define USE_ASM_FOR_MASKED 1
        #define USE_ASM_FOR_SEGMENTED 1

        #define ENABLE_FAULTONLYFIRST 0
        // This *should* work but LLVM complains about "invalid operand for instruction"
        #define ENABLE_ASM_WHOLEREG 0
    #else
        // Enable everything
        #define ENABLE_UNIT 1
        #define ENABLE_STRIDED 1
        #define ENABLE_INDEXED 1
        #define ENABLE_MASKED 1
        #define ENABLE_SEGMENTED 1
        #define ENABLE_FRAC_LMUL 1

        // Use intrinsics for everything
        #define USE_ASM_FOR_UNIT 0
        #define USE_ASM_FOR_STRIDED 0
        #define USE_ASM_FOR_INDEXED 0
        #define USE_ASM_FOR_MASKED 0
        #define USE_ASM_FOR_SEGMENTED 0

        #define ENABLE_FAULTONLYFIRST 1
        #define ENABLE_ASM_WHOLEREG 1
    #endif
#elif defined(__GNUC__) && !defined(__INTEL_COMPILER)
// GNU exts enabled, not in LLVM or Intel, => in GCC

// My version of GCC intrinsics doesn't have correct intrinsics for segmented loads,
// doesn't support fractional LMUL,
// doesn't have byte-mask intrinsics.

    // Enable everything except fractional LMUL
    #define ENABLE_UNIT 1
    #define ENABLE_STRIDED 1
    #define ENABLE_INDEXED 1
    #define ENABLE_MASKED 1
    #define ENABLE_SEGMENTED 1
    #define ENABLE_FRAC_LMUL 0

    // Use intrinsics for all except segmented loads and bytemask
    #define USE_ASM_FOR_UNIT 0
    #define USE_ASM_FOR_STRIDED 0
    #define USE_ASM_FOR_INDEXED 0
    #define USE_ASM_FOR_MASKED 0
    #define USE_ASM_FOR_SEGMENTED 1

#define ENABLE_BYTEMASKLOAD 0
// it doesn't seem to compile fault-only-first correctly
#define ENABLE_FAULTONLYFIRST 0
// it has been tested with the inline asm whole-register loads
#define ENABLE_ASM_WHOLEREG 1
#endif



// This file is a testbench for vector_memcpys of pointers
// It consructs an array of small Element structures, which have a pointer to one of many Base structures
// The Elements are copied into a second array, and the pointers they contain are tested to tell if they can still dereference the bases correctly

typedef struct {
    uint64_t value;
} Base;


typedef struct {
    // We check if the value contained in the Base is the same as it was before
    uint64_t expected_base_value;
    // Make sure the Base pointer is a capability if we're on a capability platform
    const Base* CAPABILITY_IF_SUPPORTED base_ptr;
} Element;

#if __has_feature(capabilities)
static_assert(alignof(Base* CAPABILITY_IF_SUPPORTED) == 128/8, "Base* capability should be 128-bit aligned");
static_assert(alignof(Element) == 128/8, "Element should be 128-bit aligned");
#endif

void vector_memcpy(uint8_t __attribute__((aligned(16)))* dst, const uint8_t __attribute__((aligned(16)))* src, size_t num_bytes) {
    // 128-bit instructions are only present on our modified version of CHERI-Clang
    #if __has_feature(capabilities)
    while (num_bytes >= 128) {
        size_t num_elements = num_bytes / 128;
        size_t copied_128byte_elems_per_iter;
        
        // Do the copy in assembly - didn't have enough time to add intrinsics
        asm volatile ("vsetvli %0, %1, e128, m8, tu, mu" : "=r"(copied_128byte_elems_per_iter) : "r"(num_elements));
        asm volatile ("vle128.v v8, (%0)" :: ASM_PREG(src));
        asm volatile ("vse128.v v8, (%0)" :: ASM_PREG(dst));

        src += copied_128byte_elems_per_iter * 128;
        dst += copied_128byte_elems_per_iter * 128;
        num_bytes -= copied_128byte_elems_per_iter * 128;
    }
    #endif
    // Remainder copy
    // These parts will not copy capabilities!
    while (num_bytes > 0) {
        size_t copied_per_iter = vsetvl_e8m8(num_bytes);

        vuint8m8_t data;
        #if USE_ASM_FOR_UNIT
        asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : ASM_PREG(src));
        asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  ASM_PREG(dst));
        #else
        data = vle8_v_u8m8(src, copied_per_iter);
        vse8_v_u8m8(dst, data, copied_per_iter);
        #endif // USE_ASM_FOR_UNIT

        src += copied_per_iter;
        dst += copied_per_iter;
        num_bytes -= copied_per_iter;
    }
}

int run_base_test(void) {

    // Random numbers
    // 746ef0f2a5b4975a 8ce7e0643a62b4a4 
    // 672799971c33ecde 94ff5c7c75ade697 
    const Base bases[4] = {
        { .value = 0x746ef0f2a5b4975a },
        { .value = 0x8ce7e0643a62b4a4 },
        { .value = 0x672799971c33ecde },
        { .value = 0x94ff5c7c75ade697 },
    };

    // Randomly generated
    // The index of the Base that each Element will point to
    int indices[128] = {
        1, 1, 1, 1, 2, 2, 3, 0,
        1, 0, 3, 0, 0, 3, 3, 0,
        1, 0, 0, 2, 2, 0, 2, 1,
        0, 0, 0, 3, 2, 0, 1, 1,
        3, 2, 3, 0, 2, 2, 0, 0,
        0, 1, 1, 3, 0, 0, 1, 3,
        1, 2, 3, 2, 2, 0, 2, 1,
        0, 3, 1, 1, 3, 3, 2, 2,
        0, 1, 3, 2, 2, 1, 1, 3,
        2, 2, 0, 1, 1, 3, 0, 1,
        0, 0, 3, 2, 2, 3, 3, 1,
        1, 1, 1, 2, 1, 1, 2, 1,
        2, 2, 1, 1, 3, 1, 1, 3,
        0, 2, 3, 1, 1, 3, 2, 3,
        2, 1, 2, 0, 2, 2, 2, 3,
        0, 3, 1, 0, 3, 2, 1, 0
    };

    Element source_array[128];
    for (size_t i = 0; i < 128; i++) {
        int index = indices[i];
        source_array[i] = Element {
            .expected_base_value = bases[index].value,
            .base_ptr = (const Base* CAPABILITY_IF_SUPPORTED) &bases[index]
        };
    }

    Element dest_array[128] = {0};

    Element* src_ptr = &source_array[0];
    Element* dst_ptr = &dest_array[0];

    // Don't force the pointers to the elements to be capabilities
    vector_memcpy((uint8_t*)dst_ptr, (const uint8_t*)src_ptr, sizeof(Element) * 128);

    // Check the resuls
    for (size_t i = 0; i < 128; i++) {
        // This makes sure the base_ptr is dereferenceable at all
        // and that it's actually a pointer to the value we meant
        if (dest_array[i].base_ptr->value != dest_array[i].expected_base_value) {
            return 0; // Failure
        }
        if (dest_array[i].base_ptr != (const Base* CAPABILITY_IF_SUPPORTED) &bases[indices[i]]) {
            return 0;
        }
    }
    return 1;
}

// Magical output devices, set by linker
volatile extern int outputAttempted;
volatile extern int outputSucceeded;

#ifdef __cplusplus
extern "C" {
#endif
int main(void)
{
    int result = 0;
    int attempted = 0;

    attempted |= 1 << 0;
    result |= run_base_test() << 0;

    *(&outputAttempted) = attempted;
    *(&outputSucceeded) = result;
    return result;
}
#ifdef __cplusplus
}
#endif
