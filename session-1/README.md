# Session 1 - Code Dojo

## Kata / Problem

### Multiples of 3 and 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the multiples of 3 or 5 below 1000.

Source: <https://projecteuler.net/problem=1>

## Learning steps

### TDD workflow

Start with the brute force implementation in a test, for a library. Hardcode the test to `10` and expect `23` as in the problem definition.

```rust
#[test]
fn should_be_23_for_10() {
    let actual: u64 = (1..10).filter(|i| i % 3 == 0 || i % 5 == 0).sum();
    assert_eq!(23, actual);
}
```

Now extract that function into the library.

```rust
pub fn multiples_of_5_and_3() -> u64 {
    (1..10).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
```

Now you can use this to determine the result for `1000`, if interessted in solving the project euler problem. For this we must make the argument not hardcoded:

```rust
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
```
