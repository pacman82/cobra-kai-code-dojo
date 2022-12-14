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

### Property based testing

Of course our implementation is currently brute force. Surly we can do better. However, often if we have an easy to understand solution it is a good idea to use it for property based testing, to verify the solution which might be more difficult to understand.

Copy the simple brute force implementation to the testframework again, and use it to generate an expectation for the first `1000` values.

```rust
#[test]
fn should_be_identical_to_slow_and_simple_solution() {
    for n in 0..=1000 {
        assert_eq!(
            multiples_of_5_and_3_slow_and_simple(n),
            multiples_of_5_and_3(n)
        )
    }
}

pub fn multiples_of_5_and_3_slow_and_simple(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
```

### Iterative development of the fast solution

Now lets go for the fast solution. Using forumlas instead of loops:

```rust
pub fn multiples_of_5_and_3(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
```

iteration

```rust
pub fn multiples_of_5_and_3(n: u64) -> u64 {
    let multiples_3: u64 = (1..n).filter(|i| i % 3 == 0).sum();
    let multiples_5: u64 = (1..n).filter(|i| i % 5 == 0).sum();
    let multiples_15: u64 = (1..n).filter(|i| i % 15 == 0).sum();
    multiples_3 + multiples_5 - multiples_15
}
```

iteration

```rust
pub fn multiples_of_5_and_3(up_to: u64) -> u64 {
    multiples_of(3, up_to) + multiples_of(5, up_to) - multiples_of(15, up_to)
}

fn multiples_of(of: u64, up_to: u64) -> u64 {
    (1..up_to).filter(|i| i % of == 0).sum()
}
```

iteration (What step I would have missed the `if up_to == 0` without the tests!)

```rust
fn multiples_of(of: u64, up_to: u64) -> u64 {
    if up_to == 0 {
        0
    } else {
        of * (0..=(up_to - 1)/of).sum::<u64>()
    }
}
```

iteration

```rust
fn multiples_of(of: u64, up_to: u64) -> u64 {
    if up_to == 0 {
        0
    } else {
        of * sum((up_to - 1)/of)
    }
}

fn sum(up_to: u64) -> u64 {
    (0..=up_to).sum()
}
```

iteration

```rust
fn sum(up_to: u64) -> u64 {
    up_to * (up_to + 1) / 2
}
```

Final solution

```rust
pub fn multiples_of_5_and_3(up_to: u64) -> u64 {
    multiples_of(3, up_to) + multiples_of(5, up_to) - multiples_of(15, up_to)
}

fn multiples_of(of: u64, up_to: u64) -> u64 {
    if up_to == 0 {
        0
    } else {
        of * sum((up_to - 1)/of)
    }
}

fn sum(up_to: u64) -> u64 {
    up_to * (up_to + 1) / 2
}
```

Please notice how we:

* Could compile and test between each step
* Did develop the solution top down (starting with a single test case)
* Did develop a specific solution (10) and generalized it later to `n` once we had the first feedback loop.
* The code did not necessarily get less readable.

The more complicated the problem the **better** this approach will work. Frankly, it does not shine yet, because we have so little unknowns on the way yet.

## Benchmarking

Use to add criterian bench mark suite to dev depnedencies:

```shell
cargo add --dev criterion
```

Create file `benches/bench.rs`.

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_1::multiples_of_3_and_5;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("up_to 10", |b| b.iter(|| multiples_of_3_and_5(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Add new benchmark suite to `Cargo.toml`

```toml
[[bench]]
name = "bench"
harness = false
```

Run with `cargo bench`. Find results in `target/criterion`.
