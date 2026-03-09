//! Dependency edges and snapshots for signal nodes.

use serde::{Deserialize, Serialize};

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::handle::NodeId;
use crate::data::output::{InternedPartitionSubscription, PartitionSubscription};

/// A dependency edge recording which upstream node and aspect a downstream reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdge {
    source: NodeId,
    aspect: Aspect,
    #[serde(default)]
    scope: Option<PartitionSubscription>,
    #[serde(default)]
    interned_scope: Option<InternedPartitionSubscription>,
}

impl DependencyEdge {
    /// Create a new dependency edge.
    pub fn new(source: NodeId, aspect: Aspect) -> Self {
        Self {
            source,
            aspect,
            scope: None,
            interned_scope: None,
        }
    }

    /// Create a partition-scoped dependency edge.
    pub fn with_scope(
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
        interned_scope: InternedPartitionSubscription,
    ) -> Self {
        Self {
            source,
            aspect,
            scope: Some(scope),
            interned_scope: Some(interned_scope),
        }
    }

    /// The upstream node this edge points to.
    pub fn source(&self) -> NodeId {
        self.source
    }

    /// Which aspect of the upstream node is subscribed to.
    pub fn aspect(&self) -> Aspect {
        self.aspect
    }

    /// Optional partition subscription scope.
    pub fn scope_ref(&self) -> Option<&PartitionSubscription> {
        self.scope.as_ref()
    }

    /// Optional compact interned scope for hot-path routing.
    pub fn interned_scope(&self) -> Option<InternedPartitionSubscription> {
        self.interned_scope
    }

    /// The subscribed aspect mask.
    pub fn aspect_mask(&self) -> AspectMask {
        AspectMask::from_aspect(self.aspect)
    }
}

/// A snapshot of upstream aspect versions at the time a node was last evaluated.
///
/// Used by the pull phase to determine if a `MaybeStale` node can revert
/// to `Clean`: if all upstream versions match the snapshot, no recomputation needed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySnapshot {
    entries: Vec<(NodeId, Aspect, u64, Option<PartitionSubscription>)>,
}

impl DependencySnapshot {
    /// Create an empty snapshot.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Record an upstream version.
    pub fn record(
        &mut self,
        source: NodeId,
        aspect: Aspect,
        version: u64,
        scope: Option<PartitionSubscription>,
    ) {
        self.entries.push((source, aspect, version, scope));
    }

    /// All recorded entries.
    pub fn entries(&self) -> &[(NodeId, Aspect, u64, Option<PartitionSubscription>)] {
        &self.entries
    }
}
