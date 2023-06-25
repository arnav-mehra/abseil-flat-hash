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
    fn hash(&self) -> u64 { *self as u64 }
}

impl Hashable for u64 {
    const DEFAULT: u64 = 0;
    fn hash(&self) -> u64 { *self as u64 }
}

impl Hashable for i64 {
    const DEFAULT: i64 = 0;
    fn hash(&self) -> u64 { *self as u64 }
}