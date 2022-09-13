#[test]
fn should_be_44_for_cutoff_50() {
    // Given
    let cutoff = 50;

    // When

    // Yes we can have local functions in functions
    fn fib(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 2,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
    let sum_even_fib = (0..cutoff)
        .map(fib)
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum();

    // Assert
    assert_eq!(44u64, sum_even_fib);
}
