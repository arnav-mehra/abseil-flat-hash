#![feature(portable_simd)]

#[path="../../src/abseil/mod.rs"] mod abseil;
use abseil::flat_hash_map::AFHM;

use rand::Rng;
use criterion::{Criterion};

const N : usize = 10_000_000;

pub fn criterion_benchmark(c : &mut Criterion) {
    let mut hmap: AFHM<i32, i32> = AFHM::new();

    for i in 0..100_000_000 {
        hmap.insert(i, i);
    }

    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("SIMD");
    
    group.bench_function(
        "Pre",
        |b| {
            b.iter(|| {
                let rand: i32 = rng.gen();
                hmap.get_loc(rand);
            })
        }
    );

    group.bench_function(
        "Post",
        |b: &mut criterion::Bencher<'_>| {
            b.iter(|| {
                let rand: i32 = rng.gen();
                hmap.get_loc_simd(rand);
            })
        }
    );
    
    group.finish();
}