use std::hash::Hasher;
use std::collections::hash_map::*;
use fxhash::FxHasher64;

pub const HASHES : [(&str, fn(u64) -> u64); 3] = [
    ("StdHash", std_hash),
    ("FxHash", fx_hash),
    ("CtrlHash", |x : u64| x >> 32)
];

pub const VEC_HASHES : [(&str, fn(&Vec<u8>) -> u64); 3] = [
    ("StdHash", std_hash_vec),
    ("FxHash", fx_hash_vec),
    ("CtrlHash", |x : &Vec<u8>| x[0] as u64)
];

fn fx_hash(x : u64) -> u64 {
    let mut hasher: FxHasher64 = FxHasher64::default();
    hasher.write_u64(x);
    hasher.finish()
}

fn std_hash(x : u64) -> u64 {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    hasher.write_u64(x);
    hasher.finish()
}

fn fx_hash_vec(x : &Vec<u8>) -> u64 {
    let mut hasher: FxHasher64 = FxHasher64::default();
    hasher.write(&x);
    hasher.finish()
}

fn std_hash_vec(x : &Vec<u8>) -> u64 {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    hasher.write(&x);
    hasher.finish()
}