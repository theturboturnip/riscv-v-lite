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
  if (fibbonacci(33) == 3524578){
    return 1;
  }
  return 0;
}

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result = 0;

  result |= fac_test() << 0;
  result |= fib_test() << 1;


#define VECTOR_TEST 0
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
  asm volatile(
      "vl1r.v v1, %0" 
      :  // no variable outputs
      : "m"(in_addr) // input, 'r' -> register
  );
  asm volatile(
      "vs1r.v v1, %0" 
      :: "m"(out_addr)
      : "memory"
  );
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

  outputDevice[0] = result;
  return result;
}