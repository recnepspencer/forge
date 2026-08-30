use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

enum PersistentHashMapStorage<K, V> {
    Exclusive(HashMap<K, V>),
    ForkShared {
        base: Arc<HashMap<K, V>>,
        changes: im::HashMap<K, Option<V>>,
        len: usize,
    },
}

/// A hash map with flat ordinary storage and per-key fork overlays.
pub(crate) struct PersistentHashMap<K, V> {
    storage: PersistentHashMapStorage<K, V>,
}

impl<K, V> PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            storage: PersistentHashMapStorage::Exclusive(HashMap::new()),
        }
    }

    pub(crate) fn len(&self) -> usize {
        match &self.storage {
            PersistentHashMapStorage::Exclusive(values) => values.len(),
            PersistentHashMapStorage::ForkShared { len, .. } => *len,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        match &self.storage {
            PersistentHashMapStorage::Exclusive(values) => values.get(key),
            PersistentHashMapStorage::ForkShared { base, changes, .. } => changes
                .get(key)
                .map_or_else(|| base.get(key), Option::as_ref),
        }
    }

    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.storage {
            PersistentHashMapStorage::Exclusive(values) => values.get_mut(key),
            PersistentHashMapStorage::ForkShared { base, changes, .. } => {
                if !changes.contains_key(key) {
                    let value = base.get(key).cloned()?;
                    changes.insert(key.clone(), Some(value));
                }
                changes.get_mut(key).and_then(Option::as_mut)
            }
        }
    }

    #[inline]
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.storage {
            PersistentHashMapStorage::Exclusive(values) => values.insert(key, value),
            PersistentHashMapStorage::ForkShared { base, changes, len } => {
                let prior = changes
                    .get(&key)
                    .map_or_else(|| base.get(&key), Option::as_ref)
                    .cloned();
                if prior.is_none() {
                    *len += 1;
                }
                changes.insert(key, Some(value));
                prior
            }
        }
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        match &mut self.storage {
            PersistentHashMapStorage::Exclusive(values) => values.remove(key),
            PersistentHashMapStorage::ForkShared { base, changes, len } => {
                let prior = changes
                    .get(key)
                    .map_or_else(|| base.get(key), Option::as_ref)
                    .cloned();
                if prior.is_some() {
                    *len -= 1;
                    if base.contains_key(key) {
                        changes.insert(key.clone(), None);
                    } else {
                        changes.remove(key);
                    }
                }
                prior
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.storage = PersistentHashMapStorage::Exclusive(HashMap::new());
    }

    pub(crate) fn entry(&mut self, key: K) -> PersistentHashMapEntry<'_, K, V> {
        PersistentHashMapEntry { map: self, key }
    }

    pub(crate) fn iter(&self) -> Box<dyn Iterator<Item = (&K, &V)> + '_> {
        match &self.storage {
            PersistentHashMapStorage::Exclusive(values) => Box::new(values.iter()),
            PersistentHashMapStorage::ForkShared { base, changes, .. } => {
                let unchanged = base.iter().filter(|(key, _)| !changes.contains_key(*key));
                let changed = changes
                    .iter()
                    .filter_map(|(key, value)| value.as_ref().map(|value| (key, value)));
                Box::new(unchanged.chain(changed))
            }
        }
    }

    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        match (&self.storage, &other.storage) {
            (
                PersistentHashMapStorage::ForkShared {
                    base: left_base,
                    changes: left_changes,
                    ..
                },
                PersistentHashMapStorage::ForkShared {
                    base: right_base,
                    changes: right_changes,
                    ..
                },
            ) => Arc::ptr_eq(left_base, right_base) && left_changes.ptr_eq(right_changes),
            _ => false,
        }
    }

    pub(crate) fn operational_clone(&self) -> Self {
        self.iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        if let PersistentHashMapStorage::Exclusive(values) = &mut self.storage {
            let base = Arc::new(std::mem::take(values));
            let len = base.len();
            self.storage = PersistentHashMapStorage::ForkShared {
                base,
                changes: im::HashMap::new(),
                len,
            };
        }
        self.fork_storage_identity()
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
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

    #[cfg(not(test))]
    fn fork_storage_identity(&self) -> Self {
        match &self.storage {
            PersistentHashMapStorage::Exclusive(_) => unreachable!("fork converts storage"),
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

pub(crate) struct PersistentHashMapEntry<'a, K, V> {
    map: &'a mut PersistentHashMap<K, V>,
    key: K,
}

impl<'a, K, V> PersistentHashMapEntry<'a, K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    pub(crate) fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        if self.map.get(&self.key).is_none() {
            self.map.insert(self.key.clone(), V::default());
        }
        self.map.get_mut(&self.key).expect("entry must exist")
    }
}

impl<K, V> Clone for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        self.operational_clone()
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

impl<K, V> Serialize for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash + Serialize,
    V: Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

impl<'de, K, V> Deserialize<'de> for PersistentHashMap<K, V>
where
    K: Clone + Eq + Hash + Deserialize<'de>,
    V: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HashMap::<K, V>::deserialize(deserializer)?
            .into_iter()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{PersistentHashMap, PersistentHashMapStorage};

    #[test]
    fn overlay_only_insert_remove_churn_retains_no_historical_tombstones() {
        let mut source = PersistentHashMap::<u64, u64>::new();
        source.insert(0, 7);
        let mut fork = source.fork_persistent();

        for key in 1..=65_536 {
            assert_eq!(fork.get_mut(&key), None);
            assert_eq!(fork.insert(key, key), None);
            assert_eq!(fork.remove(&key), Some(key));
            assert_eq!(fork.get_mut(&key), None);
        }

        assert_eq!(fork.len(), 1);
        let PersistentHashMapStorage::ForkShared { changes, .. } = &fork.storage else {
            panic!("fork must retain shared storage");
        };
        assert!(
            changes.is_empty(),
            "overlay-only churn must erase its delta"
        );
        assert_eq!(source.get(&0), Some(&7));
    }
}
