//! Dense bitset over entity indices for O(1) membership tracking.
//!
//! DOMAIN: Reusable visited-set for BFS traversals, validation sweeps,
//! and shell discovery. Replaces `BTreeSet<u32>` with cache-friendly
//! bit-per-entity storage.
//!
//! INVARIANTS:
//! - All indices in [0, capacity) are valid; out-of-range returns `Err`
//! - `count()` is maintained incrementally (O(1) query)
//! - `clear()` resets without deallocation (O(words))
//! - Iteration via `iter_ones()` produces indices in ascending order
//!
//! DEPENDENCIES: `forge_core::KernelError`

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

/// A dense bitset over entity indices in range [0, capacity).
///
/// Designed for reuse across multiple BFS/validation phases:
/// `clear()` resets in O(words) without deallocation.
///
/// Academic basis: Dense bitvector for bounded-universe membership
/// (Lemire et al. 2016, "Roaring Bitmaps"; CGAL mark mechanisms).
#[derive(Clone, Serialize, Deserialize)]
pub struct EntityBitset {
    words: Vec<u64>,
    len: u32,
    capacity: u32,
}

impl EntityBitset {
    /// Create a bitset that can hold indices in [0, capacity).
    pub fn with_capacity(capacity: u32) -> Self {
        let word_count = word_count_for(capacity);
        Self {
            words: vec![0u64; word_count],
            len: 0,
            capacity,
        }
    }

    /// Create a bitset sized for faces in the given arena.
    pub fn for_faces(arena: &crate::arena::TopologyArena) -> Self {
        let max_index = arena.iter_faces()
            .map(|(id, _)| id.index())
            .max()
            .map(|m| m + 1)
            .unwrap_or(0);
        Self::with_capacity(max_index)
    }

    /// Create a bitset sized for half-edges in the given arena.
    pub fn for_half_edges(arena: &crate::arena::TopologyArena) -> Self {
        let max_index = arena.iter_half_edges()
            .map(|(id, _)| id.index())
            .max()
            .map(|m| m + 1)
            .unwrap_or(0);
        Self::with_capacity(max_index)
    }

    /// Create a bitset sized for vertices in the given arena.
    pub fn for_vertices(arena: &crate::arena::TopologyArena) -> Self {
        let max_index = arena.iter_vertices()
            .map(|(id, _)| id.index())
            .max()
            .map(|m| m + 1)
            .unwrap_or(0);
        Self::with_capacity(max_index)
    }

    /// Create a bitset sized for edges in the given arena.
    pub fn for_edges(arena: &crate::arena::TopologyArena) -> Self {
        let max_index = arena.iter_edges()
            .map(|(id, _)| id.index())
            .max()
            .map(|m| m + 1)
            .unwrap_or(0);
        Self::with_capacity(max_index)
    }

    /// Reset all bits to 0 without deallocating.
    pub fn clear(&mut self) {
        for word in self.words.iter_mut() {
            *word = 0;
        }
        self.len = 0;
    }

    /// Insert an index. Returns `true` if it was newly inserted.
    pub fn insert(&mut self, index: u32) -> Result<bool, KernelError> {
        self.check_bounds(index)?;
        let (word_idx, bit_idx) = split_index(index);
        let mask = 1u64 << bit_idx;
        let was_set = self.words[word_idx] & mask != 0;
        self.words[word_idx] |= mask;
        if !was_set {
            self.len += 1;
        }
        Ok(!was_set)
    }

    /// Check membership.
    pub fn contains(&self, index: u32) -> Result<bool, KernelError> {
        self.check_bounds(index)?;
        let (word_idx, bit_idx) = split_index(index);
        Ok(self.words[word_idx] & (1u64 << bit_idx) != 0)
    }

    /// Remove an index. Returns `true` if it was present.
    pub fn remove(&mut self, index: u32) -> Result<bool, KernelError> {
        self.check_bounds(index)?;
        let (word_idx, bit_idx) = split_index(index);
        let mask = 1u64 << bit_idx;
        let was_set = self.words[word_idx] & mask != 0;
        self.words[word_idx] &= !mask;
        if was_set {
            self.len -= 1;
        }
        Ok(was_set)
    }

    /// Number of set bits. O(1).
    pub fn count(&self) -> u32 {
        self.len
    }

