use std::hash::Hash;
use std::sync::Arc;

use super::{PersistentHashMap, PersistentHashMapStorage};

impl<K, V> Clone for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        match &self.storage {
            PersistentHashMapStorage::Exclusive(_) => self.operational_clone(),
            PersistentHashMapStorage::ForkShared { base, changes, len } => Self {
                storage: PersistentHashMapStorage::ForkShared {
                    base: Arc::clone(base),
                    changes: changes.clone(),
                    len: *len,
                },
            },
        }
    }
}

impl<K, V> Default for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            storage: PersistentHashMapStorage::Exclusive(iter.into_iter().collect()),
        }
    }
}

impl<K, V> PartialEq for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .all(|(key, value)| other.get(key) == Some(value))
    }
}

impl<K, V> Eq for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Eq,
{
}

impl<K, V> std::fmt::Debug for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}
