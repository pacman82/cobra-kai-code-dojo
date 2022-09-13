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

#[test]
fn should_be_identical_to_slow_and_simple_solution() {
    for n in 0..=1000 {
        assert_eq!(
            sum_even_fib_slow_and_simple(n),
            sum_even_fib(n)
        )
    }
}

fn sum_even_fib_slow_and_simple(cutoff: u64) -> u64 {
    fn fib(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 2,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
    (0..cutoff)
        .map(fib)
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= cutoff)
        .sum()
}