pub fn sum_even_fib(cutoff: u64) -> u64 {
    (0..cutoff)
        .map(fib)
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum()
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}