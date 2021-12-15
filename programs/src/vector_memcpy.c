// int fib(const int a)
// {
//   if(a<2)
//     return a;
//   else
//     return fib(a-1) + fib(a-2);
// }

#include <riscv_vector.h>
#include <stdint.h>

// n = number of elements to copy
// in = pointer to data (should be aligned to 128-bit?)
// out = pointer to output data (should be aligned?)
void vector_memcpy_e8(size_t n, const int32_t* __restrict__ in, const int32_t* __restrict__ out) {
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

int vector_memcpy_harness() {
    static int data[2048] = {0};
    static int out_data[2048] = {0};

    for (int i = 0; i < 2048; i++) {
        data[i] = i;
    }

    vector_memcpy_e8(2048, data, out_data);

    for (int i = 0; i < 2048; i++) {
        if (data[i] != out_data[i]) {
            return 0;
        }
    }
    return 1;
}

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result;
  result = vector_memcpy_harness();
  outputDevice[0] = result;
  return result;
}