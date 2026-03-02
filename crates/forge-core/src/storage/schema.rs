//! Schema definitions for transactional property storage.
//!
//! `PropertyLayer<K, V>` — resting-state storage.
//! `PropertyPatch<K, V>` — transactional overlay with copy-on-write mutation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// ── PropertyLayer (resting state) ────────────────────────────────────────

/// Immutable (resting) storage for a single property.
///
/// Maps typed keys directly to values. This is the serialized form that
/// persists between transactions.
///
/// # Key Requirements
/// Keys must be `Eq + Hash + Copy` (topology handles satisfy this).
///
/// # Value Requirements
/// Values should be `Clone` if you intend to use `PropertyPatch::get_mut()`.
/// Values must be `Serialize + Deserialize` if the layer will be persisted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyLayer<K: Eq + Hash, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash + Copy, V> PropertyLayer<K, V> {
    /// Create an empty layer.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Get a value by key.
    pub fn get(&self, key: K) -> Option<&V> {
        self.data.get(&key)
    }

    /// Set a value by key.
    pub fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    /// Remove a value by key. Returns the removed value if present.
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.data.remove(&key)
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the layer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Whether the layer contains a binding for the given key.
    pub fn contains(&self, key: K) -> bool {
        self.data.contains_key(&key)
    }

    /// Iterate over all values.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }

    /// Iterate over all values mutably.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.data.values_mut()
    }

    /// Iterate over all (key, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }

    /// Iterate over all keys.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }
}

impl<K: Eq + Hash + Copy, V> Default for PropertyLayer<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── PropertyPatch (transactional overlay) ────────────────────────────────

/// Transactional overlay for a `PropertyLayer`.
///
/// Accumulates inserts and removes. On `commit()`, flushes mutations
/// into the base layer. On `drop` (without commit), mutations are
/// silently discarded (rollback).
pub struct PropertyPatch<K: Eq + Hash, V> {
    base: PropertyLayer<K, V>,
    inserts: HashMap<K, V>,
    removes: HashSet<K>,
}

impl<K: Eq + Hash + Copy, V> PropertyPatch<K, V> {
    /// Create a new patch overlaying an existing layer.
    pub fn new(base: PropertyLayer<K, V>) -> Self {
        Self {
            base,
            inserts: HashMap::new(),
            removes: HashSet::new(),
        }
    }

    /// Get a value — checks patch first, then falls through to base.
    pub fn get(&self, key: K) -> Option<&V> {
        if self.removes.contains(&key) {
            return None;
        }
        if let Some(v) = self.inserts.get(&key) {
            return Some(v);
        }
        self.base.get(key)
    }

    /// Set a value in the patch.
    pub fn set(&mut self, key: K, value: V) {
        self.removes.remove(&key);
        self.inserts.insert(key, value);
    }

    /// Remove a value in the patch.
    pub fn remove(&mut self, key: K) {
        self.inserts.remove(&key);
        self.removes.insert(key);
    }

    /// Commit all mutations into the base layer. Consumes the patch.
    pub fn commit(mut self) -> PropertyLayer<K, V> {
        for (key, value) in self.inserts {
            self.base.data.insert(key, value);
        }
        for key in self.removes {
            self.base.data.remove(&key);
        }
        self.base
    }

    /// Discard all mutations and return the original base layer.
    pub fn rollback(self) -> PropertyLayer<K, V> {
        self.base
    }

    /// Read-only access to the underlying base layer.
    pub fn base(&self) -> &PropertyLayer<K, V> {
        &self.base
    }

    /// Iterate over all values visible from the patch.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        let base_vals = self.base.data.iter().filter_map(move |(k, v)| {
            if self.removes.contains(k) || self.inserts.contains_key(k) {
                None
            } else {
                Some(v)
            }
        });
        let insert_vals = self.inserts.values();
        base_vals.chain(insert_vals)
    }

    /// Iterate over all (key, value) pairs visible from the patch.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let base_entries = self.base.data.iter().filter(move |(k, _)| {
            !self.removes.contains(k) && !self.inserts.contains_key(k)
        });
        let insert_entries = self.inserts.iter();
        base_entries.chain(insert_entries)
    }

    /// Iterate over patch inserts mutably.
    ///
    /// Only values explicitly `set()` in this patch are mutable. Base values
    /// are immutable — to mutate them, use `get_mut()` which auto-promotes
    /// via copy-on-write.
    pub fn inserts_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.inserts.values_mut()
    }

    /// Number of entries visible from the patch.
    pub fn len(&self) -> usize {
        let base_count = self
            .base
            .data
            .keys()
            .filter(|k| !self.removes.contains(k) && !self.inserts.contains_key(k))
            .count();
        base_count + self.inserts.len()
    }

    /// Whether the patch view is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the patch view contains a binding for the given key.
    pub fn contains(&self, key: K) -> bool {
        if self.removes.contains(&key) {
            return false;
        }
        self.inserts.contains_key(&key) || self.base.contains(key)
    }
}

/// Copy-on-write mutation — requires `V: Clone`.
impl<K: Eq + Hash + Copy, V: Clone> PropertyPatch<K, V> {
    /// Get a mutable reference, auto-promoting from base on first access.
    ///
    /// If the key exists in the base but hasn't been modified in the patch,
    /// the value is cloned into the patch insert map (copy-on-write).
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if self.removes.contains(&key) {
            return None;
        }
        if self.inserts.contains_key(&key) {
            return self.inserts.get_mut(&key);
        }
        if let Some(val) = self.base.data.get(&key) {
            self.inserts.insert(key, val.clone());
            return self.inserts.get_mut(&key);
        }
        None
    }
}
