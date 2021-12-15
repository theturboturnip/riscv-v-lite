int fib(const int a)
{
  if(a<2)
    return a;
  else
    return fib(a-1) + fib(a-2);
}

int main(void)
{
  int *outputDevice = (int*) 0xf0000000; // magic output device
  int result;
  result = fib(8);
  outputDevice[0] = result;
  return result;
}