use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use crate::config::data::AdjacencyPolicy;

use super::AdjacencySet;

#[derive(Debug, Clone, Default)]
pub(crate) struct SparseAdjacencyTable {
    entries: BTreeMap<usize, AdjacencySet>,
}

impl SparseAdjacencyTable {
    pub(crate) fn get(&self, slot: usize) -> Option<&AdjacencySet> {
        self.entries.get(&slot)
    }

    pub(crate) fn get_mut(&mut self, slot: usize) -> Option<&mut AdjacencySet> {
        self.entries.get_mut(&slot)
    }

    pub(crate) fn ensure(&mut self, slot: usize, policy: &AdjacencyPolicy) -> &mut AdjacencySet {
        self.entries
            .entry(slot)
            .or_insert_with(|| AdjacencySet::new(policy))
    }

    pub(crate) fn clear_slot(&mut self, slot: usize, policy: &AdjacencyPolicy) {
        self.entries.insert(slot, AdjacencySet::new(policy));
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&usize, &AdjacencySet)> {
        self.entries.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut AdjacencySet)> {
        self.entries.iter_mut()
    }

    pub(crate) fn into_entries(self) -> impl Iterator<Item = (usize, AdjacencySet)> {
        self.entries.into_iter()
    }

    pub(crate) fn from_entries(entries: impl IntoIterator<Item = (usize, AdjacencySet)>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }

    pub(crate) fn allocation_bytes(&self) -> u64 {
        (self.entries.len() as u64).saturating_mul(
            (std::mem::size_of::<usize>() + std::mem::size_of::<AdjacencySet>()) as u64,
        )
    }
}

impl From<Vec<AdjacencySet>> for SparseAdjacencyTable {
    fn from(entries: Vec<AdjacencySet>) -> Self {
        Self {
            entries: entries.into_iter().enumerate().collect(),
        }
    }
}

impl Index<usize> for SparseAdjacencyTable {
    type Output = AdjacencySet;

    fn index(&self, slot: usize) -> &Self::Output {
        &self.entries[&slot]
    }
}

impl IndexMut<usize> for SparseAdjacencyTable {
    fn index_mut(&mut self, slot: usize) -> &mut Self::Output {
        self.entries
            .get_mut(&slot)
            .expect("adjacency slot must be materialized")
    }
}
