//! Method implementations for `PropertyLayer<K, V>`.

use std::collections::HashMap;
use std::hash::Hash;

use crate::storage::data::PropertyLayer;

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
