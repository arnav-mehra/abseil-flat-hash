#![feature(portable_simd)]

#[path="../../src/abseil/mod.rs"] mod abseil;
use abseil::flat_hash_map::AFHM;

use rand::Rng;
use criterion::{Criterion};

const N : usize = 10_000_000;

fn get_rands() -> Vec<u64> {
    let mut res: Vec<u64> = vec![0; N];
    for i in 0..N {
        let mut rng = rand::thread_rng();
        res[i] = rng.gen();
    }
    res
}

pub fn criterion_benchmark(c : &mut Criterion) {
    let mut hmap: AFHM<i32, i32> = AFHM::new();

    for i in 0..100_000_000 {
        hmap.insert(i, i);
    }

    let rands = get_rands();

    let mut group = c.benchmark_group("SIMD");
    group.bench_function(
        "Pre",
        |b: &mut criterion::Bencher<'_>| {
            for i in 0..N {
                hmap.get_loc(rands[i] as i32);
            }
        }
    );
    group.bench_function(
        "Post",
        |b: &mut criterion::Bencher<'_>| {
            for i in 0..N {
                hmap.get_loc_simd(rands[i] as i32);
            }
        }
    );
    group.finish();
}