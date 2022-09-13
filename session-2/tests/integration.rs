use session_2::sum_even_fib;

#[test]
fn should_be_44_for_cutoff_50() {
    // Given
    let cutoff = 50;

    // When
    let sum_even_fib = sum_even_fib(cutoff);

    // Assert
    assert_eq!(44u64, sum_even_fib);
}
