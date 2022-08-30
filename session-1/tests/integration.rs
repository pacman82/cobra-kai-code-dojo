use session_1::multiples_of_3_and_5;

#[test]
fn should_be_23_for_10() {
    let actual = multiples_of_3_and_5(10);
    assert_eq!(23, actual);
}

#[test]
fn should_be_233168_for_1000() {
    let actual = multiples_of_3_and_5(1000);
    assert_eq!(233168, actual);
}

#[test]
fn should_be_identical_to_slow_and_simple_solution() {
    for n in 0..=1000 {
        assert_eq!(
            multiples_of_5_and_3_slow_and_simple(n),
            multiples_of_3_and_5(n)
        )
    }
}

pub fn multiples_of_5_and_3_slow_and_simple(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
