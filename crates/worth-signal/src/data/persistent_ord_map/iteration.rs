use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

use super::fork_overlay::retired_interval_end;

pub(crate) struct LiveBaseIter<'a, K: Clone + Ord, V> {
    base: &'a BTreeMap<K, V>,
    range: std::collections::btree_map::Range<'a, K, V>,
    retired_intervals: &'a im::OrdMap<K, K>,
}

impl<'a, K: Clone + Ord, V> LiveBaseIter<'a, K, V> {
    pub(crate) fn new(
        base: &'a BTreeMap<K, V>,
        retired_intervals: &'a im::OrdMap<K, K>,
        first_live_key: &'a K,
    ) -> Self {
        Self {
            base,
            range: base.range((Included(first_live_key), Unbounded)),
            retired_intervals,
        }
    }
}

impl<'a, K: Clone + Ord, V> Iterator for LiveBaseIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (key, value) = self.range.next()?;
            let Some(retired_end) = retired_interval_end(self.retired_intervals, key) else {
                return Some((key, value));
            };
            self.range = self.base.range((Excluded(retired_end), Unbounded));
        }
    }
}

pub(crate) enum PersistentOrdMapIter<'a, K: Clone + Ord, V: Clone> {
    Empty,
    Exclusive(std::collections::btree_map::Iter<'a, K, V>),
    ForkShared {
        base: std::iter::Peekable<LiveBaseIter<'a, K, V>>,
        changes: std::iter::Peekable<im::ordmap::Iter<'a, K, V>>,
        remaining: usize,
    },
}

impl<'a, K: Clone + Ord, V: Clone> Iterator for PersistentOrdMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self, Self::Empty) {
            return None;
        }
        let Self::ForkShared {
            base,
            changes,
            remaining,
        } = self
        else {
            return match self {
                Self::Exclusive(values) => values.next(),
                Self::Empty | Self::ForkShared { .. } => unreachable!(),
            };
        };
        let next = match (base.peek(), changes.peek()) {
            (Some((base_key, _)), Some((change_key, _))) if base_key < change_key => base.next(),
            (Some((base_key, _)), Some((change_key, _))) if base_key == change_key => {
                base.next();
                changes.next()
            }
            (_, Some(_)) => changes.next(),
            (Some(_), None) => base.next(),
            (None, None) => None,
        };
        if next.is_some() {
            *remaining -= 1;
        }
        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Empty => (0, Some(0)),
            Self::Exclusive(values) => values.size_hint(),
            Self::ForkShared { remaining, .. } => (*remaining, Some(*remaining)),
        }
    }
}
