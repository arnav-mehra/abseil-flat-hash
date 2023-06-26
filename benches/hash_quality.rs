// test various biased inputs to ensure uniform output (at various container sizes / bit masks)

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("gotta-go-fast");
    let mut hasher: DefaultHasher = DefaultHasher::new();

    let mut rng = rand::thread_rng();
    let rands: Vec<u64> = (0..100).map(
        |_| rng.gen_range(0..u64::MAX)
    ).collect();

    for size in [1, 2, 4, 8, 16, 32].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b: &mut criterion::Bencher<'_>, &size: &usize| {
                let s: Vec<u8> = vec![1; size];
                b.iter(|| {
                    hasher.write(&s);
                    hasher.finish();
                });
            }
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);