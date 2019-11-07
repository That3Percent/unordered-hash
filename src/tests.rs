use crate::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn order_independent_hash<T: Hash>(value: &[T]) -> u64 {
    let mut hasher = UnorderedHasher::new();
    for item in value {
        hasher.write(item);
    }
    hasher.finish()
}

#[test]
fn list_permutations_have_same_hash() {
    let mut values: Vec<_> = (0..50).collect();

    let mut hash = order_independent_hash(&values);

    while values.pop().is_some() {
        // Strictly speaking, we can't guarantee no collision here. But, we still want some test to say that
        // there is some variation in hashes. We don't have to worry about the birthday paradox, so this is unlikely
        // enough
        assert_ne!(hash, order_independent_hash(&values));
        hash = order_independent_hash(&values);

        for _ in 0..10 {
            values.shuffle(&mut thread_rng());

            assert_eq!(hash, order_independent_hash(&values));
        }
    }
}
