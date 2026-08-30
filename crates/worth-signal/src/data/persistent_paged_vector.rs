use std::ops::{Index, IndexMut};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

const PAGE_LEN: usize = 64;

/// A persistent sequence with explicit bounded copy-on-write pages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PersistentPagedVector<T: Clone> {
    pages: crate::data::persistent_vector::PersistentVector<Arc<Vec<T>>>,
    len: usize,
}

impl<T: Clone> PersistentPagedVector<T> {
    pub(crate) fn new() -> Self {
        Self {
            pages: crate::data::persistent_vector::PersistentVector::new(),
            len: 0,
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| &self.pages[index / PAGE_LEN][index % PAGE_LEN])
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        let page = self.pages.get_mut(index / PAGE_LEN)?;
        Arc::make_mut(page).get_mut(index % PAGE_LEN)
    }

    pub(crate) fn last(&self) -> Option<&T> {
        self.len.checked_sub(1).and_then(|index| self.get(index))
    }

    pub(crate) fn push_back(&mut self, value: T) {
        let needs_page = self.len % PAGE_LEN == 0;
        if needs_page {
            self.pages.push_back(Arc::new(vec![value]));
        } else {
            Arc::make_mut(
                self.pages
                    .get_mut(self.len / PAGE_LEN)
                    .expect("nonempty tail page must exist"),
            )
            .push(value);
        }
        self.len += 1;
    }

    pub(crate) fn pop_back(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        let page_index = index / PAGE_LEN;
        let page = self.pages.get_mut(page_index)?;
        let value = Arc::make_mut(page).pop();
        if page.is_empty() {
            self.pages.pop_back();
        }
        self.len = index;
        value
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.pages.iter().flat_map(|page| page.iter())
    }

    pub(crate) fn operational_clone(&self) -> Self {
        self.iter().cloned().collect()
    }

    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.pages.shares_storage_with(&other.pages)
    }

    #[cfg(test)]
    pub(crate) fn page_identities(&self) -> Vec<usize> {
        self.pages
            .iter()
            .map(|page| Arc::as_ptr(page) as usize)
            .collect()
    }
}

impl<T: Clone> Default for PersistentPagedVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> FromIterator<T> for PersistentPagedVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vector = Self::new();
        for value in iter {
            vector.push_back(value);
        }
        vector
    }
}

impl<T: Clone> Index<usize> for PersistentPagedVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("persistent page index in bounds")
    }
}

impl<T: Clone> IndexMut<usize> for PersistentPagedVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("persistent page index in bounds")
    }
}

impl<T> Serialize for PersistentPagedVector<T>
where
    T: Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de, T> Deserialize<'de> for PersistentPagedVector<T>
where
    T: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::<T>::deserialize(deserializer).map(|values| values.into_iter().collect())
    }
}
