use super::hashable::Hashable;
use std::{simd::{u8x32, SimdPartialEq, u8x16, Simd, ToBitMask}};
use std::{arch::x86_64};
// SIMD: AVX2. Supports 256-bit instructions. Works since Intel Haswell processors.
// https://abseil.io/about/design/swisstables

pub struct AFHM<K, V> {
    pub meta: Vec<u8>,
    pub arr: Vec<(K, V)>,
    pub size: usize
}

type SimdChunk = u8x32;
const SIMD_CHUNK_SIZE : usize = 32;
const INITIAL_SIZE : usize = 32;

const EMPTY_ENTRY : u8 = 0b1000_0000;
const TOMBSTONE_ENTRY : u8 = 0b1111_1110;
const BOTTOM_SEVEN : u64 = 0b0111_1111;

const MAX_LOAD_FACTOR : f32 = 0.875;
const MIN_LOAD_FACTOR : f32 = 0.4375;

impl<
    K: Hashable + Default + Eq + Copy,
    V: Default + Copy
> AFHM<K, V> {

    // CONSTRUCTORS

    pub fn new() -> AFHM<K, V> {
        AFHM::with_capacity(INITIAL_SIZE)
    }

    pub fn with_capacity(size : usize) -> AFHM<K, V> {
        let mut new_map = AFHM {
            meta: vec![EMPTY_ENTRY; size + SIMD_CHUNK_SIZE],
            arr: vec![(K::default(), V::default()); size],
            size: 0
        };
        for i in size..(size + SIMD_CHUNK_SIZE) {
            new_map.meta[i] = EMPTY_ENTRY;
        }
        new_map
    }

    // CRUD

    pub fn get(&self, k : K) -> Option<V> {
        match self.get_loc(k) {
            usize::MAX => None,
            loc => {
                let (_, v) = &self.arr[loc];
                Some(v.clone())
            }
        }
    }
    
    pub fn set(&mut self, k : K, v : V) {
        match self.get_loc(k) {
            usize::MAX => {}
            loc => {
                self.arr[loc].1 = v;
            }
        }
    }

    pub fn insert(&mut self, k : K, v : V) {
        self.expand_if_necessary();

        // hashing
        let (bot_7, mut ind) = self.get_hash(&k);

        // probing. stop at empty, tombstone, or key match.
        while self.is_full(ind) {
            if self.check_key(k, bot_7, ind) {
                return;
            }
            ind = (ind + 1) & (self.capacity() - 1);
        }
        
        // updates
        self.size += 1;
        self.meta[ind] = bot_7;
        if ind < SIMD_CHUNK_SIZE {
            let ind_cpy : usize = ind + self.capacity();
            self.meta[ind_cpy] = bot_7;
        }
        self.arr[ind] = (k, v);
    }

    pub fn erase(&mut self, k : K) {
        match self.get_loc(k) {
            usize::MAX => {}
            loc => {
                self.meta[loc] = TOMBSTONE_ENTRY;
                if loc < SIMD_CHUNK_SIZE {
                    let ind_cpy : usize = loc + self.capacity();
                    self.meta[ind_cpy] = TOMBSTONE_ENTRY;
                }
                self.size -= 1;
                self.shrink_if_necessary();
            }
        }
    }

    pub fn get_loc(&self, k : K) -> usize {
        // hashing
        let (bot_7, mut ind) = self.get_hash(&k);

        // probing
        while !self.is_empty(ind) {
            if self.check_key(k, bot_7, ind) {
                return ind;
            }
            ind = (ind + 1) & (self.capacity() - 1);
        }

        // return no match found
        usize::MAX
    }

    pub fn get_loc_simd(&self, k : K) -> usize { unsafe {
        // hashing
        let (bot_7, mut ind) = self.get_hash(&k);

        let target_match: x86_64::__m256i = x86_64::_mm256_set1_epi8(bot_7 as i8);
        let target_empty: x86_64::__m256i = x86_64::_mm256_set1_epi8(1i8 << 7);

        // probing
        loop {
            let chunk_ptr : *const i8 = self.meta.as_ptr().offset(ind as isize) as *const i8;
            let chunk: x86_64::__m256i = x86_64::_mm256_loadu_epi8(chunk_ptr);

            // SIMD equality search -> bit mask search
            let mut simd_matches : u32 = x86_64::_mm256_cmpeq_epu8_mask(chunk, target_match).reverse_bits();
            let mut simd_empties : u32 = x86_64::_mm256_cmpeq_epu8_mask(chunk, target_empty).reverse_bits();

            loop {
                // empty cell before matching cell, no match.
                if simd_empties > simd_matches {
                    return usize::MAX; 
                }

                // no empty or matching cells remaining, search next chunk.
                if simd_matches == 0u32 {
                    break; 
                }

                // check for match.
                let lzs : usize = simd_matches.leading_zeros() as usize;
                let i : usize = (lzs + ind) & (self.capacity() - 1);
                let key_matches : bool = self.arr[i].0 == k;
                if key_matches {
                    return i;
                }

                // remove leading 1 (leading hit was false positive).
                simd_matches &= simd_matches - 1; 
            }

            ind = (ind + SIMD_CHUNK_SIZE) & (self.capacity() - 1);
        }
    }}

    // HASHING

    fn top_57(&self, hash : u64) -> u64 {
        hash >> 7
    }

    fn bot_7(&self, hash : u64) -> u8 {
        (BOTTOM_SEVEN & hash) as u8
    }

    fn check_key(&self, k : K, bot_7 : u8, ind : usize) -> bool {
        let meta_val : u8 = self.meta[ind];
        if meta_val != bot_7 { return false; } // fail-fast
        let key_val : K = self.arr[ind].0;
        k == key_val
    }

    fn get_hash(&self, k : &K) -> (u8, usize) {
        let hash_val : u64 = Hashable::hash(k);
        let bitmask : usize = self.capacity() - 1;
        let ind : usize = (self.top_57(hash_val) as usize) & bitmask;
        (self.bot_7(hash_val), ind)
    }

    // SIZE

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    pub fn load(&self) -> f32 {
        (self.size() as f32) / (self.capacity() as f32)
    }

    fn expand_if_necessary(&mut self) {
        if self.load() > MAX_LOAD_FACTOR {
            self.replace_self(self.capacity() << 1);  
        }
    }

    fn shrink_if_necessary(&mut self) {
        if self.load() < MIN_LOAD_FACTOR && self.capacity() != INITIAL_SIZE {
            self.replace_self(self.capacity() >> 1);  
        }
    }
    
    fn replace_self(&mut self, new_capacity : usize) {
        let mut new_hm : AFHM<K, V> = AFHM::with_capacity(new_capacity);
        for i in 0..self.capacity() {
            if self.is_full(i) {
                let (k, v) = self.arr[i];
                new_hm.insert(k, v);
            }
        }
        self.arr = new_hm.arr;
        self.meta = new_hm.meta;
        self.size = new_hm.size;
    }

    // METADATA STATUS

    pub fn is_full(&self, ind : usize) -> bool {
        (self.meta[ind] & EMPTY_ENTRY) == 0
    }

    pub fn is_empty(&self, ind : usize) -> bool {
        self.meta[ind] == EMPTY_ENTRY
    }

    pub fn is_deleted(&self, ind : usize) -> bool {
        self.meta[ind] == TOMBSTONE_ENTRY
    }
}