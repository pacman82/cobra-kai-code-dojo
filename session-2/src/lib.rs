pub fn sum_even_fib(cutoff: u64) -> u64 {
    fib_it()
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum()
}

fn fib_it() -> impl Iterator<Item = u64> {
    (0..).scan((1, 2), |(a, b), _| {
        let new = *a + *b;
        *a = *b;
        *b = new;
        Some(new)
    })
}
