use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU32;

use crate::data::dependency::DependencyEdge;
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencySetId(Option<NonZeroU32>);

impl DependencySetId {
    pub const EMPTY: Self = Self(None);

    fn from_index(index: usize) -> Self {
        debug_assert!(index > 0);
        Self(NonZeroU32::new(index as u32))
    }

    fn index(self) -> Option<usize> {
        self.0.map(|index| index.get() as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriberSetId(Option<NonZeroU32>);

impl SubscriberSetId {
    pub const EMPTY: Self = Self(None);

    fn from_index(index: usize) -> Self {
        debug_assert!(index > 0);
        Self(NonZeroU32::new(index as u32))
    }

    fn index(self) -> Option<usize> {
        self.0.map(|index| index.get() as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Segment {
    start: u32,
    len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencyEdgeStore {
    edges: Vec<DependencyEdge>,
    segments: Vec<Segment>,
    #[serde(skip, default)]
    interner: HashMap<u64, Vec<DependencySetId>>,
}

impl DependencyEdgeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.segments.is_empty() {
            return;
        }
        self.interner.reserve(self.segments.len());
        for (index, segment) in self.segments.iter().copied().enumerate() {
            let slice = &self.edges[segment.start as usize..(segment.start + segment.len) as usize];
            self.interner
                .entry(hash_slice(slice))
                .or_default()
                .push(DependencySetId::from_index(index + 1));
        }
    }

    pub fn get(&self, id: DependencySetId) -> &[DependencyEdge] {
        match id.index() {
            Some(index) => {
                let segment = self.segments[index - 1];
                &self.edges[segment.start as usize..(segment.start + segment.len) as usize]
            }
            None => &[],
        }
    }

    pub fn insert_from_slice(&mut self, edges: &[DependencyEdge]) -> DependencySetId {
        if edges.is_empty() {
            return DependencySetId::EMPTY;
        }
        self.rebuild_interner_if_needed();
        let hash = hash_slice(edges);
        if let Some(candidates) = self.interner.get(&hash) {
            for &candidate in candidates {
                if self.get(candidate) == edges {
                    return candidate;
                }
            }
        }
        let start = self.edges.len() as u32;
        self.edges.extend_from_slice(edges);
        self.segments.push(Segment {
            start,
            len: edges.len() as u32,
        });
        let id = DependencySetId::from_index(self.segments.len());
        self.interner.entry(hash).or_default().push(id);
        id
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.edges.len(), self.segments.len())
    }

    pub(crate) fn live_segment_count(&self) -> usize {
        self.segments.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscriberEdgeStore {
    subscribers: Vec<NodeId>,
    segments: Vec<Segment>,
    #[serde(skip, default)]
    interner: HashMap<u64, Vec<SubscriberSetId>>,
}

impl SubscriberEdgeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.segments.is_empty() {
            return;
        }
        self.interner.reserve(self.segments.len());
        for (index, segment) in self.segments.iter().copied().enumerate() {
            let slice =
                &self.subscribers[segment.start as usize..(segment.start + segment.len) as usize];
            self.interner
                .entry(hash_slice(slice))
                .or_default()
                .push(SubscriberSetId::from_index(index + 1));
        }
    }

    pub fn get(&self, id: SubscriberSetId) -> &[NodeId] {
        match id.index() {
            Some(index) => {
                let segment = self.segments[index - 1];
                &self.subscribers[segment.start as usize..(segment.start + segment.len) as usize]
            }
            None => &[],
        }
    }

    pub fn insert_from_slice(&mut self, subscribers: &[NodeId]) -> SubscriberSetId {
        if subscribers.is_empty() {
            return SubscriberSetId::EMPTY;
        }
        self.rebuild_interner_if_needed();
        let hash = hash_slice(subscribers);
        if let Some(candidates) = self.interner.get(&hash) {
            for &candidate in candidates {
                if self.get(candidate) == subscribers {
                    return candidate;
                }
            }
        }
        let start = self.subscribers.len() as u32;
        self.subscribers.extend_from_slice(subscribers);
        self.segments.push(Segment {
            start,
            len: subscribers.len() as u32,
        });
        let id = SubscriberSetId::from_index(self.segments.len());
        self.interner.entry(hash).or_default().push(id);
        id
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.subscribers.len(), self.segments.len())
    }

    pub(crate) fn live_segment_count(&self) -> usize {
        self.segments.len()
    }
}

fn hash_slice<T: Hash>(items: &[T]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    items.hash(&mut hasher);
    hasher.finish()
}
