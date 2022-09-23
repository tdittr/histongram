extern crate alloc;

use std::collections::hash_map::RandomState;
use std::hash::BuildHasherDefault;

use compact_str::CompactString;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use histongram::Histogram;

const APACHE: &str = include_str!("../LICENSE-APACHE");

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("chars");
    for len in [100, APACHE.chars().count()] {
        group.throughput(Throughput::Elements(len as u64));

        group.bench_with_input(
            BenchmarkId::new("fill_fxhash", len),
            &APACHE[..len],
            |b, input| {
                b.iter(|| {
                    Histogram::<_, BuildHasherDefault<rustc_hash::FxHasher>>::from_owned_iter(
                        input.chars(),
                    )
                })
            },
        );

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

    let mut group = c.benchmark_group("words");
    for len in [100, APACHE.split_whitespace().count()] {
        group.throughput(Throughput::Elements(len as u64));
        let words = || black_box(APACHE.split_whitespace().take(len));

        group.bench_with_input(BenchmarkId::new("fill_fxhash", len), &(), |b, _| {
            b.iter(|| {
                Histogram::<CompactString, BuildHasherDefault<rustc_hash::FxHasher>>::from_iter(
                    words(),
                )
            })
        });
        group.bench_with_input(BenchmarkId::new("fill_ahash", len), &(), |b, _| {
            b.iter(|| Histogram::<CompactString, ahash::RandomState>::from_iter(words()))
        });
        group.bench_with_input(BenchmarkId::new("fill_std", len), &(), |b, _| {
            b.iter(|| Histogram::<CompactString, RandomState>::from_iter(words()))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
