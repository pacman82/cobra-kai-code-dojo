pub fn sum_even_fib(cutoff: u64) -> u64 {
    fib_it()
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum()
}

struct Fib {
    a: u64,
    b: u64,
}

fn fib_it() -> impl Iterator<Item = u64> {
    (0..).scan(Fib { a: 1, b: 2 }, |fib, _| {
        let new = fib.a + fib.b;
        fib.a = fib.b;
        fib.b = new;
        Some(new)
    })
}