    /// Whether the set is empty. O(1).
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The maximum index this bitset can hold (exclusive).
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Grow capacity to accommodate a new maximum index.
    /// Existing bits are preserved.
    pub fn grow(&mut self, new_capacity: u32) {
        if new_capacity <= self.capacity {
            return;
        }
        let new_word_count = word_count_for(new_capacity);
        self.words.resize(new_word_count, 0u64);
        self.capacity = new_capacity;
    }

    /// Iterate over all set bit indices in ascending order.
    pub fn iter_ones(&self) -> BitsetIterator<'_> {
        let current_word = if self.words.is_empty() { 0 } else { self.words[0] };
        BitsetIterator {
            words: &self.words,
            word_idx: 0,
            current_word,
            base: 0,
        }
    }

    /// In-place union: self |= other.
    ///
    /// If `other` has greater capacity, self is grown to match.
    pub fn union_with(&mut self, other: &EntityBitset) {
        if other.capacity > self.capacity {
            self.grow(other.capacity);
        }
        for (i, &other_word) in other.words.iter().enumerate() {
            if i < self.words.len() {
                self.words[i] |= other_word;
            }
        }
        self.recount();
    }

    /// In-place intersection: self &= other.
    ///
    /// Bits beyond `other`'s capacity are cleared.
    pub fn intersect_with(&mut self, other: &EntityBitset) {
        for (i, word) in self.words.iter_mut().enumerate() {
            if i < other.words.len() {
                *word &= other.words[i];
            } else {
                *word = 0;
            }
        }
        self.recount();
    }

    /// In-place difference: self &= !other.
    pub fn difference_with(&mut self, other: &EntityBitset) {
        let limit = self.words.len().min(other.words.len());
        for i in 0..limit {
            self.words[i] &= !other.words[i];
        }
        self.recount();
    }

    /// Recompute `len` from scratch via popcount.
    fn recount(&mut self) {
        self.len = self.words.iter().map(|w| w.count_ones()).sum();
    }

    /// Bounds check returning KernelError on violation.
    fn check_bounds(&self, index: u32) -> Result<(), KernelError> {
        if index >= self.capacity {
            return Err(KernelError::InternalError {
                message: format!(
                    "EntityBitset index {} out of range [0, {})",
                    index, self.capacity
                ),
                context: None,
            });
        }
        Ok(())
    }
}

impl std::fmt::Debug for EntityBitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityBitset")
            .field("capacity", &self.capacity)
            .field("count", &self.len)
            .finish()
    }
}

/// Ascending iterator over set bits in an `EntityBitset`.
///
/// Extracts indices using `trailing_zeros` on each non-zero word,
/// skipping empty words in O(1).
pub struct BitsetIterator<'a> {
    words: &'a [u64],
    word_idx: usize,
    current_word: u64,
    base: u32,
}

impl<'a> Iterator for BitsetIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            if self.current_word != 0 {
                let bit = self.current_word.trailing_zeros();
                self.current_word &= self.current_word - 1;
                return Some(self.base + bit);
            }
            self.word_idx += 1;
            if self.word_idx >= self.words.len() {
                return None;
            }
            self.current_word = self.words[self.word_idx];
            self.base = (self.word_idx as u32) * 64;
        }
    }
}

/// Compute number of u64 words needed for `capacity` bits.
fn word_count_for(capacity: u32) -> usize {
    ((capacity as usize) + 63) / 64
}

/// Split index into (word_index, bit_index_within_word).
fn split_index(index: u32) -> (usize, u32) {
    ((index / 64) as usize, index % 64)
}


#[cfg(test)]
mod tests {
    use super::*;

    // ── Unit Tests ────────────────────────────────────────────────

    #[test]
    fn empty_bitset_has_zero_count() {
        let bs = EntityBitset::with_capacity(100);
        assert_eq!(bs.count(), 0);
        assert!(bs.is_empty());
        assert_eq!(bs.capacity(), 100);
    }

    #[test]
    fn insert_and_contains() {
        let mut bs = EntityBitset::with_capacity(256);
        assert!(bs.insert(0).unwrap());
        assert!(bs.insert(42).unwrap());
        assert!(bs.insert(255).unwrap());
        assert!(bs.contains(0).unwrap());
        assert!(bs.contains(42).unwrap());
        assert!(bs.contains(255).unwrap());
        assert!(!bs.contains(1).unwrap());
        assert!(!bs.contains(100).unwrap());
        assert_eq!(bs.count(), 3);
    }

