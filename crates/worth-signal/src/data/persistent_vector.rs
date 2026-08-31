use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

mod iteration;
mod serialization;
#[cfg(test)]
mod tests;

use self::iteration::PersistentVectorIter;

const DEFAULT_PAGE_LEN: usize = 32;

enum PersistentVectorStorage<T> {
    Exclusive(Vec<T>),
    ForkShared {
        base: Arc<Vec<T>>,
        changed_pages: im::OrdMap<usize, Arc<Vec<T>>>,
        len: usize,
    },
}

/// A sequence that keeps the non-forking lane flat and detaches bounded pages
/// only after an exact owner-cell fork.
pub(crate) struct PersistentVector<T: Clone, const PAGE_LEN: usize = DEFAULT_PAGE_LEN> {
    storage: PersistentVectorStorage<T>,
}

impl<T: Clone, const PAGE_LEN: usize> PersistentVector<T, PAGE_LEN> {
    pub(crate) fn new() -> Self {
        debug_assert!(PAGE_LEN != 0);
        Self {
            storage: PersistentVectorStorage::Exclusive(Vec::new()),
        }
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => values.len(),
            PersistentVectorStorage::ForkShared { len, .. } => *len,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn reserve_exclusive(&mut self, additional: usize) {
        if let PersistentVectorStorage::Exclusive(values) = &mut self.storage {
            values.reserve(additional);
        }
    }

    pub(crate) fn exclusive_capacity(&self) -> Option<usize> {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => Some(values.capacity()),
            PersistentVectorStorage::ForkShared { .. } => None,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => values.get(index),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => {
                if index >= *len {
                    return None;
                }
                let page_index = index / PAGE_LEN;
                changed_pages
                    .get(&page_index)
                    .map_or_else(|| base.get(index), |page| page.get(index % PAGE_LEN))
            }
        }
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.storage {
            PersistentVectorStorage::Exclusive(values) => values.get_mut(index),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => {
                if index >= *len {
                    return None;
                }
                let page_index = index / PAGE_LEN;
                install_changed_page::<T, PAGE_LEN>(base, changed_pages, *len, page_index);
                Arc::make_mut(
                    changed_pages
                        .get_mut(&page_index)
                        .expect("changed page must be installed"),
                )
                .get_mut(index % PAGE_LEN)
            }
        }
    }

    pub(crate) fn last(&self) -> Option<&T> {
        self.len().checked_sub(1).and_then(|index| self.get(index))
    }

    #[inline(always)]
    pub(crate) fn push_back(&mut self, value: T) {
        match &mut self.storage {
            PersistentVectorStorage::Exclusive(values) => values.push(value),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => {
                let page_index = *len / PAGE_LEN;
                install_changed_page::<T, PAGE_LEN>(base, changed_pages, *len, page_index);
                Arc::make_mut(
                    changed_pages
                        .get_mut(&page_index)
                        .expect("changed page must be installed"),
                )
                .push(value);
                *len += 1;
            }
        }
    }

    pub(crate) fn pop_back(&mut self) -> Option<T> {
        match &mut self.storage {
            PersistentVectorStorage::Exclusive(values) => values.pop(),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => {
                let index = len.checked_sub(1)?;
                let page_index = index / PAGE_LEN;
                install_changed_page::<T, PAGE_LEN>(base, changed_pages, *len, page_index);
                let page = Arc::make_mut(
                    changed_pages
                        .get_mut(&page_index)
                        .expect("changed page must be installed"),
                );
                let value = page.pop();
                *len = index;
                if page.is_empty() {
                    changed_pages.remove(&page_index);
                }
                value
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.storage = PersistentVectorStorage::Exclusive(Vec::new());
    }

    pub(crate) fn insert(&mut self, index: usize, value: T) {
        self.make_exclusive().insert(index, value);
    }

    pub(crate) fn binary_search_by_key<B, F>(&self, key: &B, mut key_of: F) -> Result<usize, usize>
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let middle = left + (right - left) / 2;
            match key_of(&self[middle]).cmp(key) {
                std::cmp::Ordering::Less => left = middle + 1,
                std::cmp::Ordering::Equal => return Ok(middle),
                std::cmp::Ordering::Greater => right = middle,
            }
        }
        Err(left)
    }

