
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
int vector_memcpy_harness(void (*memcpy_fn)(size_t, const int8_t* __restrict__, int8_t* __restrict__)) {
    uint8_t data[128] = {0};
    uint8_t out_data[128] = {0};
    for (uint32_t i = 0;
    i < 128;
    i++) {
            data[i] = i;
    }
        
        // ONLY copy 103 values
        memcpy_fn(103, data, out_data);
    // Check the first 103 values of output are the same
        // This ensures that the emulator correctly loaded/stored enough values
        for (uint32_t i = 0;
    i < 103;
    i++) {
            if (data[i] != out_data[i]) {
                return 0;
    }
        }
        // Check that the rest are 0 (the original value)
        // This ensures that the emulator didn't store more elements than it should have
        for (uint32_t i = 103;
    i < 128;
    i++) {
            if (out_data[i] != 0) {
                return 0;
    }
        }
        return 1;
}
void vector_memcpy_unit_stride_e8mf8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8mf8(n/1);
            if (copied_per_iter == 0) break;
            vuint8mf8_t data = vle8_v_u8mf8(in, copied_per_iter);
            vse8_v_u8mf8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16mf8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16mf8(n/2);
            if (copied_per_iter == 0) break;
            vuint16mf8_t data = vle16_v_u16mf8(in, copied_per_iter);
            vse16_v_u16mf8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32mf8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32mf8(n/4);
            if (copied_per_iter == 0) break;
            vuint32mf8_t data = vle32_v_u32mf8(in, copied_per_iter);
            vse32_v_u32mf8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8mf4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8mf4(n/1);
            if (copied_per_iter == 0) break;
            vuint8mf4_t data = vle8_v_u8mf4(in, copied_per_iter);
            vse8_v_u8mf4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16mf4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16mf4(n/2);
            if (copied_per_iter == 0) break;
            vuint16mf4_t data = vle16_v_u16mf4(in, copied_per_iter);
            vse16_v_u16mf4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32mf4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32mf4(n/4);
            if (copied_per_iter == 0) break;
            vuint32mf4_t data = vle32_v_u32mf4(in, copied_per_iter);
            vse32_v_u32mf4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8mf2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8mf2(n/1);
            if (copied_per_iter == 0) break;
            vuint8mf2_t data = vle8_v_u8mf2(in, copied_per_iter);
            vse8_v_u8mf2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16mf2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16mf2(n/2);
            if (copied_per_iter == 0) break;
            vuint16mf2_t data = vle16_v_u16mf2(in, copied_per_iter);
            vse16_v_u16mf2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32mf2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32mf2(n/4);
            if (copied_per_iter == 0) break;
            vuint32mf2_t data = vle32_v_u32mf2(in, copied_per_iter);
            vse32_v_u32mf2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8m1(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m1(n/1);
            if (copied_per_iter == 0) break;
            vuint8m1_t data = vle8_v_u8m1(in, copied_per_iter);
            vse8_v_u8m1(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16m1(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m1(n/2);
            if (copied_per_iter == 0) break;
            vuint16m1_t data = vle16_v_u16m1(in, copied_per_iter);
            vse16_v_u16m1(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32m1(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m1(n/4);
            if (copied_per_iter == 0) break;
            vuint32m1_t data = vle32_v_u32m1(in, copied_per_iter);
            vse32_v_u32m1(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8m2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m2(n/1);
            if (copied_per_iter == 0) break;
            vuint8m2_t data = vle8_v_u8m2(in, copied_per_iter);
            vse8_v_u8m2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16m2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m2(n/2);
            if (copied_per_iter == 0) break;
            vuint16m2_t data = vle16_v_u16m2(in, copied_per_iter);
            vse16_v_u16m2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32m2(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m2(n/4);
            if (copied_per_iter == 0) break;
            vuint32m2_t data = vle32_v_u32m2(in, copied_per_iter);
            vse32_v_u32m2(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8m4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m4(n/1);
            if (copied_per_iter == 0) break;
            vuint8m4_t data = vle8_v_u8m4(in, copied_per_iter);
            vse8_v_u8m4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16m4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m4(n/2);
            if (copied_per_iter == 0) break;
            vuint16m4_t data = vle16_v_u16m4(in, copied_per_iter);
            vse16_v_u16m4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32m4(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m4(n/4);
            if (copied_per_iter == 0) break;
            vuint32m4_t data = vle32_v_u32m4(in, copied_per_iter);
            vse32_v_u32m4(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e8m8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e8m8(n/1);
            if (copied_per_iter == 0) break;
            vuint8m8_t data = vle8_v_u8m8(in, copied_per_iter);
            vse8_v_u8m8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 1 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e16m8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e16m8(n/2);
            if (copied_per_iter == 0) break;
            vuint16m8_t data = vle16_v_u16m8(in, copied_per_iter);
            vse16_v_u16m8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 2 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
void vector_memcpy_unit_stride_e32m8(size_t n, const int8_t* __restrict__ in, int8_t* __restrict__ out) {
    while (true) {
         {
            size_t copied_per_iter = vsetvl_e32m8(n/4);
            if (copied_per_iter == 0) break;
            vuint32m8_t data = vle32_v_u32m8(in, copied_per_iter);
            vse32_v_u32m8(out, data, copied_per_iter);
            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
    // Cleanup, in case 'n' wasn't a clean multiple of 4 bytes;
    while (n > 0) {
        *out = *in;
        out += 1;
        in += 1;
        n -= 1;
    }
}
#ifdef __cplusplus;
extern "C" {;
#endif // __cplusplus;
int main(void) {
    int *outputDevice = (int*) 0xf0000000;
    // magic output device;
    int result = 0;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8mf8) << 0;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16mf8) << 1;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32mf8) << 2;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8mf4) << 3;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16mf4) << 4;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32mf4) << 5;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8mf2) << 6;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16mf2) << 7;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32mf2) << 8;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8m1) << 9;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16m1) << 10;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32m1) << 11;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8m2) << 12;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16m2) << 13;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32m2) << 14;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8m4) << 15;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16m4) << 16;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32m4) << 17;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e8m8) << 18;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e16m8) << 19;
    result |= vector_memcpy_harness(vector_memcpy_unit_stride_e32m8) << 20;
    outputDevice[0] = result;
    return result;
}
#ifdef __cplusplus;
};
#endif // __cplusplus;
