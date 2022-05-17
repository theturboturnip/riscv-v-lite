#include "vec_wrappers.h"
#include <stdint.h>

// Patch over differences between GCC vector intrinsics and clang ones
#if defined(__GNUC__) && !defined(__llvm__) && !defined(__INTEL_COMPILER)
// GNU exts enabled, not in LLVM or Intel, => in GCC

// My version of GCC intrinsics doesn't have the same functions for segmented loads,
// doesn't support fractional LMUL,
// doesn't have byte-mask intrinsics
#define ENABLE_FRAC 0
#define ENABLE_STRIDED 1
#define ENABLE_INDEXED 1
#define ENABLE_MASKED 1
#define ENABLE_SEG 0
#define ENABLE_BYTEMASKLOAD 0
// it doesn't seem to compile fault-only-first correctly
#define ENABLE_FAULTONLYFIRST 0
// it has been tested with the inline asm whole-register loads
#define ENABLE_ASM_WHOLEREG 1
#define HAS_CAPABILITIES 0
#else
// Clang intrinsics are correct for segmented loads,
// supports fractional LMUL,
// clang 14+ has the correct intrinsics for bytemask loads,
// and clang has been tested with wholereg ASM
    #if __clang_major__ >= 14
        #define ENABLE_BYTEMASKLOAD 1
    #else
        #define ENABLE_BYTEMASKLOAD 0
    #endif

    #if __has_feature(capabilities)
    #define HAS_CAPABILITIES 1
    #define ENABLE_FRAC 1
    #define ENABLE_STRIDED 1
    // These haven't been ported to use CHERI-compatible VEC_INTRIN() wrappers
    #define ENABLE_INDEXED 0 // INDEXED needs non-hardcoded vector registers for loading the indices
    #define ENABLE_MASKED 0 // MASKED needs non-hardcoded vector registers for setting up the mask?
    #define ENABLE_SEG 0 // SEG needs non-hardcoded vector registers for individual stores
    // This doesn't quite work yet - it relies on making up an address out of thin air, which CHERI doesn't support :D
    #define ENABLE_FAULTONLYFIRST 0
    // This *should* work but LLVM complains about "invalid operand for instruction"
    #define ENABLE_ASM_WHOLEREG 0
    #else
    #define HAS_CAPABILITIES 0
    #define ENABLE_FRAC 1
    #define ENABLE_STRIDED 1
    #define ENABLE_INDEXED 1
    #define ENABLE_MASKED 1
    #define ENABLE_SEG 1
    #define ENABLE_FAULTONLYFIRST 1
    #define ENABLE_ASM_WHOLEREG 1
    #endif
#endif

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
#ifdef __cplusplus
}
#endif

