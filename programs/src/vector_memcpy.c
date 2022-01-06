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

void vector_memcpy_indexed(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    // Generate indices
    uint32_t indices[128] = {0};
    size_t vlmax = vsetvlmax_e32m4();

    for (size_t i = 0; i < vlmax; i++) {
        // Use xor to generate a shuffled index pattern
        indices[i] = ((uint32_t)i) ^ 1;
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

// TODO - Make this work once I figure out which intrinsics actually trigger a unit-stride-mask-load
/*
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
*/

void vector_memcpy_strided(size_t n, const int32_t* __restrict__ in, int32_t* __restrict__ out) {
    const size_t STRIDE_FACTOR = 4;
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
                vint32m1_t data = vlse32_v_i32m1(in+i, STRIDE_FACTOR, copied_per_iter);
                vsse32_v_i32m1(out+i, STRIDE_FACTOR, data, copied_per_iter);
            }

            in += copied_per_iter * STRIDE_FACTOR;
            out += copied_per_iter * STRIDE_FACTOR;
            n -= copied_per_iter * STRIDE_FACTOR;
        } else {
            // We don't have room to do STRIDE*elems,
            // pick up the rest with normal copies
            vint32m1_t data = vle32_v_i32m1(in, copied_per_iter);
            vse32_v_i32m1(out, data, copied_per_iter);

            in += copied_per_iter;
            out += copied_per_iter;
            n -= copied_per_iter;
        }
    }
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

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result = 0;
  result |= vector_memcpy_harness(vector_memcpy_32m8);
  result |= vector_memcpy_harness(vector_memcpy_32mf2) << 1;
  result |= vector_memcpy_harness(vector_memcpy_strided) << 2;
  result |= vector_memcpy_harness(vector_memcpy_indexed) << 3;
  result |= vector_memcpy_masked_harness(vector_memcpy_masked) << 4;
//   result |= vector_memcpy_masked_harness(vector_memcpy_masked_bytemaskload) << 5;
  outputDevice[0] = result;
  return result;
}