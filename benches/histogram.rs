use std::collections::hash_map::RandomState;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use histongram::Histogram;

const APACHE: &str = include_str!("../LICENSE-APACHE");

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("chars");

    for len in [10, 100, APACHE.chars().count()] {
        group.throughput(Throughput::Elements(len as u64));

        #[cfg(feature = "fxhash")]
        {
            group.bench_with_input(
                BenchmarkId::new("fill_fxhash", len),
                &APACHE[..len],
                |b, input| {
                    b.iter(|| Histogram::<_, fxhash::FxBuildHasher>::from_owned_iter(input.chars()))
                },
            );
        }
        group.bench_with_input(
            BenchmarkId::new("fill_ahash", len),
            &APACHE[..len],
            |b, input| {
                b.iter(|| Histogram::<_, ahash::RandomState>::from_owned_iter(input.chars()))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("fill_std", len),
            &APACHE[..len],
            |b, input| b.iter(|| Histogram::<_, RandomState>::from_owned_iter(input.chars())),
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
