#ifndef CHERI_VECTOR_WRAPPERS
#define CHERI_VECTOR_WRAPPERS
#include <stdint.h>
#include <riscv_vector.h>

// Script for compatibility with old vector_memcpy
// Used to have wrapper functions for vector intrinsics on CHERI, but now we don't

#if __has_feature(capabilities)
#error "Doesn't support pure-capability compilation"
#else
#define VEC_INTRIN(i) i
#define VEC_TYPE(T) T
#endif // __has_feature(capabilities)

#endif // CHERI_VECTOR_WRAPPERS