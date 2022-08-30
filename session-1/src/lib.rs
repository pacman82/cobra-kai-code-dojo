pub fn multiples_of_5_and_3(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