    #[test]
    fn insert_returns_false_on_duplicate() {
        let mut bs = EntityBitset::with_capacity(64);
        assert!(bs.insert(10).unwrap());
        assert!(!bs.insert(10).unwrap());
        assert_eq!(bs.count(), 1);
    }

    #[test]
    fn remove_and_count() {
        let mut bs = EntityBitset::with_capacity(64);
        bs.insert(5).unwrap();
        bs.insert(10).unwrap();
        assert_eq!(bs.count(), 2);

        assert!(bs.remove(5).unwrap());
        assert_eq!(bs.count(), 1);
        assert!(!bs.contains(5).unwrap());

        assert!(!bs.remove(5).unwrap());
        assert_eq!(bs.count(), 1);
    }

    #[test]
    fn clear_resets_all_bits() {
        let mut bs = EntityBitset::with_capacity(128);
        for i in 0..128 {
            bs.insert(i).unwrap();
        }
        assert_eq!(bs.count(), 128);
        bs.clear();
        assert_eq!(bs.count(), 0);
        assert!(bs.is_empty());
        assert!(!bs.contains(0).unwrap());
        assert!(!bs.contains(127).unwrap());
    }

    #[test]
    fn out_of_range_returns_error() {
        let bs = EntityBitset::with_capacity(10);
        assert!(bs.contains(10).is_err());
        assert!(bs.contains(100).is_err());

        let mut bs_mut = EntityBitset::with_capacity(10);
        assert!(bs_mut.insert(10).is_err());
        assert!(bs_mut.remove(10).is_err());
    }

    #[test]
    fn grow_preserves_existing_bits() {
        let mut bs = EntityBitset::with_capacity(64);
        bs.insert(0).unwrap();
        bs.insert(63).unwrap();
        assert_eq!(bs.count(), 2);

        bs.grow(256);
        assert_eq!(bs.capacity(), 256);
        assert!(bs.contains(0).unwrap());
        assert!(bs.contains(63).unwrap());
        assert_eq!(bs.count(), 2);

        bs.insert(200).unwrap();
        assert!(bs.contains(200).unwrap());
        assert_eq!(bs.count(), 3);
    }

