use serde::{Deserialize, Serialize};
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
}

impl DependencyEdgeStore {
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
        let start = self.edges.len() as u32;
        self.edges.extend_from_slice(edges);
        self.segments.push(Segment {
            start,
            len: edges.len() as u32,
        });
        DependencySetId::from_index(self.segments.len())
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.edges.len(), self.segments.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscriberEdgeStore {
    subscribers: Vec<NodeId>,
    segments: Vec<Segment>,
}

impl SubscriberEdgeStore {
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
        let start = self.subscribers.len() as u32;
        self.subscribers.extend_from_slice(subscribers);
        self.segments.push(Segment {
            start,
            len: subscribers.len() as u32,
        });
        SubscriberSetId::from_index(self.segments.len())
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.subscribers.len(), self.segments.len())
    }
}
