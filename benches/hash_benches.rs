use criterion::{criterion_group, criterion_main};

#[path="./hash/quality.rs"] mod hash_quality;
#[path="./hash/speed.rs"] mod hash_speed;

criterion_group!(
    benches,
    hash_quality::criterion_benchmark,
    hash_speed::criterion_benchmark
);

criterion_main!(benches);