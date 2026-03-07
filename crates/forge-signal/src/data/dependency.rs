//! Dependency edges and snapshots for signal nodes.

use serde::{Deserialize, Serialize};

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::handle::NodeId;

/// A dependency edge recording which upstream node and aspect a downstream reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdge {
    source: NodeId,
    aspect: Aspect,
}

impl DependencyEdge {
    /// Create a new dependency edge.
    pub fn new(source: NodeId, aspect: Aspect) -> Self {
        Self { source, aspect }
    }

    /// The upstream node this edge points to.
    pub fn source(self) -> NodeId {
        self.source
    }

    /// Which aspect of the upstream node is subscribed to.
    pub fn aspect(self) -> Aspect {
        self.aspect
    }

    /// The subscribed aspect mask.
    pub fn aspect_mask(self) -> AspectMask {
        AspectMask::from_aspect(self.aspect)
    }
}

/// A snapshot of upstream aspect versions at the time a node was last evaluated.
///
/// Used by the pull phase to determine if a `MaybeStale` node can revert
/// to `Clean`: if all upstream versions match the snapshot, no recomputation needed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySnapshot {
    entries: Vec<(NodeId, Aspect, u64)>,
}

impl DependencySnapshot {
    /// Create an empty snapshot.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Record an upstream version.
    pub fn record(&mut self, source: NodeId, aspect: Aspect, version: u64) {
        self.entries.push((source, aspect, version));
    }

    /// All recorded entries.
    pub fn entries(&self) -> &[(NodeId, Aspect, u64)] {
        &self.entries
    }
}
