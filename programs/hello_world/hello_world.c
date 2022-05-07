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

volatile extern int64_t outputAttempted; // magic output device
volatile extern int64_t outputSucceeded; // magic output device

int main(void)
{
  int result = 0;

  result |= fac_test() << 0;
  result |= fib_test() << 1;
  result |= fib_memo_test() << 2;

  int ran = 0b111;

  // On pure-capability platforms this is equivalent to *(&outputAttempted) = ran;
  // but on hybrid-capability platforms this tests if capabilities can still be constructed and used.
  #if __has_feature(capabilities)
  volatile int64_t* __capability data = &outputAttempted;
  *data = ran;
  #else 
  *(&outputAttempted) = ran;
  #endif

  *(&outputSucceeded) = result;
  return result;
}