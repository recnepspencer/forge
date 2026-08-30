use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::data::persistent_ord_map::PersistentOrdMap;

/// An ordered set with flat unique storage and element-granular fork changes.
#[derive(PartialEq, Eq)]
pub(crate) struct PersistentOrdSet<T: Clone + Ord> {
    values: PersistentOrdMap<T, ()>,
}

impl<T: Clone + Ord> PersistentOrdSet<T> {
    pub(crate) fn new() -> Self {
        Self {
            values: PersistentOrdMap::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub(crate) fn contains<Q>(&self, value: &Q) -> bool
    where
        T: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.contains_key(value)
    }

    pub(crate) fn insert(&mut self, value: T) -> bool {
        self.values.insert(value, ()).is_none()
    }

    pub(crate) fn remove(&mut self, value: &T) -> bool {
        self.values.remove(value).is_some()
    }

    pub(crate) fn clear(&mut self) {
        self.values.clear();
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.keys()
    }

    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        self.values.ptr_eq(&other.values)
    }

    pub(crate) fn operational_clone(&self) -> Self {
        self.iter().cloned().collect()
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        Self {
            values: self.values.fork_persistent(),
        }
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        Self {
            values: self.values.fork_storage_identity(),
        }
    }
}

impl<T: Clone + Ord> Clone for PersistentOrdSet<T> {
    fn clone(&self) -> Self {
        self.operational_clone()
    }
}

impl<T: Clone + Ord> FromIterator<T> for PersistentOrdSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            values: iter.into_iter().map(|value| (value, ())).collect(),
        }
    }
}

impl<T: Clone + Ord> Default for PersistentOrdSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Ord> Extend<T> for PersistentOrdSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<T> std::fmt::Debug for PersistentOrdSet<T>
where
    T: Clone + Ord + std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_set().entries(self.iter()).finish()
    }
}

impl<T> Serialize for PersistentOrdSet<T>
where
    T: Clone + Ord + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.iter().collect::<Vec<_>>().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for PersistentOrdSet<T>
where
    T: Clone + Ord + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<T>::deserialize(deserializer)?.into_iter().collect())
    }
}

impl<'a, T: Clone + Ord> IntoIterator for &'a PersistentOrdSet<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl<T: Clone + Ord> IntoIterator for PersistentOrdSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().cloned().collect::<Vec<_>>().into_iter()
    }
}
