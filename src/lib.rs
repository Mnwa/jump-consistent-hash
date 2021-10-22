///! Rust implementation of the jump consistent hash [algorithm](https://arxiv.org/pdf/1406.2294.pdf) by John Lamping and Eric Veach.
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

type BuildHasherJump = BuildHasherDefault<DefaultHasher>;

/// Structure which create
#[derive(Clone, Debug)]
pub struct JumpConsistentHash<H = BuildHasherJump> {
    buckets: i32,
    hash_builder: H,
}

impl<H: BuildHasher> JumpConsistentHash<H> {
    /// Get bucket for key.
    pub fn get<K: Hash>(&self, key: &K) -> i32 {
        self.get_bucket(self.prepare_key(key))
    }

    #[inline]
    fn prepare_key<K: Hash>(&self, key: &K) -> u64 {
        let mut hasher = self.hash_builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn get_bucket(&self, mut key: u64) -> i32 {
        let mut b = 1;
        let mut j = 0;

        while j < self.buckets {
            b = j;
            key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
            j = ((b.wrapping_add(1) as f32) * (((1u64 << 31) as f32) / (((key >> 33) + 1) as f32)))
                as i32;
        }
        b
    }
}

impl JumpConsistentHash<BuildHasherJump> {
    /// Create new `JumpConsistentHash` with the `DefaultHasher` builder.
    /// ```rust
    /// use jump_consistent_hash::JumpConsistentHash;
    ///
    /// let buckets = 5;
    /// let bucket_balancer = JumpConsistentHash::new(buckets);
    /// ```
    pub fn new(buckets: i32) -> Self {
        Self::new_with_hash_builder(buckets, BuildHasherJump::default())
    }
}

impl<H> JumpConsistentHash<H> {
    /// Create new `JumpConsistentHash` with the custom `Hasher` builder.
    /// ```rust
    /// use std::collections::hash_map::RandomState;
    /// use jump_consistent_hash::JumpConsistentHash;
    ///
    /// let buckets = 5;
    /// let bucket_balancer = JumpConsistentHash::new_with_hash_builder(buckets, RandomState::new());
    /// ```
    pub fn new_with_hash_builder(buckets: i32, builder: H) -> Self {
        assert!(buckets > 0);

        JumpConsistentHash {
            buckets,
            hash_builder: builder,
        }
    }
}

impl From<i32> for JumpConsistentHash<BuildHasherJump> {
    fn from(buckets: i32) -> Self {
        Self::new(buckets)
    }
}

#[cfg(test)]
mod tests {
    use crate::JumpConsistentHash;
    use std::collections::hash_map::RandomState;
    use std::collections::BTreeSet;

    const BUCKETS: i32 = 4;

    #[test]
    fn get_nodes_default() {
        let buckets = JumpConsistentHash::new(BUCKETS);
        let mut result = BTreeSet::new();
        for key in 0..100 {
            result.insert(buckets.get(&key));
        }

        assert_eq!(
            (0..BUCKETS).collect::<Vec<_>>(),
            result.into_iter().collect::<Vec<_>>()
        )
    }

    #[test]
    fn get_nodes_custom() {
        let hasher: RandomState = RandomState::default();
        let buckets = JumpConsistentHash::new_with_hash_builder(BUCKETS, hasher);
        let mut result = BTreeSet::new();
        for key in 0..100 {
            result.insert(buckets.get(&key));
        }

        assert_eq!(
            (0..BUCKETS).collect::<Vec<_>>(),
            result.into_iter().collect::<Vec<_>>()
        )
    }
}
