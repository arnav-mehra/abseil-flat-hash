#![feature(portable_simd)]
#![feature(pointer_is_aligned)]
#![feature(stdsimd)]
#![feature(unchecked_math)]

#[path="../src/abseil/mod.rs"] mod abseil;
use abseil::flat_hash_map::AFHM;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(_ : &mut Criterion) {
    let mut hmap: AFHM<i32, i32> = AFHM::new();

    for i in 0..100_000_000 {
        hmap.insert(i, i);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);