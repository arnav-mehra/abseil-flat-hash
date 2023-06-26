use std::f64::consts::{PI, E};

pub trait Hashable {
    const DEFAULT: Self;
    fn hash(&self) -> u64;
}

impl Hashable for u32 {
    const DEFAULT: u32 = 0;
    fn hash(&self) -> u64 { *self as u64 }
}

impl Hashable for i32 {
    const DEFAULT: i32 = 0;
    fn hash(&self) -> u64 {
        let mut x: u64 = (*self as f64).to_bits();
        for _ in 0..20 {
            x ^= (x << 21) ^ (x >> 17) ^ PI.to_bits();
        }
        x
    }
}

impl Hashable for u64 {
    const DEFAULT: u64 = 0;
    fn hash(&self) -> u64 { *self as u64 }
}

impl Hashable for i64 {
    const DEFAULT: i64 = 0;
    fn hash(&self) -> u64 { *self as u64 }
}