

pub fn fib(index: u32) -> u64 {
    if index < 2 && index > 0 {
      return 1;
    } else if index == 0 || index < 0 {
        return 0;
    }
    return fib(index - 1) + fib(index - 2);
}