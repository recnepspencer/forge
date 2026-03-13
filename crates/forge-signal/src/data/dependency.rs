//! Dependency edges and snapshots for signal nodes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::handle::NodeId;
use crate::data::output::{InternedPartitionSubscription, PartitionSubscription, PartitionToken};
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

    /// Create a dependency edge scoped to one whole partition.
    pub fn whole_partition(
        source: NodeId,
        aspect: Aspect,
        partition: impl Into<PartitionToken>,
    ) -> Self {
        Self {
            source,
            aspect,
            scope: Some(PartitionSubscription::whole_partition(partition)),
            interned_scope: None,
        }
    }

    /// Create a dependency edge from an explicit partition subscription scope.
    pub fn with_partition_scope(
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Self {
        Self {
            source,
            aspect,
            scope: Some(scope),
            interned_scope: None,
        }
    }

    /// Create a dependency edge scoped to one partition detail.
    pub fn partition_detail(
        source: NodeId,
        aspect: Aspect,
        partition: impl Into<PartitionToken>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            source,
            aspect,
            scope: Some(PartitionSubscription::partition_and_detail(
                partition, detail,
            )),
            interned_scope: None,
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

    pub fn sort_key(&self) -> DependencySortKey {
        DependencySortKey {
            source_index: self.source.index(),
            source_generation: self.source.generation(),
            aspect_index: self.aspect.index(),
            scope: self.scope.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalDependencies {
    edges: Vec<DependencyEdge>,
}

impl CanonicalDependencies {
    pub fn new(edges: impl IntoIterator<Item = DependencyEdge>) -> Self {
        let mut edges = edges.into_iter().collect::<Vec<_>>();
        if edges.len() > 1 {
            edges.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
            edges.dedup_by(|left, right| left.sort_key() == right.sort_key());
        }
        Self { edges }
    }

    pub fn from_slice(edges: &[DependencyEdge]) -> Self {
        Self::new(edges.iter().cloned())
    }

    pub fn as_slice(&self) -> &[DependencyEdge] {
        &self.edges
    }

    pub fn into_vec(self) -> Vec<DependencyEdge> {
        self.edges
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

impl From<Vec<DependencyEdge>> for CanonicalDependencies {
    fn from(edges: Vec<DependencyEdge>) -> Self {
        Self::new(edges)
    }
}

impl From<&[DependencyEdge]> for CanonicalDependencies {
    fn from(edges: &[DependencyEdge]) -> Self {
        Self::from_slice(edges)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DependencySortKey {
    source_index: u32,
    source_generation: u32,
    aspect_index: usize,
    scope: Option<PartitionSubscription>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencySnapshotEntry {
    pub source: NodeId,
    pub aspect: Aspect,
    pub cached_version: u64,
    #[serde(default)]
    pub scope: Option<PartitionSubscription>,
}

impl DependencySnapshotEntry {
    pub fn sort_key(&self) -> DependencySortKey {
        DependencySortKey {
            source_index: self.source.index(),
            source_generation: self.source.generation(),
            aspect_index: self.aspect.index(),
            scope: self.scope.clone(),
        }
    }
}

/// A snapshot of upstream aspect versions at the time a node was last evaluated.
///
/// Used by the pull phase to determine if a `MaybeStale` node can revert
/// to `Clean`: if all upstream versions match the snapshot, no recomputation needed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencySnapshot {
    entries: Arc<Vec<DependencySnapshotEntry>>,
}

impl DependencySnapshot {
    /// Create an empty snapshot.
    pub fn empty() -> Self {
        Self {
            entries: Arc::new(Vec::new()),
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
        Arc::make_mut(&mut self.entries).push(DependencySnapshotEntry {
            source,
            aspect,
            cached_version: version,
            scope,
        });
    }

    /// All recorded entries.
    pub fn entries(&self) -> &[DependencySnapshotEntry] {
        self.entries.as_slice()
    }

    fn canonicalize(&mut self) {
        let entries = Arc::make_mut(&mut self.entries);
        entries.sort_by(|left, right| {
            left.sort_key()
                .cmp(&right.sort_key())
                .then(left.cached_version.cmp(&right.cached_version))
        });
        let mut normalized: Vec<DependencySnapshotEntry> = Vec::with_capacity(entries.len());
        for entry in entries.drain(..) {
            if let Some(previous) = normalized.last_mut() {
                if previous.sort_key() == entry.sort_key() {
                    if previous.cached_version <= entry.cached_version {
                        *previous = entry;
                    }
                    continue;
                }
            }
            normalized.push(entry);
        }
        *entries = normalized;
    }

    pub fn shared_entries(&self) -> Arc<Vec<DependencySnapshotEntry>> {
        Arc::clone(&self.entries)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedDependencySnapshot {
    snapshot: DependencySnapshot,
}

impl SharedDependencySnapshot {
    pub fn new(snapshot: DependencySnapshot) -> Self {
        Self { snapshot }
    }

    pub fn empty() -> Self {
        Self::new(DependencySnapshot::empty())
    }

    pub fn snapshot(&self) -> &DependencySnapshot {
        &self.snapshot
    }

    pub fn entries(&self) -> &[DependencySnapshotEntry] {
        self.snapshot.entries()
    }

    pub fn into_snapshot(self) -> DependencySnapshot {
        self.snapshot
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
        self.interner.reserve(self.snapshots.len());
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
