use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use histongram::Ngrams;

const APACHE: &str = include_str!("../LICENSE-APACHE");

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ngrams");
    for len in [100, APACHE.chars().count()] {
        group.throughput(Throughput::Elements(len as u64));

        group.bench_with_input(
            BenchmarkId::new("count", len),
            &APACHE[..len],
            |b, input| b.iter(|| Ngrams::new(1..=5).count(input.split_whitespace())),
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
