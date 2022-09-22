use std::collections::hash_map::RandomState;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use histongram::Histogram;

const APACHE: &str = include_str!("../LICENSE-APACHE");

pub fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "fxhash")]
    c.bench_function("fill_fxhash", |b| {
        b.iter(|| Histogram::<_, fxhash::FxBuildHasher>::from_owned_iter(black_box(APACHE).chars()))
    });
    c.bench_function("fill_ahash", |b| {
        b.iter(|| Histogram::<_, ahash::RandomState>::from_owned_iter(black_box(APACHE).chars()))
    });
    c.bench_function("fill_std", |b| {
        b.iter(|| Histogram::<_, RandomState>::from_owned_iter(black_box(APACHE).chars()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
