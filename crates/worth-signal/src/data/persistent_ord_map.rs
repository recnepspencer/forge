use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

enum PersistentOrdMapStorage<K, V> {
    Exclusive(BTreeMap<K, V>),
    ForkShared {
        base: Arc<BTreeMap<K, V>>,
        changes: im::OrdMap<K, Option<V>>,
        len: usize,
    },
}

/// An ordered map with flat ordinary storage and per-key fork overlays.
pub(crate) struct PersistentOrdMap<K: Clone + Ord, V: Clone> {
    storage: PersistentOrdMapStorage<K, V>,
}

impl<K: Clone + Ord, V: Clone> PersistentOrdMap<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            storage: PersistentOrdMapStorage::Exclusive(BTreeMap::new()),
        }
    }

    pub(crate) fn len(&self) -> usize {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.len(),
            PersistentOrdMapStorage::ForkShared { len, .. } => *len,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.get(key),
            PersistentOrdMapStorage::ForkShared { base, changes, .. } => changes
                .get(key)
                .map_or_else(|| base.get(key), Option::as_ref),
        }
    }

    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.get_mut(key),
            PersistentOrdMapStorage::ForkShared { base, changes, .. } => {
                if !changes.contains_key(key) {
                    let value = base.get(key).cloned()?;
                    changes.insert(key.clone(), Some(value));
                }
                changes.get_mut(key).and_then(Option::as_mut)
            }
        }
    }

    pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.insert(key, value),
            PersistentOrdMapStorage::ForkShared { base, changes, len } => {
                let previous = changes
                    .get(&key)
                    .map_or_else(|| base.get(&key), Option::as_ref)
                    .cloned();
                if previous.is_none() {
                    *len += 1;
                }
                changes.insert(key, Some(value));
                previous
            }
        }
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        match &mut self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.remove(key),
            PersistentOrdMapStorage::ForkShared { base, changes, len } => {
                let previous = changes
                    .get(key)
                    .map_or_else(|| base.get(key), Option::as_ref)
                    .cloned();
                if previous.is_some() {
                    *len -= 1;
                    if base.contains_key(key) {
                        changes.insert(key.clone(), None);
                    } else {
                        changes.remove(key);
                    }
                }
                previous
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.storage = PersistentOrdMapStorage::Exclusive(BTreeMap::new());
    }

    pub(crate) fn iter(&self) -> PersistentOrdMapIter<'_, K, V> {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(values) => {
                PersistentOrdMapIter::Exclusive(values.iter())
            }
            PersistentOrdMapStorage::ForkShared { base, changes, .. } => {
                PersistentOrdMapIter::ForkShared {
                    base: base.iter().peekable(),
                    changes: changes.iter().peekable(),
                }
            }
        }
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(key, _)| key)
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, value)| value)
    }

    pub(crate) fn entry(&mut self, key: K) -> PersistentOrdMapEntry<'_, K, V> {
        PersistentOrdMapEntry { map: self, key }
    }

    pub(crate) fn operational_clone(&self) -> Self {
        self.iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        if let PersistentOrdMapStorage::Exclusive(values) = &mut self.storage {
            let base = Arc::new(std::mem::take(values));
            let len = base.len();
            self.storage = PersistentOrdMapStorage::ForkShared {
                base,
                changes: im::OrdMap::new(),
                len,
            };
        }
        self.fork_storage_identity()
    }

    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        match (&self.storage, &other.storage) {
            (
                PersistentOrdMapStorage::ForkShared {
                    base: left_base,
                    changes: left_changes,
                    ..
                },
                PersistentOrdMapStorage::ForkShared {
                    base: right_base,
                    changes: right_changes,
                    ..
                },
            ) => Arc::ptr_eq(left_base, right_base) && left_changes.ptr_eq(right_changes),
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(_) => self.operational_clone(),
            PersistentOrdMapStorage::ForkShared { base, changes, len } => Self {
                storage: PersistentOrdMapStorage::ForkShared {
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
            PersistentOrdMapStorage::Exclusive(_) => unreachable!("fork converts storage"),
            PersistentOrdMapStorage::ForkShared { base, changes, len } => Self {
                storage: PersistentOrdMapStorage::ForkShared {
                    base: Arc::clone(base),
                    changes: changes.clone(),
                    len: *len,
                },
            },
        }
    }
}

pub(crate) struct PersistentOrdMapEntry<'a, K: Clone + Ord, V: Clone> {
    map: &'a mut PersistentOrdMap<K, V>,
    key: K,
}

