// int fib(const int a)
// {
//   if(a<2)
//     return a;
//   else
//     return fib(a-1) + fib(a-2);
// }

#include <riscv_vector.h>
#include <stdint.h>

void* memset(void* dest, int ch, size_t count) {
    unsigned char ch_uc = (unsigned char)ch;
    unsigned char* dest_uc = (unsigned char*)dest;
    for (int i = 0; i < count; i++) {
        *(dest_uc + i) = ch_uc;
    }

    return dest_uc;
}

// f2 => LMUL = 1/2, which means only 1/2 of the vector register is used per iteration
// I would use 1/4, but it doesn't exist for 32bit in the intrinsics
void vector_memcpy_32mf2(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    size_t copied_per_iter = 0;
    for (; n > 0; n -= copied_per_iter) {
        copied_per_iter = vsetvl_e32mf2(n);
        // copied_per_iter is included in the intrinsic, not because it changes the actual instruction,
        // but if you wanted to change it it would do vsetvl to set architectural state
        vint32mf2_t data = vle32_v_i32mf2(in, copied_per_iter);
        vse32_v_i32mf2(out, data, copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
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
        vint32m8_t data = vle32_v_i32m8(in, copied_per_iter);
        vse32_v_i32m8(out, data, copied_per_iter);

        in += copied_per_iter;
        out += copied_per_iter;
    }
}

int vector_memcpy_harness(void (*memcpy_fn)(size_t, const int32_t*, int32_t*)) {
    int data[2048] = {0};
    int out_data[2048] = {0};

    for (int i = 0; i < 2048; i++) {
        data[i] = i;
    }

    // ONLY copy 1003 values
    memcpy_fn(1003, data, out_data);

    // Check the first 1003 values of output are the same
    // This ensures that the emulator correctly loaded/stored enough values
    for (int i = 0; i < 1003; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    // Check that the rest are 0 (the original value)
    // This ensures that the emulator didn't store more elements than it should have
    for (int i = 1003; i < 2048; i++) {
        if (out_data[i] != 0) {
            return 0;
        }
    }
    return 1;
}

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result;
  result = vector_memcpy_harness(vector_memcpy_32m8);
  result |= vector_memcpy_harness(vector_memcpy_32mf2) << 1;
  outputDevice[0] = result;
  return result;
}