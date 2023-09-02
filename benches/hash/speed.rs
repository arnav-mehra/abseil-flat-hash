// TEST: input size vs. time-to-hash
//  1. SipHash: High quality    + slow. (current std hasher)
//  3. FnvHash: Medium quality  + moderate.
//  2. FxHash:  Low quality     + fast.
//  4. AbseilHash

#[path="./functions.rs"] mod hash_functions;
use hash_functions::VEC_HASHES;

use criterion::{Criterion, BenchmarkId, Throughput};

pub fn criterion_benchmark(c: &mut Criterion) {
    for (hash_name, hash_fn) in VEC_HASHES {
        let mut group = c.benchmark_group(hash_name);

        for size in [8, 64, 256].iter() {
            let input: Vec<u8> = vec![1; *size];
            group.throughput(Throughput::Elements(*size as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(size),
                &input,
                |b: &mut criterion::Bencher<'_>, input| {
                    b.iter(|| hash_fn(&input));
                }
            );
        }

        group.finish();
    }    
}