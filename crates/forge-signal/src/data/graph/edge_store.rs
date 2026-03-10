use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    interner: HashMap<Vec<DependencyEdge>, DependencySetId>,
}

impl DependencyEdgeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.segments.is_empty() {
            return;
        }
        for (index, segment) in self.segments.iter().copied().enumerate() {
            let slice = &self.edges[segment.start as usize..(segment.start + segment.len) as usize];
            self.interner
                .insert(slice.to_vec(), DependencySetId::from_index(index + 1));
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
        if let Some(id) = self.interner.get(edges).copied() {
            return id;
        }
        let start = self.edges.len() as u32;
        self.edges.extend_from_slice(edges);
        self.segments.push(Segment {
            start,
            len: edges.len() as u32,
        });
        let id = DependencySetId::from_index(self.segments.len());
        self.interner.insert(edges.to_vec(), id);
        id
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
    #[serde(skip, default)]
    interner: HashMap<Vec<NodeId>, SubscriberSetId>,
}

impl SubscriberEdgeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.segments.is_empty() {
            return;
        }
        for (index, segment) in self.segments.iter().copied().enumerate() {
            let slice =
                &self.subscribers[segment.start as usize..(segment.start + segment.len) as usize];
            self.interner
                .insert(slice.to_vec(), SubscriberSetId::from_index(index + 1));
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
        if let Some(id) = self.interner.get(subscribers).copied() {
            return id;
        }
        let start = self.subscribers.len() as u32;
        self.subscribers.extend_from_slice(subscribers);
        self.segments.push(Segment {
            start,
            len: subscribers.len() as u32,
        });
        let id = SubscriberSetId::from_index(self.segments.len());
        self.interner.insert(subscribers.to_vec(), id);
        id
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> (usize, usize) {
        (self.subscribers.len(), self.segments.len())
    }
}
