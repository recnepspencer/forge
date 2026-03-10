//! Dependency edges and snapshots for signal nodes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU32;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::handle::NodeId;
use crate::data::output::{InternedPartitionSubscription, PartitionSubscription};
use std::cmp::Ordering;

/// A dependency edge recording which upstream node and aspect a downstream reads.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    fn canonicalize(&mut self) {
        self.entries.sort_by(compare_snapshot_entries);
        self.entries.dedup();
    }
}

/// Stable handle into graph-owned dependency snapshot storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencySnapshotId(Option<NonZeroU32>);

impl DependencySnapshotId {
    /// The canonical empty snapshot id.
    pub const EMPTY: Self = Self(None);

    fn from_index(index: usize) -> Self {
        debug_assert!(index > 0);
        Self(NonZeroU32::new(index as u32))
    }

    fn index(self) -> Option<usize> {
        self.0.map(|index| index.get() as usize)
    }
}

/// Graph-owned storage for immutable dependency snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencySnapshotStore {
    snapshots: Vec<DependencySnapshot>,
    #[serde(skip, default)]
    interner: HashMap<DependencySnapshot, DependencySnapshotId>,
}

impl DependencySnapshotStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.snapshots.is_empty() {
            return;
        }
        for (index, snapshot) in self.snapshots.iter().cloned().enumerate() {
            self.interner
                .insert(snapshot, DependencySnapshotId::from_index(index + 1));
        }
    }

    /// Read one snapshot by id.
    pub fn get(&self, id: DependencySnapshotId) -> &DependencySnapshot {
        match id.index() {
            Some(index) => &self.snapshots[index - 1],
            None => empty_dependency_snapshot(),
        }
    }

    /// Store one immutable snapshot and return its id.
    pub fn insert(&mut self, snapshot: DependencySnapshot) -> DependencySnapshotId {
        let mut snapshot = snapshot;
        snapshot.canonicalize();
        if snapshot.entries().is_empty() {
            return DependencySnapshotId::EMPTY;
        }
        self.rebuild_interner_if_needed();
        if let Some(id) = self.interner.get(&snapshot).copied() {
            return id;
        }
        self.snapshots.push(snapshot);
        let id = DependencySnapshotId::from_index(self.snapshots.len());
        let snapshot = self.snapshots[id.index().expect("snapshot id should index") - 1].clone();
        self.interner.insert(snapshot, id);
        id
    }

    #[cfg(test)]
    pub(crate) fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    pub(crate) fn live_snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

fn empty_dependency_snapshot() -> &'static DependencySnapshot {
    static EMPTY: std::sync::OnceLock<DependencySnapshot> = std::sync::OnceLock::new();
    EMPTY.get_or_init(DependencySnapshot::empty)
}

fn compare_snapshot_entries(
    left: &(NodeId, Aspect, u64, Option<PartitionSubscription>),
    right: &(NodeId, Aspect, u64, Option<PartitionSubscription>),
) -> Ordering {
    (
        left.0.index(),
        left.0.generation(),
        left.1.index(),
        left.2,
        &left.3,
    )
        .cmp(&(
            right.0.index(),
            right.0.generation(),
            right.1.index(),
            right.2,
            &right.3,
        ))
}
