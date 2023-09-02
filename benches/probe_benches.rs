#![feature(portable_simd)]

use criterion::{criterion_group, criterion_main};

#[path="./probe/pre_post_simd.rs"] mod pre_post_simd;

criterion_group!(benches, pre_post_simd::criterion_benchmark);
criterion_main!(benches);