pub fn multiples_of_5_and_3(up_to: u64) -> u64 {
    multiples_of(3, up_to) + multiples_of(5, up_to) - multiples_of(15, up_to)
}

fn multiples_of(of: u64, up_to: u64) -> u64 {
    (1..up_to).filter(|i| i % of == 0).sum()
}
