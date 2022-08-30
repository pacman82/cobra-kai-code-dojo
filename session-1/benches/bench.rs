use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_1::multiples_of_3_and_5;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("up_to 10", |b| b.iter(|| multiples_of_3_and_5(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
