use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::handles::{DependencySetId, SetHandle, SubscriberSetId};
use crate::data::dependency::DependencyEdge;
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Segment {
    start: u32,
    len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentedStore<T, Id> {
    items: Vec<T>,
    segments: Vec<Segment>,
    #[serde(skip, default)]
    interner: HashMap<u64, Vec<Id>>,
}

impl<T, Id> Default for SegmentedStore<T, Id> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            segments: Vec::new(),
            interner: HashMap::new(),
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
        self.interner.reserve(self.segments.len());
        for (index, segment) in self.segments.iter().copied().enumerate() {
            let slice = &self.items[segment.start as usize..(segment.start + segment.len) as usize];
            self.interner
                .entry(hash_slice(slice))
                .or_default()
                .push(Id::from_index(index + 1));
        }
    }

    pub fn get(&self, id: Id) -> &[T] {
        match id.index() {
            Some(index) => {
                let segment = self.segments[index - 1];
                &self.items[segment.start as usize..(segment.start + segment.len) as usize]
            }
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
        let start = checked_segment_component(self.items.len(), "segment start");
        self.items.extend_from_slice(items);
        self.segments.push(Segment {
            start,
            len: checked_segment_component(items.len(), "segment length"),
        });
        let id = Id::from_index(self.segments.len());
        self.interner.entry(hash).or_default().push(id);
        id
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.items.len(), self.segments.len())
    }

    pub(crate) fn live_segment_count(&self) -> usize {
        self.segments.len()
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
