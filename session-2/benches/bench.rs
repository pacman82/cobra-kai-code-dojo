use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_2::sum_even_fib;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cutoff 4.000.000", |b| {
        b.iter(|| sum_even_fib(black_box(4_000_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