impl<'a, K: Clone + Ord, V: Clone> PersistentOrdMapEntry<'a, K, V> {
    pub(crate) fn or_insert(self, value: V) -> &'a mut V {
        self.or_insert_with(|| value)
    }

    pub(crate) fn or_insert_with(self, make: impl FnOnce() -> V) -> &'a mut V {
        if !self.map.contains_key(&self.key) {
            self.map.insert(self.key.clone(), make());
        }
        self.map.get_mut(&self.key).expect("entry must exist")
    }

    pub(crate) fn and_modify(self, modify: impl FnOnce(&mut V)) -> Self {
        if let Some(value) = self.map.get_mut(&self.key) {
            modify(value);
        }
        self
    }
}

impl<'a, K: Clone + Ord, V: Clone + Default> PersistentOrdMapEntry<'a, K, V> {
    pub(crate) fn or_default(self) -> &'a mut V {
        self.or_insert_with(V::default)
    }
}

pub(crate) enum PersistentOrdMapIter<'a, K: Clone + Ord, V: Clone> {
    Exclusive(std::collections::btree_map::Iter<'a, K, V>),
    ForkShared {
        base: std::iter::Peekable<std::collections::btree_map::Iter<'a, K, V>>,
        changes: std::iter::Peekable<im::ordmap::Iter<'a, K, Option<V>>>,
    },
}

impl<'a, K: Clone + Ord, V: Clone> Iterator for PersistentOrdMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let Self::ForkShared { base, changes } = self else {
            return match self {
                Self::Exclusive(values) => values.next(),
                Self::ForkShared { .. } => unreachable!(),
            };
        };
        loop {
            match (base.peek(), changes.peek()) {
                (Some((base_key, _)), Some((change_key, _))) if base_key < change_key => {
                    return base.next();
                }
                (Some((base_key, _)), Some((change_key, _))) if base_key == change_key => {
                    base.next();
                    let (key, value) = changes.next().expect("peeked change exists");
                    if let Some(value) = value.as_ref() {
                        return Some((key, value));
                    }
                }
                (_, Some(_)) => {
                    let (key, value) = changes.next().expect("peeked change exists");
                    if let Some(value) = value.as_ref() {
                        return Some((key, value));
                    }
                }
                (Some(_), None) => return base.next(),
                (None, None) => return None,
            }
        }
    }
}

impl<K: Clone + Ord, V: Clone> Clone for PersistentOrdMap<K, V> {
    fn clone(&self) -> Self {
        self.operational_clone()
    }
}

impl<K: Clone + Ord, V: Clone> Default for PersistentOrdMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Clone + Ord, V: Clone> FromIterator<(K, V)> for PersistentOrdMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            storage: PersistentOrdMapStorage::Exclusive(iter.into_iter().collect()),
        }
    }
}

impl<K: Clone + Ord + fmt::Debug, V: Clone + fmt::Debug> fmt::Debug for PersistentOrdMap<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Clone + Ord + PartialEq, V: Clone + PartialEq> PartialEq for PersistentOrdMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<K: Clone + Ord + Eq, V: Clone + Eq> Eq for PersistentOrdMap<K, V> {}

impl<K, V> Serialize for PersistentOrdMap<K, V>
where
    K: Clone + Ord + Serialize,
    V: Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

impl<'de, K, V> Deserialize<'de> for PersistentOrdMap<K, V>
where
    K: Clone + Ord + Deserialize<'de>,
    V: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            storage: PersistentOrdMapStorage::Exclusive(BTreeMap::deserialize(deserializer)?),
        })
    }
}

impl<'a, K: Clone + Ord, V: Clone> IntoIterator for &'a PersistentOrdMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = PersistentOrdMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{PersistentOrdMap, PersistentOrdMapStorage};

    #[test]
    fn overlay_only_insert_remove_churn_retains_no_historical_tombstones() {
        let mut source = PersistentOrdMap::<u64, u64>::new();
        source.insert(0, 7);
        let mut fork = source.fork_persistent();

        for key in 1..=65_536 {
            assert_eq!(fork.get_mut(&key), None);
            assert_eq!(fork.insert(key, key), None);
            assert_eq!(fork.remove(&key), Some(key));
            assert_eq!(fork.get_mut(&key), None);
        }

        assert_eq!(fork.len(), 1);
        let PersistentOrdMapStorage::ForkShared { changes, .. } = &fork.storage else {
            panic!("fork must retain shared storage");
        };
        assert!(
            changes.is_empty(),
            "overlay-only churn must erase its delta"
        );
        assert_eq!(source.get(&0), Some(&7));
    }
}
