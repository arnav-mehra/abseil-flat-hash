use std::hash::Hasher;
use std::collections::hash_map::*;

pub trait Hashable {
    fn hash(&self) -> u64;
}

impl Hashable for u32 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

impl Hashable for i32 {
    fn hash(&self) -> u64 {
        // let mut hasher: DefaultHasher = DefaultHasher::new();
        // hasher.write_i32(*self);
        // hasher.finish()
        (*self as u64) << 7
    }
}

impl Hashable for u64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

impl Hashable for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}