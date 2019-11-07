use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

// An alternative API was considered for an UnorderedHasher which implements Hasher by treating each call to write(&mut self, bytes: &[u8])
// as being atomic. This quickly runs into issues, as eg: &[u64] makes only 2 calls to write, first with the length of the slice,
// and again with the binary representation of all it's items. So, it's impossible for the UnorderedHasher to know
// what the boundaries between atomic hashables are.
pub struct UnorderedHasher<T = DefaultHasher> {
    hash: u64,
    _marker: PhantomData<*const T>,
}

impl UnorderedHasher {
    pub fn finish(&self) -> u64 {
        self.hash
    }
}

impl<T: Hasher + Default> UnorderedHasher<T> {
    pub fn new() -> Self {
        Self {
            hash: 0,
            _marker: PhantomData,
        }
    }

    pub fn write<H: Hash>(&mut self, value: &H) {
        // Hashing each independently than adding the results is simple, but correct.
        // If the original hash function is well distributed, then the addition
        // will also result in a well distributed value.
        let mut hasher = T::default();
        value.hash(&mut hasher);
        self.hash = self.hash.wrapping_add(hasher.finish());
    }
}

impl Default for UnorderedHasher {
    fn default() -> Self {
        Self::new()
    }
}

pub fn unordered_hash<T: Hash>(values: impl Iterator<Item = T>) -> u64 {
    let mut hash = 0;
    for value in values {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        hash = hasher.finish().wrapping_add(hash);
    }
    hash
}
