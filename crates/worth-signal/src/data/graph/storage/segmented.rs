use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use super::handles::{DependencySetId, SetHandle, SubscriberSetId};
use crate::data::dependency::DependencyEdge;
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Segment {
    start: u32,
    len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentedStore<T: Clone, Id: Clone> {
    segments: crate::data::persistent_vector::PersistentVector<Vec<T>>,
    interner: crate::data::persistent_hash_map::PersistentHashMap<u64, Vec<Id>>,
    id: PhantomData<Id>,
}

impl<T: Clone, Id: Clone> Default for SegmentedStore<T, Id> {
    fn default() -> Self {
        Self {
            segments: crate::data::persistent_vector::PersistentVector::new(),
            interner: crate::data::persistent_hash_map::PersistentHashMap::new(),
            id: PhantomData,
        }
    }
}

impl<T, Id> SegmentedStore<T, Id>
where
    T: Clone + Hash + PartialEq,
    Id: SetHandle,
{
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.segments.is_empty() {
            return;
        }
        for (index, segment) in self.segments.iter().enumerate() {
            self.interner
                .entry(hash_slice(segment))
                .or_default()
                .push(Id::from_index(index + 1));
        }
    }

    pub fn get(&self, id: Id) -> &[T] {
        match id.index() {
            Some(index) => self.segments[index - 1].as_slice(),
            None => &[],
        }
    }

    pub fn insert_from_slice(&mut self, items: &[T]) -> Id {
        if items.is_empty() {
            return Id::EMPTY;
        }
        self.rebuild_interner_if_needed();
        let hash = hash_slice(items);
        if let Some(candidates) = self.interner.get(&hash) {
            for &candidate in candidates {
                if self.get(candidate) == items {
                    return candidate;
                }
            }
        }
        self.segments.push_back(items.to_vec());
        let id = Id::from_index(self.segments.len());
        self.interner.entry(hash).or_default().push(id);
        id
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (
            self.segments.iter().map(Vec::len).sum(),
            self.segments.len(),
        )
    }

    pub(crate) fn live_segment_count(&self) -> usize {
        self.segments.len()
    }
}

impl<T, Id> SegmentedStore<T, Id>
where
    T: Clone,
    Id: Clone,
{
    pub(crate) fn operational_clone(&self) -> Self {
        Self {
            segments: self.segments.operational_clone(),
            interner: self.interner.operational_clone(),
            id: PhantomData,
        }
    }

    pub(crate) fn fork_persistent(&mut self) -> Self {
        Self {
            segments: self.segments.fork_persistent(),
            interner: self.interner.fork_persistent(),
            id: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        Self {
            segments: self.segments.clone(),
            interner: self.interner.fork_storage_identity(),
            id: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.segments.shares_storage_with(&other.segments) && self.interner.ptr_eq(&other.interner)
    }
}

#[derive(Serialize, Deserialize)]
struct SegmentedStoreWire<T> {
    items: Vec<T>,
    segments: Vec<Segment>,
}

impl<T, Id> Serialize for SegmentedStore<T, Id>
where
    T: Clone + Serialize,
    Id: Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut items = Vec::new();
        let mut segments = Vec::with_capacity(self.segments.len());
        for values in &self.segments {
            let start = checked_segment_component(items.len(), "segment start");
            items.extend(values.iter().cloned());
            segments.push(Segment {
                start,
                len: checked_segment_component(values.len(), "segment length"),
            });
        }
        SegmentedStoreWire { items, segments }.serialize(serializer)
    }
}

impl<'de, T, Id> Deserialize<'de> for SegmentedStore<T, Id>
where
    T: Clone + Deserialize<'de>,
    Id: Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = SegmentedStoreWire::<T>::deserialize(deserializer)?;
        let mut segments = crate::data::persistent_vector::PersistentVector::new();
        for segment in wire.segments {
            let start = segment.start as usize;
            let end = start.saturating_add(segment.len as usize);
            let values = wire.items.get(start..end).ok_or_else(|| {
                serde::de::Error::custom("segmented store range exceeds item storage")
            })?;
            segments.push_back(values.to_vec());
        }
        Ok(Self {
            segments,
            interner: crate::data::persistent_hash_map::PersistentHashMap::new(),
            id: PhantomData,
        })
    }
}

pub type DependencyEdgeStore = SegmentedStore<DependencyEdge, DependencySetId>;
pub type SubscriberEdgeStore = SegmentedStore<NodeId, SubscriberSetId>;

#[cfg(test)]
pub(crate) fn checked_segment_component_for_test(
    value: usize,
) -> Result<u32, crate::data::error::SignalError> {
    u32::try_from(value).map_err(|_| {
        crate::data::error::SignalError::invalid_input(format!(
            "edge-store segment component `{value}` exceeds u32 capacity"
        ))
    })
}

fn hash_slice<T: Hash>(items: &[T]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    items.hash(&mut hasher);
    hasher.finish()
}

fn checked_segment_component(value: usize, label: &str) -> u32 {
    u32::try_from(value).unwrap_or_else(|_| {
        panic!("worth-signal edge store overflow: {label} `{value}` exceeds u32 capacity")
    })
}
