
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
#ifdef __cplusplus
}
#endif
int vector_memcpy_harness_uint8_t(void (*memcpy_fn)(size_t, const uint8_t* __restrict__, uint8_t* __restrict__)) {
    uint8_t data[128] = {0};
    uint8_t out_data[128] = {0};
    
    for (uint8_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    // ONLY copy 110 elements
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint8_t i = 0; i < 110; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (uint8_t i = 110; i < 128; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_harness_uint16_t(void (*memcpy_fn)(size_t, const uint16_t* __restrict__, uint16_t* __restrict__)) {
    uint16_t data[128] = {0};
    uint16_t out_data[128] = {0};
    
    for (uint16_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    // ONLY copy 110 elements
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint16_t i = 0; i < 110; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (uint16_t i = 110; i < 128; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_harness_uint32_t(void (*memcpy_fn)(size_t, const uint32_t* __restrict__, uint32_t* __restrict__)) {
    uint32_t data[128] = {0};
    uint32_t out_data[128] = {0};
    
    for (uint32_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    // ONLY copy 110 elements
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint32_t i = 0; i < 110; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (uint32_t i = 110; i < 128; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_harness_uint64_t(void (*memcpy_fn)(size_t, const uint64_t* __restrict__, uint64_t* __restrict__)) {
    uint64_t data[128] = {0};
    uint64_t out_data[128] = {0};
    
    for (uint64_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    // ONLY copy 110 elements
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint64_t i = 0; i < 110; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (uint64_t i = 110; i < 128; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}
void vector_memcpy_unit_stride_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            #if __has_feature(capabilities);
            asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else;
            data = vle8_v_u8m8(in, copied_per_iter);
            vse8_v_u8m8(out, data, copied_per_iter);
            #endif;
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e16m8(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            #if __has_feature(capabilities);
            asm volatile ("vle16.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse16.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else;
            data = vle16_v_u16m8(in, copied_per_iter);
            vse16_v_u16m8(out, data, copied_per_iter);
            #endif;
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e32m8(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            #if __has_feature(capabilities);
            asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else;
            data = vle32_v_u32m8(in, copied_per_iter);
            vse32_v_u32m8(out, data, copied_per_iter);
            #endif;
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            #if __has_feature(capabilities);
            asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else;
            data = vle32_v_u32mf2(in, copied_per_iter);
            vse32_v_u32mf2(out, data, copied_per_iter);
            #endif;
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_strided_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    const size_t STRIDE_ELEMS = 4;
    const size_t STRIDE_BYTES = 4 * sizeof(uint8_t);
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint8_t* in_offset = in + i;
                    uint8_t* out_offset = out + i;
                    #if __has_feature(capabilities);
                    asm volatile ("vlse8.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse8.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else;
                    data = vlse8_v_u8m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse8_v_u8m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif;
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities);
                asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else;
                data = vle8_v_u8m8(in, copied_per_iter);
                vse8_v_u8m8(out, data, copied_per_iter);
                #endif;
                in += copied_per_iter;
                out += copied_per_iter;
                n -= copied_per_iter;
            }
        }
    }
}
void vector_memcpy_strided_e16m8(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out) {
    const size_t STRIDE_ELEMS = 4;
    const size_t STRIDE_BYTES = 4 * sizeof(uint16_t);
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint16_t* in_offset = in + i;
                    uint16_t* out_offset = out + i;
                    #if __has_feature(capabilities);
                    asm volatile ("vlse16.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse16.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else;
                    data = vlse16_v_u16m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse16_v_u16m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif;
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities);
                asm volatile ("vle16.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse16.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else;
                data = vle16_v_u16m8(in, copied_per_iter);
                vse16_v_u16m8(out, data, copied_per_iter);
                #endif;
                in += copied_per_iter;
                out += copied_per_iter;
                n -= copied_per_iter;
            }
        }
    }
}
void vector_memcpy_strided_e32m8(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    const size_t STRIDE_ELEMS = 4;
    const size_t STRIDE_BYTES = 4 * sizeof(uint32_t);
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint32_t* in_offset = in + i;
                    uint32_t* out_offset = out + i;
                    #if __has_feature(capabilities);
                    asm volatile ("vlse32.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse32.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else;
                    data = vlse32_v_u32m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse32_v_u32m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif;
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities);
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else;
                data = vle32_v_u32m8(in, copied_per_iter);
                vse32_v_u32m8(out, data, copied_per_iter);
                #endif;
                in += copied_per_iter;
                out += copied_per_iter;
                n -= copied_per_iter;
            }
        }
    }
}
void vector_memcpy_strided_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    const size_t STRIDE_ELEMS = 4;
    const size_t STRIDE_BYTES = 4 * sizeof(uint32_t);
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint32_t* in_offset = in + i;
                    uint32_t* out_offset = out + i;
                    #if __has_feature(capabilities);
                    asm volatile ("vlse32.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse32.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else;
                    data = vlse32_v_u32mf2(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse32_v_u32mf2(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif;
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities);
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else;
                data = vle32_v_u32mf2(in, copied_per_iter);
                vse32_v_u32mf2(out, data, copied_per_iter);
                #endif;
                in += copied_per_iter;
                out += copied_per_iter;
                n -= copied_per_iter;
            }
        }
    }
}
#ifdef __cplusplus;
extern "C" {;
#endif // __cplusplus;
int main(void) {
    int *outputDevice = (int*) 0xf0000000;
    // magic output device;
    int result = 0;
    result |= vector_memcpy_harness_uint8_t(vector_memcpy_unit_stride_e8m8) << 0;
    result |= vector_memcpy_harness_uint16_t(vector_memcpy_unit_stride_e16m8) << 1;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_unit_stride_e32m8) << 2;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_unit_stride_e32mf2) << 3;
    result |= vector_memcpy_harness_uint8_t(vector_memcpy_strided_e8m8) << 4;
    result |= vector_memcpy_harness_uint16_t(vector_memcpy_strided_e16m8) << 5;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_strided_e32m8) << 6;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_strided_e32mf2) << 7;
    outputDevice[0] = result;
    return result;
}
#ifdef __cplusplus;
};
#endif // __cplusplus;
