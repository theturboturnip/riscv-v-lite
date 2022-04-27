
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
int vector_memcpy_masked_harness_uint8_t(void (*memcpy_fn)(size_t, const uint8_t* __restrict__, uint8_t* __restrict__)) {
    uint8_t data[128] = {0};
    uint8_t out_data[128] = {0};
    const uint8_t SENTINEL_NOT_WRITTEN = 0xbb;
    
    for (uint8_t i = 0; i < 128; i++) {
        data[i] = i;
        out_data[i] = SENTINEL_NOT_WRITTEN;
    }
    
    // ONLY copy 110 elements
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint8_t i = 0; i < 110; i++) {
        if ((i & 1) == 1 && data[i] != out_data[i]) {
            return 0;
        } else if ((i & 1) == 0 && out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    // Check that the rest are all the original value
    // This ensures that the emulator didn't store more elements than it should have
    for (uint8_t i = 110; i < 128; i++) {
        if (out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_masked_harness_uint16_t(void (*memcpy_fn)(size_t, const uint16_t* __restrict__, uint16_t* __restrict__)) {
    uint16_t data[128] = {0};
    uint16_t out_data[128] = {0};
    const uint16_t SENTINEL_NOT_WRITTEN = 0xbb;
    
    for (uint16_t i = 0; i < 128; i++) {
        data[i] = i;
        out_data[i] = SENTINEL_NOT_WRITTEN;
    }
    
    // ONLY copy 110 elements
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint16_t i = 0; i < 110; i++) {
        if ((i & 1) == 1 && data[i] != out_data[i]) {
            return 0;
        } else if ((i & 1) == 0 && out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    // Check that the rest are all the original value
    // This ensures that the emulator didn't store more elements than it should have
    for (uint16_t i = 110; i < 128; i++) {
        if (out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_masked_harness_uint32_t(void (*memcpy_fn)(size_t, const uint32_t* __restrict__, uint32_t* __restrict__)) {
    uint32_t data[128] = {0};
    uint32_t out_data[128] = {0};
    const uint32_t SENTINEL_NOT_WRITTEN = 0xbb;
    
    for (uint32_t i = 0; i < 128; i++) {
        data[i] = i;
        out_data[i] = SENTINEL_NOT_WRITTEN;
    }
    
    // ONLY copy 110 elements
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint32_t i = 0; i < 110; i++) {
        if ((i & 1) == 1 && data[i] != out_data[i]) {
            return 0;
        } else if ((i & 1) == 0 && out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    // Check that the rest are all the original value
    // This ensures that the emulator didn't store more elements than it should have
    for (uint32_t i = 110; i < 128; i++) {
        if (out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_masked_harness_uint64_t(void (*memcpy_fn)(size_t, const uint64_t* __restrict__, uint64_t* __restrict__)) {
    uint64_t data[128] = {0};
    uint64_t out_data[128] = {0};
    const uint64_t SENTINEL_NOT_WRITTEN = 0xbb;
    
    for (uint64_t i = 0; i < 128; i++) {
        data[i] = i;
        out_data[i] = SENTINEL_NOT_WRITTEN;
    }
    
    // ONLY copy 110 elements
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(110, data, out_data);
    
    // Check the first 110 elements of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint64_t i = 0; i < 110; i++) {
        if ((i & 1) == 1 && data[i] != out_data[i]) {
            return 0;
        } else if ((i & 1) == 0 && out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    // Check that the rest are all the original value
    // This ensures that the emulator didn't store more elements than it should have
    for (uint64_t i = 110; i < 128; i++) {
        if (out_data[i] != SENTINEL_NOT_WRITTEN) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_segmented_harness_uint8_t(void (*memcpy_fn)(size_t, const uint8_t* __restrict__, uint8_t* __restrict__[4])) {
    uint8_t data[128] = {0};
    uint8_t out_r[32] = {0};
    uint8_t out_g[32] = {0};
    uint8_t out_b[32] = {0};
    uint8_t out_a[32] = {0};
    
    for (uint8_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    uint8_t* out_datas[4] = {out_r, out_g, out_b, out_a};
    
    
    // ONLY copy 104 elements = 26 segments
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(26, data, out_datas);
    
    // Check the first 104 elements = 26 segments of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint8_t i = 0; i < 26; i++) {
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
    for (uint8_t i = 26; i < 32; i++) {
        if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_segmented_harness_uint16_t(void (*memcpy_fn)(size_t, const uint16_t* __restrict__, uint16_t* __restrict__[4])) {
    uint16_t data[128] = {0};
    uint16_t out_r[32] = {0};
    uint16_t out_g[32] = {0};
    uint16_t out_b[32] = {0};
    uint16_t out_a[32] = {0};
    
    for (uint16_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    uint16_t* out_datas[4] = {out_r, out_g, out_b, out_a};
    
    
    // ONLY copy 104 elements = 26 segments
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(26, data, out_datas);
    
    // Check the first 104 elements = 26 segments of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint16_t i = 0; i < 26; i++) {
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
    for (uint16_t i = 26; i < 32; i++) {
        if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_segmented_harness_uint32_t(void (*memcpy_fn)(size_t, const uint32_t* __restrict__, uint32_t* __restrict__[4])) {
    uint32_t data[128] = {0};
    uint32_t out_r[32] = {0};
    uint32_t out_g[32] = {0};
    uint32_t out_b[32] = {0};
    uint32_t out_a[32] = {0};
    
    for (uint32_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    uint32_t* out_datas[4] = {out_r, out_g, out_b, out_a};
    
    
    // ONLY copy 104 elements = 26 segments
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(26, data, out_datas);
    
    // Check the first 104 elements = 26 segments of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint32_t i = 0; i < 26; i++) {
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
    for (uint32_t i = 26; i < 32; i++) {
        if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {
            return 0;
        }
    }
    return 1;
}
int vector_memcpy_segmented_harness_uint64_t(void (*memcpy_fn)(size_t, const uint64_t* __restrict__, uint64_t* __restrict__[4])) {
    uint64_t data[128] = {0};
    uint64_t out_r[32] = {0};
    uint64_t out_g[32] = {0};
    uint64_t out_b[32] = {0};
    uint64_t out_a[32] = {0};
    
    for (uint64_t i = 0; i < 128; i++) {
        data[i] = i;
    }
    
    uint64_t* out_datas[4] = {out_r, out_g, out_b, out_a};
    
    
    // ONLY copy 104 elements = 26 segments
    // For the masked function, this should only copy odd-indexed elements.
    memcpy_fn(26, data, out_datas);
    
    // Check the first 104 elements = 26 segments of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (uint64_t i = 0; i < 26; i++) {
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
    for (uint64_t i = 26; i < 32; i++) {
        if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {
            return 0;
        }
    }
    return 1;
}
void vector_memcpy_unit_stride_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else
            data = vle8_v_u8m8(in, copied_per_iter);
            vse8_v_u8m8(out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e16m8(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle16.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse16.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else
            data = vle16_v_u16m8(in, copied_per_iter);
            vse16_v_u16m8(out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e32m8(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else
            data = vle32_v_u32m8(in, copied_per_iter);
            vse32_v_u32m8(out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_unit_stride_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
            #else
            data = vle32_v_u32mf2(in, copied_per_iter);
            vse32_v_u32mf2(out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_strided_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    const size_t STRIDE_ELEMS = 4;
    const size_t STRIDE_BYTES = 4 * sizeof(uint8_t);
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint8_t* in_offset = in + i;
                    uint8_t* out_offset = out + i;
                    #if __has_feature(capabilities)
                    asm volatile ("vlse8.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse8.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else
                    data = vlse8_v_u8m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse8_v_u8m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle8_v_u8m8(in, copied_per_iter);
                vse8_v_u8m8(out, data, copied_per_iter);
                #endif
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
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint16_t* in_offset = in + i;
                    uint16_t* out_offset = out + i;
                    #if __has_feature(capabilities)
                    asm volatile ("vlse16.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse16.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else
                    data = vlse16_v_u16m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse16_v_u16m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle16.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse16.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle16_v_u16m8(in, copied_per_iter);
                vse16_v_u16m8(out, data, copied_per_iter);
                #endif
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
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint32_t* in_offset = in + i;
                    uint32_t* out_offset = out + i;
                    #if __has_feature(capabilities)
                    asm volatile ("vlse32.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse32.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else
                    data = vlse32_v_u32m8(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse32_v_u32m8(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle32_v_u32m8(in, copied_per_iter);
                vse32_v_u32m8(out, data, copied_per_iter);
                #endif
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
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            if (copied_per_iter * STRIDE_ELEMS < n) {
                for (size_t i = 0; i < STRIDE_ELEMS; i++) {
                    const uint32_t* in_offset = in + i;
                    uint32_t* out_offset = out + i;
                    #if __has_feature(capabilities)
                    asm volatile ("vlse32.v %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));
                    asm volatile ("vsse32.v %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));
                    #else
                    data = vlse32_v_u32mf2(in_offset, STRIDE_BYTES, copied_per_iter);
                    vsse32_v_u32mf2(out_offset, STRIDE_BYTES, data, copied_per_iter);
                    #endif
                }
                in += copied_per_iter * STRIDE_ELEMS;
                out += copied_per_iter * STRIDE_ELEMS;
                n -= copied_per_iter * STRIDE_ELEMS;
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle32_v_u32mf2(in, copied_per_iter);
                vse32_v_u32mf2(out, data, copied_per_iter);
                #endif
                in += copied_per_iter;
                out += copied_per_iter;
                n -= copied_per_iter;
            }
        }
    }
}
void vector_memcpy_indexed_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    const size_t ELEM_WIDTH = sizeof(uint8_t);
    const size_t VLMAX = vsetvlmax_e8m8();
    uint8_t indices[128] = {0};
    for (size_t i = 0; i < VLMAX; i++) {
        indices[i] = (((uint8_t) i) ^ 1) * ELEM_WIDTH;
    }
    vuint8m8_t indices_v;
    #if __has_feature(capabilities)
    asm volatile ("vle8.v %0, (%1)" : "=vr"(indices_v) : "C"(indices));
    #else
    indices_v = vle8_v_u8m8(indices, VLMAX);
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            if (copied_per_iter == VLMAX) {
                #if __has_feature(capabilities)
                asm volatile ("vluxei8.v %0, (%1), %2" : "=vr"(data) : "C"(in), "vr"(indices_v));
                asm volatile ("vsuxei8.v %0, (%1), %2" :: "vr"(data),  "C"(out), "vr"(indices_v));
                #else
                data = vluxei8_v_u8m8(in, indices_v, copied_per_iter);
                vsuxei8_v_u8m8(out, indices_v, data, copied_per_iter);
                #endif
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle8.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse8.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle8_v_u8m8(in, copied_per_iter);
                vse8_v_u8m8(out, data, copied_per_iter);
                #endif
            }
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_indexed_e16m8(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out) {
    const size_t ELEM_WIDTH = sizeof(uint16_t);
    const size_t VLMAX = vsetvlmax_e16m8();
    uint16_t indices[128] = {0};
    for (size_t i = 0; i < VLMAX; i++) {
        indices[i] = (((uint16_t) i) ^ 1) * ELEM_WIDTH;
    }
    vuint16m8_t indices_v;
    #if __has_feature(capabilities)
    asm volatile ("vle16.v %0, (%1)" : "=vr"(indices_v) : "C"(indices));
    #else
    indices_v = vle16_v_u16m8(indices, VLMAX);
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            if (copied_per_iter == VLMAX) {
                #if __has_feature(capabilities)
                asm volatile ("vluxei16.v %0, (%1), %2" : "=vr"(data) : "C"(in), "vr"(indices_v));
                asm volatile ("vsuxei16.v %0, (%1), %2" :: "vr"(data),  "C"(out), "vr"(indices_v));
                #else
                data = vluxei16_v_u16m8(in, indices_v, copied_per_iter);
                vsuxei16_v_u16m8(out, indices_v, data, copied_per_iter);
                #endif
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle16.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse16.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle16_v_u16m8(in, copied_per_iter);
                vse16_v_u16m8(out, data, copied_per_iter);
                #endif
            }
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_indexed_e32m8(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    const size_t ELEM_WIDTH = sizeof(uint32_t);
    const size_t VLMAX = vsetvlmax_e32m8();
    uint32_t indices[128] = {0};
    for (size_t i = 0; i < VLMAX; i++) {
        indices[i] = (((uint32_t) i) ^ 1) * ELEM_WIDTH;
    }
    vuint32m8_t indices_v;
    #if __has_feature(capabilities)
    asm volatile ("vle32.v %0, (%1)" : "=vr"(indices_v) : "C"(indices));
    #else
    indices_v = vle32_v_u32m8(indices, VLMAX);
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            if (copied_per_iter == VLMAX) {
                #if __has_feature(capabilities)
                asm volatile ("vluxei32.v %0, (%1), %2" : "=vr"(data) : "C"(in), "vr"(indices_v));
                asm volatile ("vsuxei32.v %0, (%1), %2" :: "vr"(data),  "C"(out), "vr"(indices_v));
                #else
                data = vluxei32_v_u32m8(in, indices_v, copied_per_iter);
                vsuxei32_v_u32m8(out, indices_v, data, copied_per_iter);
                #endif
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle32_v_u32m8(in, copied_per_iter);
                vse32_v_u32m8(out, data, copied_per_iter);
                #endif
            }
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_indexed_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    const size_t ELEM_WIDTH = sizeof(uint32_t);
    const size_t VLMAX = vsetvlmax_e32mf2();
    uint32_t indices[128] = {0};
    for (size_t i = 0; i < VLMAX; i++) {
        indices[i] = (((uint32_t) i) ^ 1) * ELEM_WIDTH;
    }
    vuint32mf2_t indices_v;
    #if __has_feature(capabilities)
    asm volatile ("vle32.v %0, (%1)" : "=vr"(indices_v) : "C"(indices));
    #else
    indices_v = vle32_v_u32mf2(indices, VLMAX);
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            if (copied_per_iter == VLMAX) {
                #if __has_feature(capabilities)
                asm volatile ("vluxei32.v %0, (%1), %2" : "=vr"(data) : "C"(in), "vr"(indices_v));
                asm volatile ("vsuxei32.v %0, (%1), %2" :: "vr"(data),  "C"(out), "vr"(indices_v));
                #else
                data = vluxei32_v_u32mf2(in, indices_v, copied_per_iter);
                vsuxei32_v_u32mf2(out, indices_v, data, copied_per_iter);
                #endif
            }
            else {
                #if __has_feature(capabilities)
                asm volatile ("vle32.v %0, (%1)" : "=vr"(data) : "C"(in));
                asm volatile ("vse32.v %0, (%1)" :: "vr"(data),  "C"(out));
                #else
                data = vle32_v_u32mf2(in, copied_per_iter);
                vse32_v_u32mf2(out, data, copied_per_iter);
                #endif
            }
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_masked_e8m8(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out) {
    uint8_t mask_ints[128] = {0};
    const size_t VLMAX = vsetvlmax_e8m8();
    for (size_t i = 0; i < VLMAX; i++) {
        mask_ints[i] = i & 1;
    }
    vuint8m8_t mask_ints_v;
    #if __has_feature(capabilities)
    asm volatile ("vle8.v %0, (%1)" : "=vr"(mask_ints_v) : "C"(in));
    #else
    mask_ints_v = vle8_v_u8m8(in, VLMAX);
    #endif
    vbool1_t mask = vmseq_vx_u8m8_b1(mask_ints_v, 1, VLMAX);
    #if __has_feature(capabilities)
    asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n);
            if (copied_per_iter == 0) break;
            vuint8m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle8.v %0, (%1), v0.t" : "=vr"(data) : "C"(in));
            asm volatile ("vse8.v %0, (%1), v0.t" :: "vr"(data),  "C"(out));
            #else
            data = vle8_v_u8m8_m(mask, data, in, copied_per_iter);
            vse8_v_u8m8_m(mask, out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_masked_e16m8(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out) {
    uint16_t mask_ints[128] = {0};
    const size_t VLMAX = vsetvlmax_e16m8();
    for (size_t i = 0; i < VLMAX; i++) {
        mask_ints[i] = i & 1;
    }
    vuint16m8_t mask_ints_v;
    #if __has_feature(capabilities)
    asm volatile ("vle16.v %0, (%1)" : "=vr"(mask_ints_v) : "C"(in));
    #else
    mask_ints_v = vle16_v_u16m8(in, VLMAX);
    #endif
    vbool2_t mask = vmseq_vx_u16m8_b2(mask_ints_v, 1, VLMAX);
    #if __has_feature(capabilities)
    asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n);
            if (copied_per_iter == 0) break;
            vuint16m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle16.v %0, (%1), v0.t" : "=vr"(data) : "C"(in));
            asm volatile ("vse16.v %0, (%1), v0.t" :: "vr"(data),  "C"(out));
            #else
            data = vle16_v_u16m8_m(mask, data, in, copied_per_iter);
            vse16_v_u16m8_m(mask, out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_masked_e32m8(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    uint32_t mask_ints[128] = {0};
    const size_t VLMAX = vsetvlmax_e32m8();
    for (size_t i = 0; i < VLMAX; i++) {
        mask_ints[i] = i & 1;
    }
    vuint32m8_t mask_ints_v;
    #if __has_feature(capabilities)
    asm volatile ("vle32.v %0, (%1)" : "=vr"(mask_ints_v) : "C"(in));
    #else
    mask_ints_v = vle32_v_u32m8(in, VLMAX);
    #endif
    vbool4_t mask = vmseq_vx_u32m8_b4(mask_ints_v, 1, VLMAX);
    #if __has_feature(capabilities)
    asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n);
            if (copied_per_iter == 0) break;
            vuint32m8_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle32.v %0, (%1), v0.t" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1), v0.t" :: "vr"(data),  "C"(out));
            #else
            data = vle32_v_u32m8_m(mask, data, in, copied_per_iter);
            vse32_v_u32m8_m(mask, out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_masked_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out) {
    uint32_t mask_ints[128] = {0};
    const size_t VLMAX = vsetvlmax_e32mf2();
    for (size_t i = 0; i < VLMAX; i++) {
        mask_ints[i] = i & 1;
    }
    vuint32mf2_t mask_ints_v;
    #if __has_feature(capabilities)
    asm volatile ("vle32.v %0, (%1)" : "=vr"(mask_ints_v) : "C"(in));
    #else
    mask_ints_v = vle32_v_u32mf2(in, VLMAX);
    #endif
    vbool64_t mask = vmseq_vx_u32mf2_b64(mask_ints_v, 1, VLMAX);
    #if __has_feature(capabilities)
    asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));
    #endif
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data;
            #if __has_feature(capabilities)
            asm volatile ("vle32.v %0, (%1), v0.t" : "=vr"(data) : "C"(in));
            asm volatile ("vse32.v %0, (%1), v0.t" :: "vr"(data),  "C"(out));
            #else
            data = vle32_v_u32mf2_m(mask, data, in, copied_per_iter);
            vse32_v_u32mf2_m(mask, out, data, copied_per_iter);
            #endif
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_segmented_e8m2(size_t n, const uint8_t* __restrict__ in, uint8_t* __restrict__ out[4]) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e8m2(n);
            if (copied_per_iter == 0) break;
            #if __has_feature(capabilities)
            asm volatile ("vlseg4e8.v v4, (%0)" :: "C"(in));
            asm volatile ("vse8.v v4, (%0)" :: "C"(out[0]));
            asm volatile ("vse8.v v5, (%0)" :: "C"(out[1]));
            asm volatile ("vse8.v v6, (%0)" :: "C"(out[2]));
            asm volatile ("vse8.v v7, (%0)" :: "C"(out[3]));
            #else
            vuint8m2_t r, g, b, a;
            vlseg4e8_v_u8m2(&r, &g, &b, &a, in, copied_per_iter);
            vse8_v_u8m2(out[0], r, copied_per_iter);
            vse8_v_u8m2(out[1], g, copied_per_iter);
            vse8_v_u8m2(out[2], b, copied_per_iter);
            vse8_v_u8m2(out[3], a, copied_per_iter);
            #endif
            in += copied_per_iter * 4;
            for (int i = 0; i < 4; i++) {
                out[i] += copied_per_iter;
            }
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_segmented_e16m2(size_t n, const uint16_t* __restrict__ in, uint16_t* __restrict__ out[4]) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e16m2(n);
            if (copied_per_iter == 0) break;
            #if __has_feature(capabilities)
            asm volatile ("vlseg4e16.v v4, (%0)" :: "C"(in));
            asm volatile ("vse16.v v4, (%0)" :: "C"(out[0]));
            asm volatile ("vse16.v v5, (%0)" :: "C"(out[1]));
            asm volatile ("vse16.v v6, (%0)" :: "C"(out[2]));
            asm volatile ("vse16.v v7, (%0)" :: "C"(out[3]));
            #else
            vuint16m2_t r, g, b, a;
            vlseg4e16_v_u16m2(&r, &g, &b, &a, in, copied_per_iter);
            vse16_v_u16m2(out[0], r, copied_per_iter);
            vse16_v_u16m2(out[1], g, copied_per_iter);
            vse16_v_u16m2(out[2], b, copied_per_iter);
            vse16_v_u16m2(out[3], a, copied_per_iter);
            #endif
            in += copied_per_iter * 4;
            for (int i = 0; i < 4; i++) {
                out[i] += copied_per_iter;
            }
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_segmented_e32m2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out[4]) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32m2(n);
            if (copied_per_iter == 0) break;
            #if __has_feature(capabilities)
            asm volatile ("vlseg4e32.v v4, (%0)" :: "C"(in));
            asm volatile ("vse32.v v4, (%0)" :: "C"(out[0]));
            asm volatile ("vse32.v v5, (%0)" :: "C"(out[1]));
            asm volatile ("vse32.v v6, (%0)" :: "C"(out[2]));
            asm volatile ("vse32.v v7, (%0)" :: "C"(out[3]));
            #else
            vuint32m2_t r, g, b, a;
            vlseg4e32_v_u32m2(&r, &g, &b, &a, in, copied_per_iter);
            vse32_v_u32m2(out[0], r, copied_per_iter);
            vse32_v_u32m2(out[1], g, copied_per_iter);
            vse32_v_u32m2(out[2], b, copied_per_iter);
            vse32_v_u32m2(out[3], a, copied_per_iter);
            #endif
            in += copied_per_iter * 4;
            for (int i = 0; i < 4; i++) {
                out[i] += copied_per_iter;
            }
            n -= copied_per_iter;
        }
    }
}
void vector_memcpy_segmented_e32mf2(size_t n, const uint32_t* __restrict__ in, uint32_t* __restrict__ out[4]) {
    while (1) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n);
            if (copied_per_iter == 0) break;
            #if __has_feature(capabilities)
            asm volatile ("vlseg4e32.v v4, (%0)" :: "C"(in));
            asm volatile ("vse32.v v4, (%0)" :: "C"(out[0]));
            asm volatile ("vse32.v v5, (%0)" :: "C"(out[1]));
            asm volatile ("vse32.v v6, (%0)" :: "C"(out[2]));
            asm volatile ("vse32.v v7, (%0)" :: "C"(out[3]));
            #else
            vuint32mf2_t r, g, b, a;
            vlseg4e32_v_u32mf2(&r, &g, &b, &a, in, copied_per_iter);
            vse32_v_u32mf2(out[0], r, copied_per_iter);
            vse32_v_u32mf2(out[1], g, copied_per_iter);
            vse32_v_u32mf2(out[2], b, copied_per_iter);
            vse32_v_u32mf2(out[3], a, copied_per_iter);
            #endif
            in += copied_per_iter * 4;
            for (int i = 0; i < 4; i++) {
                out[i] += copied_per_iter;
            }
            n -= copied_per_iter;
        }
    }
}
#ifdef __cplusplus
extern "C" {;
#endif // __cplusplus
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
    result |= vector_memcpy_harness_uint8_t(vector_memcpy_indexed_e8m8) << 8;
    result |= vector_memcpy_harness_uint16_t(vector_memcpy_indexed_e16m8) << 9;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_indexed_e32m8) << 10;
    result |= vector_memcpy_harness_uint32_t(vector_memcpy_indexed_e32mf2) << 11;
    result |= vector_memcpy_masked_harness_uint8_t(vector_memcpy_masked_e8m8) << 12;
    result |= vector_memcpy_masked_harness_uint16_t(vector_memcpy_masked_e16m8) << 13;
    result |= vector_memcpy_masked_harness_uint32_t(vector_memcpy_masked_e32m8) << 14;
    result |= vector_memcpy_masked_harness_uint32_t(vector_memcpy_masked_e32mf2) << 15;
    result |= vector_memcpy_segmented_harness_uint8_t(vector_memcpy_segmented_e8m2) << 16;
    result |= vector_memcpy_segmented_harness_uint16_t(vector_memcpy_segmented_e16m2) << 17;
    result |= vector_memcpy_segmented_harness_uint32_t(vector_memcpy_segmented_e32m2) << 18;
    result |= vector_memcpy_segmented_harness_uint32_t(vector_memcpy_segmented_e32mf2) << 19;
    outputDevice[0] = result;
    return result;
}
#ifdef __cplusplus
};
#endif // __cplusplus
