pub fn sum_even_fib(cutoff: u64) -> u64 {
    FibSeq::new()
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum()
}

struct FibSeq {
    a: u64,
    b: u64,
}

impl FibSeq {
    pub fn new() -> FibSeq {
        FibSeq { a: 0, b: 1 }
    }
}

impl Iterator for FibSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(self.b)
    }
}
