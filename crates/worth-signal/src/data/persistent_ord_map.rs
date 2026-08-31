use std::collections::BTreeMap;
use std::sync::Arc;

#[path = "persistent_ord_map/fork_overlay.rs"]
mod fork_overlay;
#[path = "persistent_ord_map/iteration.rs"]
mod iteration;
#[path = "persistent_ord_map/serialization.rs"]
mod serialization;
#[path = "persistent_ord_map/traits.rs"]
mod traits;

use fork_overlay::{
    base_value_if_live, next_live_key_after, record_base_readmission, record_base_retirement,
};
use iteration::{LiveBaseIter, PersistentOrdMapIter};

enum PersistentOrdMapStorage<K, V> {
    Exclusive(BTreeMap<K, V>),
    ForkShared {
        base: Arc<BTreeMap<K, V>>,
        changes: im::OrdMap<K, V>,
        live_change_keys: im::OrdSet<K>,
        retired_base_intervals: im::OrdMap<K, K>,
        len: usize,
        first_live_key: Option<K>,
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
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                retired_base_intervals,
                ..
            } => changes
                .get(key)
                .or_else(|| base_value_if_live(base, retired_base_intervals, key)),
        }
    }

    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.get_mut(key),
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                live_change_keys,
                retired_base_intervals,
                ..
            } => {
                if !changes.contains_key(key) {
                    let value = base_value_if_live(base, retired_base_intervals, key)?.clone();
                    changes.insert(key.clone(), value);
                    live_change_keys.insert(key.clone());
                }
                changes.get_mut(key)
            }
        }
    }

    pub(crate) fn first_key_value(&self) -> Option<(&K, &V)> {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.first_key_value(),
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                first_live_key,
                retired_base_intervals,
                ..
            } => {
                let key = first_live_key.as_ref()?;
                let value = changes
                    .get(key)
                    .or_else(|| base_value_if_live(base, retired_base_intervals, key))
                    .expect("first-live key must address a live value");
                Some((key, value))
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
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                live_change_keys,
                retired_base_intervals,
                len,
                first_live_key,
            } => {
                let previous = changes
                    .get(&key)
                    .or_else(|| base_value_if_live(base, retired_base_intervals, &key))
                    .cloned();
                if previous.is_none() {
                    *len += 1;
                }
                if first_live_key
                    .as_ref()
                    .is_none_or(|first_key| key < *first_key)
                {
                    *first_live_key = Some(key.clone());
                }
                if base.contains_key(&key) && previous.is_none() {
                    record_base_readmission(base, retired_base_intervals, &key);
                }
                live_change_keys.insert(key.clone());
                changes.insert(key, value);
                previous
            }
        }
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        match &mut self.storage {
            PersistentOrdMapStorage::Exclusive(values) => values.remove(key),
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                live_change_keys,
                retired_base_intervals,
                len,
                first_live_key,
            } => {
                let previous = changes
                    .get(key)
                    .or_else(|| base_value_if_live(base, retired_base_intervals, key))
                    .cloned();
                if previous.is_some() {
                    let removed_first = first_live_key.as_ref() == Some(key);
                    *len -= 1;
                    live_change_keys.remove(key);
                    changes.remove(key);
                    if base.contains_key(key) {
                        record_base_retirement(base, retired_base_intervals, key);
                    }
                    if removed_first {
                        *first_live_key = next_live_key_after(
                            base,
                            live_change_keys,
                            retired_base_intervals,
                            key,
                        );
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
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                first_live_key,
                retired_base_intervals,
                ..
            } => {
                let Some(first_live_key) = first_live_key.as_ref() else {
                    return PersistentOrdMapIter::Empty;
                };
                PersistentOrdMapIter::ForkShared {
                    base: LiveBaseIter::new(base, retired_base_intervals, first_live_key)
                        .peekable(),
                    changes: changes
                        .range((
                            std::ops::Bound::Included(first_live_key),
                            std::ops::Bound::Unbounded,
                        ))
                        .peekable(),
                    remaining: self.len(),
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
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(values) => Self {
                storage: PersistentOrdMapStorage::Exclusive(values.clone()),
            },
            PersistentOrdMapStorage::ForkShared { .. } => self
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        }
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        if let PersistentOrdMapStorage::Exclusive(values) = &mut self.storage {
            let base = Arc::new(std::mem::take(values));
            let len = base.len();
            let first_live_key = base.first_key_value().map(|(key, _)| key.clone());
            self.storage = PersistentOrdMapStorage::ForkShared {
                base,
                changes: im::OrdMap::new(),
                live_change_keys: im::OrdSet::new(),
                retired_base_intervals: im::OrdMap::new(),
                len,
                first_live_key,
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
                    live_change_keys: left_live_change_keys,
                    retired_base_intervals: left_retired_base_intervals,
                    ..
                },
                PersistentOrdMapStorage::ForkShared {
                    base: right_base,
                    changes: right_changes,
                    live_change_keys: right_live_change_keys,
                    retired_base_intervals: right_retired_base_intervals,
                    ..
                },
            ) => {
                Arc::ptr_eq(left_base, right_base)
                    && left_changes.ptr_eq(right_changes)
                    && left_live_change_keys.ptr_eq(right_live_change_keys)
                    && left_retired_base_intervals.ptr_eq(right_retired_base_intervals)
            }
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(_) => self.operational_clone(),
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                live_change_keys,
                retired_base_intervals,
                len,
                first_live_key,
            } => Self {
                storage: PersistentOrdMapStorage::ForkShared {
                    base: Arc::clone(base),
                    changes: changes.clone(),
                    live_change_keys: live_change_keys.clone(),
                    retired_base_intervals: retired_base_intervals.clone(),
                    len: *len,
                    first_live_key: first_live_key.clone(),
                },
            },
        }
    }

    #[cfg(not(test))]
    fn fork_storage_identity(&self) -> Self {
        match &self.storage {
            PersistentOrdMapStorage::Exclusive(_) => unreachable!("fork converts storage"),
            PersistentOrdMapStorage::ForkShared {
                base,
                changes,
                live_change_keys,
                retired_base_intervals,
                len,
                first_live_key,
            } => Self {
                storage: PersistentOrdMapStorage::ForkShared {
                    base: Arc::clone(base),
                    changes: changes.clone(),
                    live_change_keys: live_change_keys.clone(),
                    retired_base_intervals: retired_base_intervals.clone(),
                    len: *len,
                    first_live_key: first_live_key.clone(),
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

#[cfg(test)]
mod tests;
