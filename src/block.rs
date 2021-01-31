use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

// Struct for the block where date is generic type T
#[derive(Copy, Clone)]
pub struct Block<T> {
    id: u64,
    previous_hash: u64,
    block_hash: u64,
    data: T,
}

impl<T> Block<T>
where
    T: Display,
{
    pub fn new(id: u64, previous_hash: u64, data: T) -> Self {
        // Instantiate new block before hashing
        let mut b = Self {
            id: id,
            previous_hash: previous_hash,
            block_hash: 0x0,
            data: data,
        };

        // Hash self
        b.block_hash = b.calculate_hash();

        return b;
    }

    pub fn get_hash(&self) -> u64 {
        self.block_hash
    }

    pub fn set_previous_hash(&mut self, prev_hash: u64) {
        self.previous_hash = prev_hash;
    }

    fn calculate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let obj = self.serialize();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    pub fn serialize(&self) -> String {
        format!("{}.{}.{}", self.id, self.previous_hash, self.data)
    }
}