    #[test]
    fn iter_ones_ascending_order() {
        let mut bs = EntityBitset::with_capacity(300);
        let indices = [250, 0, 128, 63, 64, 65, 1, 199];
        for &i in &indices {
            bs.insert(i).unwrap();
        }
        let result: Vec<u32> = bs.iter_ones().collect();
        let mut expected = indices.to_vec();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn iter_ones_empty() {
        let bs = EntityBitset::with_capacity(100);
        let result: Vec<u32> = bs.iter_ones().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn union_with_combines_sets() {
        let mut a = EntityBitset::with_capacity(128);
        let mut b = EntityBitset::with_capacity(128);
        a.insert(0).unwrap();
        a.insert(10).unwrap();
        b.insert(10).unwrap();
        b.insert(20).unwrap();
        a.union_with(&b);
        assert_eq!(a.count(), 3);
        assert!(a.contains(0).unwrap());
        assert!(a.contains(10).unwrap());
        assert!(a.contains(20).unwrap());
    }

    #[test]
    fn intersect_with_keeps_common() {
        let mut a = EntityBitset::with_capacity(128);
        let mut b = EntityBitset::with_capacity(128);
        a.insert(0).unwrap();
        a.insert(10).unwrap();
        a.insert(20).unwrap();
        b.insert(10).unwrap();
        b.insert(30).unwrap();
        a.intersect_with(&b);
        assert_eq!(a.count(), 1);
        assert!(a.contains(10).unwrap());
        assert!(!a.contains(0).unwrap());
        assert!(!a.contains(20).unwrap());
    }

    #[test]
    fn difference_with_removes_overlap() {
        let mut a = EntityBitset::with_capacity(128);
        let mut b = EntityBitset::with_capacity(128);
        a.insert(0).unwrap();
        a.insert(10).unwrap();
        a.insert(20).unwrap();
        b.insert(10).unwrap();
        b.insert(30).unwrap();
        a.difference_with(&b);
        assert_eq!(a.count(), 2);
        assert!(a.contains(0).unwrap());
        assert!(!a.contains(10).unwrap());
        assert!(a.contains(20).unwrap());
    }

    // ── Adversarial Tests ────────────────────────────────────────

    #[test]
    fn adversarial_capacity_zero() {
        let bs = EntityBitset::with_capacity(0);
        assert_eq!(bs.count(), 0);
        assert!(bs.is_empty());
        assert!(bs.contains(0).is_err());

        let result: Vec<u32> = bs.iter_ones().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn adversarial_capacity_one() {
        let mut bs = EntityBitset::with_capacity(1);
        assert!(bs.insert(0).unwrap());
        assert!(bs.contains(0).unwrap());
        assert!(bs.insert(1).is_err());
        assert_eq!(bs.count(), 1);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn adversarial_word_boundary_63_64_65() {
        let mut bs = EntityBitset::with_capacity(128);
        bs.insert(63).unwrap();
        bs.insert(64).unwrap();
        bs.insert(65).unwrap();
        assert_eq!(bs.count(), 3);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result, vec![63, 64, 65]);

        bs.remove(64).unwrap();
        assert_eq!(bs.count(), 2);
        assert!(!bs.contains(64).unwrap());
        assert!(bs.contains(63).unwrap());
        assert!(bs.contains(65).unwrap());
    }

    #[test]
    fn adversarial_max_capacity() {
        let cap = 100_000u32;
        let mut bs = EntityBitset::with_capacity(cap);
        bs.insert(0).unwrap();
        bs.insert(cap - 1).unwrap();
        bs.insert(50_000).unwrap();
        assert_eq!(bs.count(), 3);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result, vec![0, 50_000, cap - 1]);
    }

    #[test]
    fn adversarial_all_bits_set() {
        let cap = 200u32;
        let mut bs = EntityBitset::with_capacity(cap);
        for i in 0..cap {
            bs.insert(i).unwrap();
        }
        assert_eq!(bs.count(), cap);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result.len(), cap as usize);
        assert_eq!(result[0], 0);
        assert_eq!(result[result.len() - 1], cap - 1);
    }

    #[test]
    fn adversarial_alternating_bits() {
        let cap = 256u32;
        let mut bs = EntityBitset::with_capacity(cap);
        for i in (0..cap).step_by(2) {
            bs.insert(i).unwrap();
        }
        assert_eq!(bs.count(), cap / 2);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result.len(), (cap / 2) as usize);
        for (pos, &val) in result.iter().enumerate() {
            assert_eq!(val, (pos * 2) as u32);
        }
    }

    #[test]
    fn adversarial_grow_across_word_boundary() {
        let mut bs = EntityBitset::with_capacity(60);
        bs.insert(59).unwrap();
        assert_eq!(bs.count(), 1);

        bs.grow(70);
        assert!(bs.contains(59).unwrap());
        bs.insert(64).unwrap();
        bs.insert(69).unwrap();
        assert_eq!(bs.count(), 3);

        let result: Vec<u32> = bs.iter_ones().collect();
        assert_eq!(result, vec![59, 64, 69]);
    }

    #[test]
    fn adversarial_set_operations_mismatched_size() {
        let mut a = EntityBitset::with_capacity(64);
        let mut b = EntityBitset::with_capacity(256);
        a.insert(10).unwrap();
        b.insert(10).unwrap();
        b.insert(200).unwrap();

        a.union_with(&b);
        assert_eq!(a.capacity(), 256);
        assert_eq!(a.count(), 2);
        assert!(a.contains(10).unwrap());
        assert!(a.contains(200).unwrap());

        let mut c = EntityBitset::with_capacity(256);
        let mut d = EntityBitset::with_capacity(64);
        c.insert(10).unwrap();
        c.insert(200).unwrap();
        d.insert(10).unwrap();

        c.intersect_with(&d);
        assert_eq!(c.count(), 1);
        assert!(c.contains(10).unwrap());
        assert!(!c.contains(200).unwrap());

        let mut e = EntityBitset::with_capacity(256);
        let mut f = EntityBitset::with_capacity(64);
        e.insert(10).unwrap();
        e.insert(200).unwrap();
        f.insert(10).unwrap();

        e.difference_with(&f);
        assert_eq!(e.count(), 1);
        assert!(!e.contains(10).unwrap());
        assert!(e.contains(200).unwrap());
    }
}