    pub(crate) fn iter(&self) -> PersistentVectorIter<'_, T, PAGE_LEN> {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => {
                PersistentVectorIter::Exclusive(values.iter())
            }
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => PersistentVectorIter::ForkShared {
                base,
                changed_pages,
                len: *len,
                next: 0,
            },
        }
    }

    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.make_exclusive().iter_mut()
    }

    pub(crate) fn extend<I>(&mut self, values: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in values {
            self.push_back(value);
        }
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        if let PersistentVectorStorage::Exclusive(values) = &mut self.storage {
            let base = Arc::new(std::mem::take(values));
            let len = base.len();
            self.storage = PersistentVectorStorage::ForkShared {
                base,
                changed_pages: im::OrdMap::new(),
                len,
            };
        }
        match &self.storage {
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => Self {
                storage: PersistentVectorStorage::ForkShared {
                    base: Arc::clone(base),
                    changed_pages: changed_pages.clone(),
                    len: *len,
                },
            },
            PersistentVectorStorage::Exclusive(_) => unreachable!("fork preparation must share"),
        }
    }

    pub(crate) fn operational_clone(&self) -> Self {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => Self {
                storage: PersistentVectorStorage::Exclusive(values.clone()),
            },
            PersistentVectorStorage::ForkShared { .. } => self.iter().cloned().collect(),
        }
    }

    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        match (&self.storage, &other.storage) {
            (
                PersistentVectorStorage::ForkShared {
                    base: left_base,
                    changed_pages: left_pages,
                    ..
                },
                PersistentVectorStorage::ForkShared {
                    base: right_base,
                    changed_pages: right_pages,
                    ..
                },
            ) => Arc::ptr_eq(left_base, right_base) && left_pages.ptr_eq(right_pages),
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn page_identities(&self) -> Vec<usize> {
        match &self.storage {
            PersistentVectorStorage::Exclusive(values) => values
                .chunks(PAGE_LEN)
                .map(|page| page.as_ptr() as usize)
                .collect(),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => (0..len.div_ceil(PAGE_LEN))
                .map(|page_index| {
                    changed_pages.get(&page_index).map_or_else(
                        || base.as_ptr().wrapping_add(page_index * PAGE_LEN) as usize,
                        |page| page.as_ptr() as usize,
                    )
                })
                .collect(),
        }
    }

    fn make_exclusive(&mut self) -> &mut Vec<T> {
        if matches!(self.storage, PersistentVectorStorage::ForkShared { .. }) {
            let values = self.iter().cloned().collect();
            self.storage = PersistentVectorStorage::Exclusive(values);
        }
        match &mut self.storage {
            PersistentVectorStorage::Exclusive(values) => values,
            PersistentVectorStorage::ForkShared { .. } => unreachable!("storage was flattened"),
        }
    }
}

fn install_changed_page<T: Clone, const PAGE_LEN: usize>(
    base: &Arc<Vec<T>>,
    changed_pages: &mut im::OrdMap<usize, Arc<Vec<T>>>,
    len: usize,
    page_index: usize,
) {
    if changed_pages.contains_key(&page_index) {
        return;
    }
    let start = page_index * PAGE_LEN;
    let end = len.min(start + PAGE_LEN).min(base.len());
    let page = if start < end {
        base[start..end].to_vec()
    } else {
        Vec::new()
    };
    changed_pages.insert(page_index, Arc::new(page));
}

impl<T: Clone, const PAGE_LEN: usize> Clone for PersistentVector<T, PAGE_LEN> {
    fn clone(&self) -> Self {
        match &self.storage {
            PersistentVectorStorage::Exclusive(_) => self.operational_clone(),
            PersistentVectorStorage::ForkShared {
                base,
                changed_pages,
                len,
            } => Self {
                storage: PersistentVectorStorage::ForkShared {
                    base: Arc::clone(base),
                    changed_pages: changed_pages.clone(),
                    len: *len,
                },
            },
        }
    }
}

impl<T: Clone + fmt::Debug, const PAGE_LEN: usize> fmt::Debug for PersistentVector<T, PAGE_LEN> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone + PartialEq, const PAGE_LEN: usize> PartialEq for PersistentVector<T, PAGE_LEN> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<T: Clone + Eq, const PAGE_LEN: usize> Eq for PersistentVector<T, PAGE_LEN> {}

impl<T: Clone, const PAGE_LEN: usize> Default for PersistentVector<T, PAGE_LEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const PAGE_LEN: usize> FromIterator<T> for PersistentVector<T, PAGE_LEN> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            storage: PersistentVectorStorage::Exclusive(iter.into_iter().collect()),
        }
    }
}

impl<T: Clone, const PAGE_LEN: usize> Index<usize> for PersistentVector<T, PAGE_LEN> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("persistent vector index in bounds")
    }
}

impl<T: Clone, const PAGE_LEN: usize> IndexMut<usize> for PersistentVector<T, PAGE_LEN> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("persistent vector index in bounds")
    }
}

impl<'a, T: Clone, const PAGE_LEN: usize> IntoIterator for &'a PersistentVector<T, PAGE_LEN> {
    type Item = &'a T;
    type IntoIter = PersistentVectorIter<'a, T, PAGE_LEN>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
