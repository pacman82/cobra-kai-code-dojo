#[test]
fn should_be_23_for_10() {
    let actual: u64 = (1..10).filter(|i| i % 3 == 0 || i % 5 == 0).sum();
    assert_eq!(23, actual);
}