use super::hashable::Hashable;
// https://abseil.io/about/design/swisstables

pub struct AFHS<K> {
    pub meta: Vec<u8>, 
    pub arr: Vec<K>,
    pub size: usize
}

const INITIAL_SIZE : usize = 16;
const EMPTY_ENTRY : u8 = 0b1000_0000;
const TOMBSTONE_ENTRY : u8 = 0b1111_1110;
const BOTTOM_SEVEN : u64 = 0b0111_1111;

const MAX_LOAD_FACTOR : f32 = 0.875;
const MIN_LOAD_FACTOR : f32 = 0.4375;

impl<
    K: Hashable + Default + Eq + Copy
> AFHS<K> {

    // CONSTRUCTORS
    
    pub fn new() -> AFHS<K> {
        AFHS::with_capacity(INITIAL_SIZE)
    }

    pub fn with_capacity(size : usize) -> AFHS<K> {
        AFHS {
            meta: vec![EMPTY_ENTRY; size],
            arr: vec![K::default(); size],
            size: 0
        }
    }

    // CRUD

    pub fn has(&self, k : K) -> bool {
        self.get_loc(k) != usize::MAX
    }

    pub fn add(&mut self, k : K) {
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
        self.arr[ind] = k;
    }

    pub fn remove(&mut self, k : K) {
        match self.get_loc(k) {
            usize::MAX => {}
            loc => {
                self.meta[loc] = TOMBSTONE_ENTRY;
                self.size -= 1;
                self.shrink_if_necessary();
            }
        }
    }

    fn get_loc(&self, k : K) -> usize {
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
        let key_val : K = self.arr[ind];
        k == key_val
    }

    fn get_hash(&self, k : &K) -> (u8, usize) {
        let hash_val : u64 = Hashable::hash(k);
        let bitmask : usize = self.arr.capacity() - 1;
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
            self.replace_self(self.arr.capacity() << 1);  
        }
    }

    fn shrink_if_necessary(&mut self) {
        if self.load() < MIN_LOAD_FACTOR && self.capacity() != 16 {
            self.replace_self(self.arr.capacity() >> 1);  
        }
    }

    fn replace_self(&mut self, new_capacity : usize) {
        let mut new_hm : AFHS<K> = AFHS::with_capacity(new_capacity);
        for i in 0..self.capacity() {
            if self.is_full(i) {
                new_hm.add(self.arr[i]);
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