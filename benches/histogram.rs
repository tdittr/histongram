use criterion::{black_box, criterion_group, criterion_main, Criterion};

use histongram::Histogram;

const APACHE: &str = include_str!("../LICENSE-APACHE");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fill", |b| {
        b.iter(|| Histogram::from_iter(black_box(APACHE).chars()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
