// TEST: input size vs. time-to-hash
//  1. SipHash: High quality    + slow. (current std hasher)
//  3. FnvHash: Medium quality  + moderate.
//  2. FxHash:  Low quality     + fast.
//  4. AbseilHash

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("gotta-go-fast");
    let mut hasher: DefaultHasher = DefaultHasher::new();

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