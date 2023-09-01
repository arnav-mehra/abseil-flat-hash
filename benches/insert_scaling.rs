// mass insert times vs. other hashmaps

#[path="../src/abseil/mod.rs"] mod abseil;
use abseil::flat_hash_map::AFHM;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use criterion::{criterion_group, criterion_main, Criterion};

const HASHES : [(&str, fn(u64) -> u64); 3] = [
    ("StdHash", std_hash),
    ("FxHash", fx_hash),
    ("CtrlHash", |x : u64| x >> 32)
];

fn criterion_benchmark(_ : &mut Criterion) {
    let mut hmap: AFHM<i32, i32> = AFHM::new();

    for i in 0..100_000_000 {
        hmap.insert(i, i);
    }

    for (hash_name, hash_fn) in HASHES {


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);