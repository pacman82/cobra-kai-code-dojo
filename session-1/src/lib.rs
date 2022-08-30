/// Sum of all numbers which are divided by either 3 or 5 and are below `up_to`.
/// 
/// ```
/// use session_1::multiples_of_3_and_5;
/// 
/// assert_eq!(23, multiples_of_3_and_5(10))
/// ```
pub fn multiples_of_3_and_5(up_to: u64) -> u64 {
    multiples_of(3, up_to) + multiples_of(5, up_to) - multiples_of(15, up_to)
}

/// Sum of all numbers which are divided by either `of` and are below `up_to`.
fn multiples_of(of: u64, up_to: u64) -> u64 {
    if up_to == 0 {
        0
    } else {
        of * sum((up_to - 1)/of)
    }
}

fn sum(n: u64) -> u64 {
    n * (n + 1) / 2
}