#if ENABLE_INDEXED
void vector_memcpy_indexed(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    const size_t ELEM_WIDTH = 4;

    // Generate indices
    uint32_t indices[128] = {0};
    size_t vlmax = vsetvlmax_e32m4();

    for (size_t i = 0; i < vlmax; i++) {
        // Use xor to generate a shuffled index pattern
        // The index values are in terms of bytes
        indices[i] = (((uint32_t)i) ^ 1) * ELEM_WIDTH;
    }

    vuint32m4_t indices_v = vle32_v_u32m4(&indices[0], vlmax);

    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m4(n);

        if (copied_per_iter == vlmax) {
            // We know that using indices_v will cover all values from 0..copied_per_iter-1
            vint32m4_t data = vloxei32_v_i32m4(in, indices_v, copied_per_iter);
            vsoxei32_v_i32m4(out, indices_v, data, copied_per_iter);
        } else {
            // Because the indices are shuffled, just using [0-copied_per_iter] 
            // won't necessarily cover all 0..copied_per_iter-1 values.
            // Just use a standard load
            vint32m4_t data = vle32_v_i32m4(in, copied_per_iter);
            vse32_v_i32m4(out, data, copied_per_iter);
        }

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_INDEXED

#if ENABLE_MASKED
// special version that only copies odd-indexed elements
void vector_memcpy_masked(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    // Generate mask
    uint32_t mask_ints[128] = {0};
    size_t vlmax = vsetvlmax_e32m4();

    for (size_t i = 0; i < vlmax; i++) {
        // ...10101010
        mask_ints[i] = i & 1;
    }

    vuint32m4_t mask_ints_v = vle32_v_u32m4(&mask_ints[0], vlmax);
    // Where mask = 1, e.g. ...10101010
    // => only odd element get written
    vbool8_t mask = vmseq_vx_u32m4_b8(mask_ints_v, 1, vlmax);
    // All zeroes
    vint32m4_t vec_zero = vmv_v_x_i32m4(0, vlmax);


    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m4(n);

        vint32m4_t data = vle32_v_i32m4_m(mask, vec_zero, in, copied_per_iter);
        vse32_v_i32m4_m(mask, out, data, copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_MASKED

// See https://raw.githubusercontent.com/riscv-non-isa/rvv-intrinsic-doc/master/intrinsic_funcs.md
// vlm_v_bX and vsm_v_bX *should* exist as intrinsict for bytemask load/store,
// but LLVM-13 says they're 'implicit declarations' when I try and use them.
// LLVM-Trunk is required (e.g. LLVM 14+) to use them
#if ENABLE_BYTEMASKLOAD
void vector_memcpy_masked_bytemaskload(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    // Generate mask
    uint32_t mask_ints[128] = {0};
    size_t vlmax = vsetvlmax_e32m2();

    for (size_t i = 0; i < vlmax; i++) {
        // ...10101010
        mask_ints[i] = i & 1;
    }

    vuint32m2_t mask_ints_v = vle32_v_u32m2(&mask_ints[0], vlmax);
    // Where mask = 1, e.g. ...10101010
    // => only odd element get written
    vbool16_t mask_initial = vmseq_vx_u32m2_b16(mask_ints_v, 1, vlmax);
    // All zeroes
    vint32m2_t vec_zero = vmv_v_x_i32m2(0, vlmax);

    uint8_t big_old_buffer[128] = {0};
    // Store the mask in
    vsm_v_b16(big_old_buffer, mask_initial, vlmax);
    // Pull the mask back out
    vbool16_t mask = vlm_v_b16(big_old_buffer, vlmax);


    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m2(n);

        vint32m2_t data = vle32_v_i32m2_m(mask, vec_zero, in, copied_per_iter);
        vse32_v_i32m2_m(mask, out, data, copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_BYTEMASKLOAD

#if ENABLE_STRIDED
void vector_memcpy_8strided(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    const size_t STRIDE_FACTOR = 4;
    const size_t ELEM_WIDTH = 1;
    size_t copied_per_iter = 0;
    for (; n > 0; ) {
        copied_per_iter = vsetvl_e8m1(n*4);

        // If we have room to do so, copy STRIDE*elems 
        // by copying STRIDE vectors each of length `elems`
        if ((copied_per_iter * STRIDE_FACTOR)/4 < n) {
            // Strided load-store with stride of N
            //    for address offset = 0
            //    and address offset = 1 element
            //    ...
            //    and address offset = N - 1 elements

            for (size_t i = 0; i < STRIDE_FACTOR*4; i++) {
                VEC_INTRIN(vsse8_v_i8m1)(
                    ((char*)out)+i,
                    STRIDE_FACTOR*ELEM_WIDTH,
                    VEC_INTRIN(vlse8_v_i8m1)(
                        ((char*)in)+i,
                        STRIDE_FACTOR*ELEM_WIDTH,
                        copied_per_iter
                    ),
                    copied_per_iter
                );
            }

            in += (copied_per_iter * STRIDE_FACTOR) / 4;
            out += (copied_per_iter * STRIDE_FACTOR) / 4;
            n -= (copied_per_iter * STRIDE_FACTOR) / 4;
        } else {
            // We don't have room to do STRIDE*elems,
            // pick up the rest with normal copies
            VEC_INTRIN(vse8_v_i8m1)(out, VEC_INTRIN(vle8_v_i8m1)(in, copied_per_iter), copied_per_iter);

            in += copied_per_iter / 4;
            out += copied_per_iter / 4;
            n -= copied_per_iter / 4;
        }
    }
}

void vector_memcpy_16strided(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    const size_t STRIDE_FACTOR = 4;
    const size_t ELEM_WIDTH = 2;
    size_t copied_per_iter = 0;
    for (; n > 0 && !(n == 1 && copied_per_iter < 2); ) {
        copied_per_iter = vsetvl_e16m1(n*2);

        // If we have room to do so, copy STRIDE*elems 
        // by copying STRIDE vectors each of length `elems`
        if (copied_per_iter * STRIDE_FACTOR < n) {
            // Strided load-store with stride of N
            //    for address offset = 0
            //    and address offset = 1 element
            //    ...
            //    and address offset = N - 1 elements

            for (size_t i = 0; i < STRIDE_FACTOR*2; i++) {
                VEC_INTRIN(vsse16_v_i16m1)(
                    ((char*)out)+(i*2),
                    STRIDE_FACTOR*ELEM_WIDTH,
                    VEC_INTRIN(vlse16_v_i16m1)(
                        ((char*)in)+(i*2),
                        STRIDE_FACTOR*ELEM_WIDTH,
                        copied_per_iter
                    ),
                    copied_per_iter
                );
            }

            in += (copied_per_iter * STRIDE_FACTOR) / 2;
            out += (copied_per_iter * STRIDE_FACTOR) / 2;
            n -= (copied_per_iter * STRIDE_FACTOR) / 2;
        } else {
            // We don't have room to do STRIDE*elems,
            // pick up the rest with normal copies
            VEC_INTRIN(vse16_v_i16m1)(out, VEC_INTRIN(vle16_v_i16m1)(in, copied_per_iter), copied_per_iter);

            in += copied_per_iter / 2;
            out += copied_per_iter / 2;
            n -= copied_per_iter / 2;
        }
    }
}

void vector_memcpy_32strided(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    const size_t STRIDE_FACTOR = 4;
    const size_t ELEM_WIDTH = 4;
    size_t copied_per_iter = 0;
    for (; n > 0; ) {
        copied_per_iter = vsetvl_e32m1(n);

        // If we have room to do so, copy STRIDE*elems 
        // by copying STRIDE vectors each of length `elems`
        if (copied_per_iter * STRIDE_FACTOR < n) {
            // Strided load-store with stride of N
            //    for address offset = 0
            //    and address offset = 1 element
            //    ...
            //    and address offset = N - 1 elements

            for (size_t i = 0; i < STRIDE_FACTOR; i++) {
                VEC_INTRIN(vsse32_v_i32m1)(
                    out+i,
                    STRIDE_FACTOR*ELEM_WIDTH,
                    VEC_INTRIN(vlse32_v_i32m1)(
                        in+i,
                        STRIDE_FACTOR*ELEM_WIDTH,
                        copied_per_iter
                    ),
                    copied_per_iter
                );
            }

            in += copied_per_iter * STRIDE_FACTOR;
            out += copied_per_iter * STRIDE_FACTOR;
            n -= copied_per_iter * STRIDE_FACTOR;
        } else {
            // We don't have room to do STRIDE*elems,
            // pick up the rest with normal copies
            VEC_INTRIN(vse32_v_i32m1)(out, VEC_INTRIN(vle32_v_i32m1)(in, copied_per_iter), copied_per_iter);

            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
#endif // ENABLE_STRIDED

#if ENABLE_FRAC
// f2 => LMUL = 1/2, which means only 1/2 of the vector register is used per iteration
// I would use 1/4, but it doesn't exist for 32bit in the intrinsics
void vector_memcpy_32mf2(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32mf2(n);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        VEC_INTRIN(vse32_v_i32mf2)(out, VEC_INTRIN(vle32_v_i32mf2)(in, copied_per_iter), copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_FRAC

// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out = pointer to output data (should be aligned?)
void vector_memcpy_8m8(size_t n_32, const int32_t* __restrict__ in_32, int32_t* __restrict__ out_32) {
    size_t n = n_32 * 4;
    const int8_t* in = (const int8_t*)in_32;
    int8_t* out = (int8_t*)out_32;

    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e8m8(n);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        VEC_INTRIN(vse8_v_i8m8)(out, VEC_INTRIN(vle8_v_i8m8)(in, copied_per_iter), copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}

// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out = pointer to output data (should be aligned?)
void vector_memcpy_16m8(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t copied_per_iter = 0;
    // TODO infinite loops
    for (; n > 0 && !(n == 1 && copied_per_iter < 2); n -= (copied_per_iter/2)) {
        copied_per_iter = vsetvl_e16m8(n*2);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        VEC_INTRIN(vse16_v_i16m8)(out, VEC_INTRIN(vle16_v_i16m8)(in, copied_per_iter), copied_per_iter);

        in += (copied_per_iter/2);
        out += (copied_per_iter/2);
    }
}

// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out = pointer to output data (should be aligned?)
void vector_memcpy_32m8(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m8(n);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        VEC_INTRIN(vse32_v_i32m8)(out, VEC_INTRIN(vle32_v_i32m8)(in, copied_per_iter), copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}


#if ENABLE_ASM_WHOLEREG
void vector_memcpy_32m1_wholereg(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t vlmax = vsetvlmax_e32m1();

    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m1(n);

        if (copied_per_iter == vlmax) {
            // wholereg loads do not have intrinsics, use inline assembly instead
            // By creating the `data' variable beforehand, we can have the compiler
            // allocate registers for us.
            vint32m1_t data;
            #if HAS_CAPABILITIES
            asm volatile(
                "vl1r.v %0, (%1)" 
                : "=vr"(data) // output, '=' -> overwrite old value, 'v' -> vector, 'r' -> register
                : "C"(in)     // input, 'r' -> register
            );
            asm volatile(
                "vs1r.v %0, (%1)" 
                :: "vr"(data), "C"(out)
            );
            #else
            asm volatile(
                "vl1r.v %0, (%1)" 
                : "=vr"(data) // output, '=' -> overwrite old value, 'v' -> vector, 'r' -> register
                : "r"(in)     // input, 'r' -> register
            );
            asm volatile(
                "vs1r.v %0, (%1)" 
                :: "vr"(data), "r"(out)
            );
            #endif

        } else {
            VEC_INTRIN(vse32_v_i32m1)(out, VEC_INTRIN(vle32_v_i32m1)(in, copied_per_iter), copied_per_iter);
        }

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_ASM_WHOLEREG


#if ENABLE_FAULTONLYFIRST
// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out = pointer to output data (should be aligned?)
void vector_memcpy_32m8_faultonlyfirst(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m8(n);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state

        // fault-only-first may modify the vector length
        // it should not, given our test cases
        // but if it does, end early (this should trigger an error)
        size_t new_vl = 0;
        VEC_TYPE(vint32m8_t) data = VEC_INTRIN(vle32ff_v_i32m8)(in, &new_vl, copied_per_iter);
        if (new_vl != copied_per_iter) 
            return;
        VEC_INTRIN(vse32_v_i32m8)(out, data, copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}
#endif // ENABLE_FAULTONLYFIRST

// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out_datas = pointer to output datas
#if ENABLE_SEG
void vector_memcpy_32m2_seg4load(size_t n_seg, const int32_t* __restrict__ in, int32_t* __restrict__ out[4]) {
    size_t copied_per_iter = 0;
    for (; n_seg > 0; n_seg -= copied_per_iter) {
        copied_per_iter = vsetvl_e32m2(n_seg);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        // vint32m8_t data = vle32_v_i32m8(in, copied_per_iter);
        // vse32_v_i32m8(out, data, copied_per_iter);

        vint32m2_t r,g,b,a;
        vlseg4e32_v_i32m2(&r, &g, &b, &a, in, copied_per_iter);
        vse32_v_i32m2(out[0], r, copied_per_iter);
        vse32_v_i32m2(out[1], g, copied_per_iter);
        vse32_v_i32m2(out[2], b, copied_per_iter);
        vse32_v_i32m2(out[3], a, copied_per_iter);

        in += copied_per_iter * 4;
        for (int i = 0; i < 4; i++)
            out[i] += copied_per_iter;
    }
}
#endif // ENABLE_SEG

int vector_memcpy_harness(void (*memcpy_fn)(size_t, const int32_t*, int32_t*)) {
    int data[128] = {0};
    int out_data[128] = {0};

    for (int i = 0; i < 128; i++) {
        data[i] = i;
    }

    // ONLY copy 103 values
    memcpy_fn(103, data, out_data);

    // Check the first 103 values of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (int i = 0; i < 103; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (int i = 103; i < 128; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}

#if ENABLE_MASKED
int vector_memcpy_masked_harness(void (*memcpy_fn)(size_t, const int32_t*, int32_t*)) {
    // This is different to the normal harness, to better test mask stuff.
    // It only tests odd-indexed elements for copy, because that's what masked memcpys do
    // (if we tested all elements, and had the masked memcpy do all elements,
    // we wouldn't be able to tell if any masking was actually enabled.)

    // also, it sets out_data to 0xFFFFFFFF
    // this means the masked-out elements actually don't write anything, instead of e.g. writing zero.

    int data[128] = {0};
    int out_data[128] = {0};
    memset(out_data, 0xFF, 128*sizeof(int));

    for (int i = 0; i < 128; i++) {
        data[i] = i;
    }

    // ONLY copy 103 values
    memcpy_fn(103, data, out_data);

    // Check all odd indices < 103 of output have been copied
    // This ensures that the emulator correctly loaded/stored enough values
    for (int i = 0; i < 103; i++) {
        if ((i & 1) == 1 && data[i] != out_data[i]) {
            return 0;
        } else if ((i & 1) == 0 && out_data[i] != 0xFFFFFFFF) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (int i = 103; i < 128; i++) {
        if (out_data[i] != 0xFFFFFFFF) {
            return 0;
        }
    }
    return 1;
}
#endif // ENABLE_MASKED

#if ENABLE_SEG
int vector_memcpy_segmented_harness_i32(void (*memcpy_fn)(size_t, const int32_t* __restrict__, int32_t* __restrict__ [4])) {
    // This is different to the normal harness, to better test segmented accesses.
    // Tests are expected to copy data out into four separate arrays

    int32_t data[128] = {0};
    int32_t out_r[32] = {0};
    int32_t out_g[32] = {0};
    int32_t out_b[32] = {0};
    int32_t out_a[32] = {0};

    for (int i = 0; i < 128; i++) {
        data[i] = i;
    }

    int32_t* out_datas[4] = {out_r, out_g, out_b, out_a};

    // copy 104 elements = 26 segments
    memcpy_fn(26, data, out_datas);

    // Check all odd indices < 103 of output have been copied
    // This ensures that the emulator correctly loaded/stored enough values
    for (int i = 0; i < 26; i++) {
        if (data[i*4 + 0] != out_r[i]) {
            return 0;
        }
        if (data[i*4 + 1] != out_g[i]) {
            return 0;
        }
        if (data[i*4 + 2] != out_b[i]) {
            return 0;
        }
        if (data[i*4 + 3] != out_a[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (int i = 26; i < 32; i++) {
        if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {
            return 0;
        }
    }
    return 1;
}
#endif // ENABLE_SEG

#if ENABLE_FAULTONLYFIRST
int vector_unit_faultonlyfirst_test_under_fault(void) {
    // TODO this breaks on GCC 10.2

    // This test is different from others.
    // It does individual fault-only-first loads on the boundary of
    // a known unmapped memory region (set in the emulator)

    int32_t* UNMAPPED_REGION_START = (int32_t*)0x25000;

    // Find the number of 32-bit elements in a single vector register
    size_t vlmax = vsetvlmax_e32m1();

    for (size_t i = 0; i < vlmax; i++) {
        *(UNMAPPED_REGION_START - vlmax + i) = i;
    }

    // for each N in [1, vlmax]
    //     run a test case that reads N elements before hitting the UNMAPPED_REGION
    //     assert the resulting vlen == N
    for (size_t expected_num_copied = 1; expected_num_copied <= vlmax; expected_num_copied++) {
        const int32_t* in = UNMAPPED_REGION_START - expected_num_copied;

        // reset the length
        size_t fof_length = vsetvlmax_e32m1();
        // do a load, see how the length changes
        VEC_INTRIN(vle32ff_v_i32m1)(in, &fof_length, vlmax);
        if (fof_length != expected_num_copied)
            return 0;
    }
    return 1;
}
#endif // ENABLE_FAULTONLYFIRST

// Magical output devices, set by linker
volatile extern int64_t outputAttempted;
volatile extern int64_t outputSucceeded;
volatile extern int8_t finished;

#ifdef __cplusplus
extern "C" {
#endif
int main(void)
{
  int64_t result = 0;
  int64_t attempted = 0;

  attempted |= 1 << 0; result |= vector_memcpy_harness(vector_memcpy_8m8) << 0;
  attempted |= 1 << 1; result |= vector_memcpy_harness(vector_memcpy_16m8) << 1;
  attempted |= 1 << 2; result |= vector_memcpy_harness(vector_memcpy_32m8) << 2;
  #if ENABLE_FRAC
  attempted |= 1 << 3; result |= vector_memcpy_harness(vector_memcpy_32mf2) << 3;
  #endif // ENABLE_FRAC

  #if ENABLE_STRIDED
  attempted |= 1 << 4; result |= vector_memcpy_harness(vector_memcpy_8strided) << 4;
  attempted |= 1 << 5; result |= vector_memcpy_harness(vector_memcpy_16strided) << 5;
  attempted |= 1 << 6; result |= vector_memcpy_harness(vector_memcpy_32strided) << 6;
  #endif // ENABLE_STRIDED

  #if ENABLE_INDEXED
  attempted |= 1 << 7; result |= vector_memcpy_harness(vector_memcpy_indexed) << 7;
  #endif // ENABLE_INDEXED

  #if ENABLE_MASKED
  attempted |= 1 << 8; result |= vector_memcpy_masked_harness(vector_memcpy_masked) << 8;
  #endif // ENABLE_MASKED

  #if ENABLE_SEG
  attempted |= 1 << 9; result |= vector_memcpy_segmented_harness_i32(vector_memcpy_32m2_seg4load) << 9;
  #endif // ENABLE_SEG
  #if ENABLE_BYTEMASKLOAD
  attempted |= 1 << 10; result |= vector_memcpy_masked_harness(vector_memcpy_masked_bytemaskload) << 10;
  #endif // ENABLE_BYTEMASKLOAD

  #if ENABLE_FAULTONLYFIRST
  attempted |= 1 << 11; result |= vector_memcpy_harness(vector_memcpy_32m8_faultonlyfirst) << 11;
  attempted |= 1 << 12; result |= vector_unit_faultonlyfirst_test_under_fault() << 12;
  #endif // ENABLE_FAULTONLYFIRST

  #if ENABLE_ASM_WHOLEREG
  attempted |= 1 << 13; result |= vector_memcpy_harness(vector_memcpy_32m1_wholereg) << 13;
  #endif // ENABLE_ASM_WHOLEREG

  *(&outputAttempted) = attempted;
  *(&outputSucceeded) = result;
  finished = 1;
  return result;
}
#ifdef __cplusplus
}
#endif
