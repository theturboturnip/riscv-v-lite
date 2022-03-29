// int fib(const int a)
// {
//   if(a<2)
//     return a;
//   else
//     return fib(a-1) + fib(a-2);
// }

#include <stdint.h>


int factorial(int n) {
  //base case
  if (n == 0) {
    return 1;
  } else {
    return n * factorial(n-1);
  }
}

int fac_test() {
  if (factorial(10) == 3628800) {
    return 1;
  }
  return 0;
}

int fibbonacci(int n) {
  if (n == 0) {
    return 0;
  } else if(n == 1) {
    return 1;
  } else {
    return (fibbonacci(n-1) + fibbonacci(n-2));
  }
}

int fib_test() {
  // if (fibbonacci(33) == 3524578){
  if (fibbonacci(10) == 55){
    return 1;
  }
  return 0;
}

#define FIB_MEMO_ARRAY_LEN 50
int fib_memo(int n) {
  int fib_memo_array[FIB_MEMO_ARRAY_LEN];
  for (int i = 0; i < FIB_MEMO_ARRAY_LEN; i++) {
    if (i == 0) {
      fib_memo_array[i] = 0;
    } else if (i == 1) {
      fib_memo_array[i] = 1;
    } else {
      fib_memo_array[i] = fib_memo_array[i - 1] + fib_memo_array[i - 2];
    }
  }

  return fib_memo_array[n];
}

int fib_memo_test() {
  if (fib_memo(33) == 3524578){
    return 1;
  }
  return 0;
}

#include <riscv_vector.h>
/// THESE FUNCTIONS ARE WACKY AND WEIRD 
/// Because Clang RISC-V vector instructions don't support the use of "m"-constraints for src/dst addresses
/// we specify the address register directly.
/// We ensure the address register is always ca0 by taking advantage of the CHERI RISC-V ABI
/// ca0 is the register for the first function argument/return value (TR-951$C.3.1).
/// Because these functions only take/return a single pointer + a vector (stored in vector registers),
/// we know the ptr is the only thing that can take up ca0.
/// TODO - use solutions from https://stackoverflow.com/a/70472883 to ensure this further?
vint32m1_t vload(const int* ptr) {
  vint32m1_t data;
  asm volatile(
      "vle32.v %0, (ca0)" 
      : "=vr"(data) // vector output
      : // no input
  );
  return data;
}
void vstore(int* ptr, vint32m1_t data) {
  asm volatile(
      "vse32.v %0, (ca0)" 
      : // No output (we specify ca0 directly)
      : "vr"(data) // input = vector register data
      : "memory"   // this affects memory in some way
  );
}

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result = 0;

  result |= fac_test() << 0;
  result |= fib_test() << 1;
  result |= fib_memo_test() << 2;

  outputDevice[0] = result;


#define VECTOR_TEST 1
#if VECTOR_TEST
  // Instead of pulling addresses out of the aether
  // Make some arrays on the stack
  int in[4] = { 12, 13, 14, 15 };
  int out[4] = { 0, 0, 0, 0 };
  // Get their arrays (these should be in capability registers)
  int* in_addr = in;
  int* out_addr = out;
  int element_count;
  asm volatile(
    "vsetivli %0, 4, e32, m1, ta, ma"
    : "=r"(element_count)
  );
  // Use the in_addr, out_addr capability registers 
  // NOTE - this works with "vl1r.v v1, (ca0)" BUT Clang generates "vl1r.v v1, 0(ca0)". The immediate value breaks it.
  // asm volatile(
  //     "vl1r.v v1, %0" 
  //     :  // no variable outputs
  //     : "m"(in_addr) // input, 'r' -> register
  // );
  vint32m1_t data = vload(in_addr);
  // asm volatile(
  //     "vs1r.v v1, %0" 
  //     :: "m"(out_addr)
  //     : "memory"
  // );
  vstore(out_addr, data);
#endif
#define LOAD_TEST 0
#if LOAD_TEST
  volatile int in = 142;
  int out = 0;
  // asm(
  //   "c.sw %0, %1" : "=r"(out) : "m"(in)
  // );
  out = *(&(in));
#endif

  return result;
}