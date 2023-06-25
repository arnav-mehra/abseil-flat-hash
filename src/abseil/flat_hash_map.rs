#[path="hash.rs"] mod hash;
use hash::Hashable;
// https://abseil.io/about/design/swisstables

pub struct AFHM<K, V> {
    pub meta: Vec<u8>, 
    pub arr: Vec<(K, V)>,
    pub size: usize
}

impl<K: Hashable, V: Hashable> AFHM<K, V> {
    pub fn new() -> AFHM<K, V> {
        let mut v: Vec<(K, V)> = Vec::with_capacity(16);
        for _ in 0..16 { v.push((K::DEFAULT, V::DEFAULT)) }
        AFHM {
            meta: vec![0x80; 16],
            arr: v,
            size: 0
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        // hashing
        let (hash_val, mut ind) = self.get_hash(&k);

        // probing
        let mut i: usize = 0;
        while !self.is_empty(ind) {
            ind += (i << 1) + 1;
            ind &= self.capacity() - 1;
            i += 1;
        }

        // updates
        self.meta[ind] = (hash_val & 0x7F) as u8;
        self.arr[ind] = (k, v);
        self.size += 1;
    }

    // pub fn erase(&mut self, k: K) {
    //     // hashing
    //     let (hash_val, mut ind) = self.get_hash(&k);
        
    //     // probing
    //     let mut i: usize = 0;
    //     while self.meta[ind] != (hash_val & 0x7F) as u8 { // TO-DO: add true equiv test
    //         if self.is_empty(ind) { return; }
    //         ind += (i << 1) + 1;
    //         ind &= self.capacity() - 1;
    //         i += 1;
    //     }

    //     // updates
    //     self.meta[ind] = 0b_1_1111110;
    //     self.size -= 1;
    // }

    // pub fn load(&self) -> f32 {
    //     return (self.size() as f32) / (self.capacity() as f32);
    // }

    // pub fn size(&self) -> usize {
    //     return self.size;
    // }

    pub fn capacity(&self) -> usize {
        return self.arr.capacity();
    }

    pub fn get_hash(&self, k: &K) -> (u64, usize) {
        let hash_val: u64 = Hashable::hash(k);
        let bitmask: usize = self.arr.capacity() - 1;
        let ind: usize = ((hash_val >> 7) as usize) & bitmask;
        (hash_val, ind)
    }

    pub fn is_full(&self, ind: usize) -> bool {
        (self.meta[ind] & 0b_1_0000000) == 0
    }

    pub fn is_empty(&self, ind: usize) -> bool {
        self.meta[ind] == 0b_1_0000000
    }

    // pub fn is_deleted(&self, ind: usize) -> bool {
    //     self.meta[ind] == 0b_1_1111110
    // }
}