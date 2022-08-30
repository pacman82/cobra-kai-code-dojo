use session_1::multiples_of_5_and_3;

#[test]
fn should_be_23_for_10() {
    let actual = multiples_of_5_and_3(10);
    assert_eq!(23, actual);
}

#[test]
fn should_be_23_for_1000() {
    let actual = multiples_of_5_and_3(1000);
    assert_eq!(233168, actual);
}