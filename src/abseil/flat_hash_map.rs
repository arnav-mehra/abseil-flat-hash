#[path="hash.rs"] mod hash;
use hash::Hashable;
// https://abseil.io/about/design/swisstables

pub struct AFHM<K, V> {
    pub meta: Vec<u8>, 
    pub arr: Vec<(K, V)>,
    pub size: usize,
    pub tombstones: usize
}

const INITIAL_SIZE : usize = 10;
const EMPTY_ENTRY : u8 = 0b1000_0000;
const TOMBSTONE_ENTRY : u8 = 0b1111_1110;

impl<K: Hashable + Eq + Copy, V: Hashable + Copy> AFHM<K, V> {

    pub fn new() -> AFHM<K, V> {
        let mut v: Vec<(K, V)> = Vec::with_capacity(INITIAL_SIZE);
        for _ in 0..16 { v.push((K::DEFAULT, V::DEFAULT)) }
        AFHM {
            meta: vec![EMPTY_ENTRY; INITIAL_SIZE],
            arr: v,
            size: 0,
            tombstones: 0
        }
    }

    pub fn get_loc(&self, k : K) -> usize {
        // hashing
        let (bot_7, mut ind) = self.get_hash(&k);

        // probing
        while !self.is_empty(ind) {
            let metadata_value = self.meta[ind];
            if metadata_value == bot_7 {
                if self.arr[ind].0 == k {
                    return ind;
                }
            }
            ind = (ind + 1) & (self.capacity() - 1);
        }
        usize::MAX
    }

    pub fn get(&self, k : K) -> Option<V>{
        match self.get_loc(k) {
            usize::MAX => {return None;}
            loc => {
                let (_,V) = &self.arr[loc];
                return Some(V.clone());
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        // hashing
        let (bot_7, mut ind) = self.get_hash(&k);

        // probing
        while !self.is_full(ind) {
            ind = (ind + 1) & (self.capacity() - 1);
        }

        // updates
        self.meta[ind] = bot_7;
        self.arr[ind] = (k, v);
        self.size += 1;
    }

    
    
    pub fn erase(&mut self, k: K) {
        match self.get_loc(k) {
            usize::MAX => {}
            loc => {
                self.meta[loc] = TOMBSTONE_ENTRY;
                self.tombstones += 1;
            }
        }
    }

    // pub fn load(&self) -> f32 {
    //     return (self.size() as f32) / (self.capacity() as f32);
    // }

    // pub fn size(&self) -> usize {
    //     return self.size;
    // }

    pub fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    pub fn top_57(&self, hash : u64) -> u64{
        hash >> 7
    }

    pub fn bot_7(&self, hash : u64) -> u8 {
        (0b1_111111 & hash) as u8
    }

    pub fn get_hash(&self, k: &K) -> (u8, usize) {
        let hash_val: u64 = Hashable::hash(k);
        let bitmask: usize = self.arr.capacity() - 1;
        let ind: usize = (self.top_57(hash_val) as usize) & bitmask;
        (self.bot_7(hash_val), ind)
    }

    pub fn is_full(&self, ind: usize) -> bool {
        (self.meta[ind] & EMPTY_ENTRY) == 0
    }

    pub fn is_empty(&self, ind: usize) -> bool {
        self.meta[ind] == EMPTY_ENTRY
    }

    pub fn is_deleted(&self, ind: usize) -> bool {
        self.meta[ind] == TOMBSTONE_ENTRY
    }
}