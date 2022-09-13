use session_2::sum_even_fib;

#[test]
fn should_be_44_for_cutoff_50() {
    // Given
    let cutoff = 50;

    // When
    let sum_even_fib = sum_even_fib(cutoff);

    // Assert
    assert_eq!(44, sum_even_fib);
}

#[test]
fn should_cutoff_for_4_000_000() {
    // Given
    let cutoff = 4_000_000;

    // When
    let sum_even_fib = sum_even_fib(cutoff);

    // Assert
    assert_eq!(4613732, sum_even_fib);
}