# Session 1 - Code Dojo

## Problem

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