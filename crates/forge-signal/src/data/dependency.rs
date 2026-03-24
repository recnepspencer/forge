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
    edges: Arc<Vec<DependencyEdge>>,
}

impl CanonicalDependencies {
    pub fn new(edges: impl IntoIterator<Item = DependencyEdge>) -> Self {
        Self::canonicalize_unordered(edges)
    }

    pub fn canonicalize_unordered(edges: impl IntoIterator<Item = DependencyEdge>) -> Self {
        let mut edges = edges.into_iter().collect::<Vec<_>>();
        if edges.len() > 1 {
            edges.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
            edges.dedup_by(|left, right| left.sort_key() == right.sort_key());
        }
        Self {
            edges: Arc::new(edges),
        }
    }

    pub fn from_ordered_unique(edges: impl IntoIterator<Item = DependencyEdge>) -> Self {
        let edges = edges.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_dependency_edge_order(edges.as_slice()));
        Self {
            edges: Arc::new(edges),
        }
    }

    pub fn from_slice(edges: &[DependencyEdge]) -> Self {
        Self::new(edges.iter().cloned())
    }

    pub fn as_slice(&self) -> &[DependencyEdge] {
        self.edges.as_slice()
    }

    pub fn into_vec(self) -> Vec<DependencyEdge> {
        match Arc::try_unwrap(self.edges) {
            Ok(edges) => edges,
            Err(edges) => edges.as_ref().clone(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct DependencySnapshotShape {
    keys: Arc<Vec<DependencySortKey>>,
}

impl DependencySnapshotShape {
    pub fn from_ordered_unique(keys: impl IntoIterator<Item = DependencySortKey>) -> Self {
        let keys = keys.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_snapshot_shape_order(keys.as_slice()));
        Self {
            keys: Arc::new(keys),
        }
    }

    pub fn as_slice(&self) -> &[DependencySortKey] {
        self.keys.as_slice()
    }

    pub(crate) fn intern(
        &self,
        store: &mut DependencySnapshotShapeStore,
    ) -> SnapshotShapeHandle {
        store.intern(self.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct SnapshotShapeHandle(Option<NonZeroU32>);

impl SnapshotShapeHandle {
    pub const EMPTY: Self = Self(None);

    fn from_index(index: usize) -> Self {
        debug_assert!(index > 0);
        Self(NonZeroU32::new(index as u32))
    }

    fn index(self) -> Option<usize> {
        self.0.map(|index| index.get() as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencySnapshotShapeStore {
    shapes: Vec<DependencySnapshotShape>,
    #[serde(skip, default)]
    interner: HashMap<DependencySnapshotShape, SnapshotShapeHandle>,
}

impl DependencySnapshotShapeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.shapes.is_empty() {
            return;
        }
        self.interner.reserve(self.shapes.len());
        for (index, shape) in self.shapes.iter().cloned().enumerate() {
            self.interner
                .insert(shape, SnapshotShapeHandle::from_index(index + 1));
        }
    }

    pub fn intern(&mut self, shape: DependencySnapshotShape) -> SnapshotShapeHandle {
        if shape.as_slice().is_empty() {
            return SnapshotShapeHandle::EMPTY;
        }
        self.rebuild_interner_if_needed();
        if let Some(handle) = self.interner.get(&shape).copied() {
            return handle;
        }
        self.shapes.push(shape);
        let handle = SnapshotShapeHandle::from_index(self.shapes.len());
        let shape = self.shapes[handle.index().expect("shape handle should index") - 1].clone();
        self.interner.insert(shape, handle);
        handle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotChangeKind {
    Unchanged,
    StableShapeVersionOnly,
    StructuralReplace,
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

    pub fn canonicalize_unordered(mut self) -> Self {
        self.canonicalize_in_place();
        self
    }

    pub fn from_ordered_unique(entries: impl IntoIterator<Item = DependencySnapshotEntry>) -> Self {
        let entries = entries.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_snapshot_entry_order(entries.as_slice()));
        Self {
            entries: Arc::new(entries),
        }
    }

    fn canonicalize_in_place(&mut self) {
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

    pub fn shape(&self) -> DependencySnapshotShape {
        DependencySnapshotShape::from_ordered_unique(
            self.entries().iter().map(DependencySnapshotEntry::sort_key),
        )
    }

    /// Whether two snapshots currently share the same backing storage.
    ///
    /// This is a storage fact only. Snapshot identity, restore semantics, and
    /// reuse semantics must still be defined by explicit snapshot contracts.
    pub fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.entries, &other.entries)
    }

    pub fn with_updated_versions(&self, cached_versions: &[u64]) -> Self {
        debug_assert_eq!(self.entries().len(), cached_versions.len());
        let mut updated = self.clone();
        for (entry, cached_version) in Arc::make_mut(&mut updated.entries)
            .iter_mut()
            .zip(cached_versions.iter().copied())
        {
            entry.cached_version = cached_version;
        }
        updated
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

    /// Whether two shared snapshots currently point at the same backing
    /// storage. This is a storage-strategy fact, not semantic identity.
    pub fn shares_storage_with(&self, other: &Self) -> bool {
        self.snapshot.shares_storage_with(&other.snapshot)
    }
}

impl CommittedSnapshotUpdate {
    pub fn between(
        node: NodeId,
        previous_snapshot_id: DependencySnapshotId,
        previous_shape_handle: SnapshotShapeHandle,
        previous: &DependencySnapshot,
        next: DependencySnapshot,
        shape_store: &mut DependencySnapshotShapeStore,
    ) -> (Self, SnapshotDeltaRecord) {
        let next = next.canonicalize_unordered();
        if snapshot_shape_matches(previous, &next) {
            let cached_versions = next
                .entries()
                .iter()
                .map(|entry| entry.cached_version)
                .collect::<Vec<_>>();
            let basis = StableShapeSnapshotBasis {
                node,
                previous_snapshot_id,
                shape_handle: previous_shape_handle,
                entry_count: cached_versions.len(),
            };
            let delta = SnapshotDeltaRecord::for_version_update(node, previous, &cached_versions);
            return (
                Self::VersionOnly(VersionOnlySnapshotUpdate::from_basis_and_versions(
                    basis,
                    VersionVector {
                        cached_versions: Arc::new(cached_versions),
                    },
                )),
                delta,
            );
        }

        let replacement = ReplacementSnapshotUpdate::from_snapshot(next, shape_store);
        let delta = SnapshotDeltaRecord::between(node, previous, replacement.snapshot());
        (Self::Replace(replacement), delta)
    }

    pub fn storage_strategy(&self) -> SnapshotStorageStrategy {
        match self {
            Self::VersionOnly(_) => SnapshotStorageStrategy::VersionOnlyDelta,
            Self::Replace(_) => SnapshotStorageStrategy::SharedReplacement,
        }
    }

    pub fn entry_count(&self) -> usize {
        match self {
            Self::VersionOnly(update) => update.versions().len(),
            Self::Replace(update) => update.snapshot().entries().len(),
        }
    }

    pub fn apply_to(self, previous: &DependencySnapshot) -> SharedDependencySnapshot {
        match self {
            Self::VersionOnly(update) => {
                SharedDependencySnapshot::new(previous.with_updated_versions(update.versions().as_slice()))
            }
            Self::Replace(update) => update.snapshot,
        }
    }

    pub fn change_kind(&self) -> SnapshotChangeKind {
        match self {
            Self::VersionOnly(_) => SnapshotChangeKind::StableShapeVersionOnly,
            Self::Replace(_) => SnapshotChangeKind::StructuralReplace,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyInputScan {
    node: NodeId,
    previous_snapshot_id: DependencySnapshotId,
    previous_entry_count: usize,
    ordered_dependency_count: usize,
    shape_stable: bool,
    stable_shape_versions: Arc<Vec<u64>>,
}

impl DependencyInputScan {
    pub(crate) fn stable_shape(
        node: NodeId,
        previous_snapshot_id: DependencySnapshotId,
        previous_entry_count: usize,
        ordered_dependency_count: usize,
        stable_shape_versions: Vec<u64>,
    ) -> Self {
        Self {
            node,
            previous_snapshot_id,
            previous_entry_count,
            ordered_dependency_count,
            shape_stable: true,
            stable_shape_versions: Arc::new(stable_shape_versions),
        }
    }

    pub(crate) fn stable_shape_versions(&self) -> &[u64] {
        self.stable_shape_versions.as_slice()
    }

    pub(crate) fn stable_shape_versions_arc(&self) -> Arc<Vec<u64>> {
        Arc::clone(&self.stable_shape_versions)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableShapeSnapshotBasis {
    node: NodeId,
    previous_snapshot_id: DependencySnapshotId,
    shape_handle: SnapshotShapeHandle,
    entry_count: usize,
}

impl StableShapeSnapshotBasis {
    pub(crate) fn prove(
        scan: &DependencyInputScan,
        previous_shape_handle: SnapshotShapeHandle,
    ) -> Option<Self> {
        if !scan.shape_stable {
            return None;
        }
        if scan.previous_entry_count != scan.ordered_dependency_count {
            return None;
        }
        Some(Self {
            node: scan.node,
            previous_snapshot_id: scan.previous_snapshot_id,
            shape_handle: previous_shape_handle,
            entry_count: scan.ordered_dependency_count,
        })
    }

    #[cfg(test)]
    pub(crate) fn shape_handle(&self) -> SnapshotShapeHandle {
        self.shape_handle
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entry_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionVector {
    cached_versions: Arc<Vec<u64>>,
}

impl VersionVector {
    pub(crate) fn from_scan(
        basis: &StableShapeSnapshotBasis,
        scan: &DependencyInputScan,
    ) -> Self {
        debug_assert_eq!(basis.entry_count(), scan.stable_shape_versions().len());
        Self {
            cached_versions: scan.stable_shape_versions_arc(),
        }
    }

    pub fn as_slice(&self) -> &[u64] {
        self.cached_versions.as_slice()
    }

    pub fn len(&self) -> usize {
        self.cached_versions.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionOnlySnapshotUpdate {
    basis: StableShapeSnapshotBasis,
    versions: VersionVector,
}

impl VersionOnlySnapshotUpdate {
    pub(crate) fn from_basis_and_versions(
        basis: StableShapeSnapshotBasis,
        versions: VersionVector,
    ) -> Self {
        Self { basis, versions }
    }

    pub fn basis(&self) -> &StableShapeSnapshotBasis {
        &self.basis
    }

    pub fn versions(&self) -> &VersionVector {
        &self.versions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementSnapshotUpdate {
    snapshot: SharedDependencySnapshot,
    shape_handle: SnapshotShapeHandle,
}

impl ReplacementSnapshotUpdate {
    pub(crate) fn from_snapshot(
        snapshot: DependencySnapshot,
        shape_store: &mut DependencySnapshotShapeStore,
    ) -> Self {
        let snapshot = snapshot.canonicalize_unordered();
        let shape_handle = snapshot.shape().intern(shape_store);
        Self {
            snapshot: SharedDependencySnapshot::new(snapshot),
            shape_handle,
        }
    }

    pub fn snapshot(&self) -> &SharedDependencySnapshot {
        &self.snapshot
    }

    pub fn shape_handle(&self) -> SnapshotShapeHandle {
        self.shape_handle
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommittedSnapshotUpdate {
    VersionOnly(VersionOnlySnapshotUpdate),
    Replace(ReplacementSnapshotUpdate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotStorageStrategy {
    SharedReplacement,
    VersionOnlyDelta,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySnapshotVersionDelta {
    cached_versions: Arc<Vec<u64>>,
}

#[allow(dead_code)]
impl DependencySnapshotVersionDelta {
    pub fn new(cached_versions: impl Into<Vec<u64>>) -> Self {
        Self {
            cached_versions: Arc::new(cached_versions.into()),
        }
    }

    pub fn len(&self) -> usize {
        self.cached_versions.len()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencySnapshotUpdate {
    Replace(SharedDependencySnapshot),
    VersionOnly(DependencySnapshotVersionDelta),
}

#[allow(dead_code)]
impl DependencySnapshotUpdate {
    pub fn between(
        node: NodeId,
        previous: &DependencySnapshot,
        next: DependencySnapshot,
    ) -> (Self, SnapshotDeltaRecord) {
        // Compatibility-only bridge for legacy callers. The proof-bearing
        // runtime path uses `CommittedSnapshotUpdate::between(...)` instead.
        let next = next.canonicalize_unordered();
        if snapshot_shape_matches(previous, &next) {
            let cached_versions = next
                .entries()
                .iter()
                .map(|entry| entry.cached_version)
                .collect::<Vec<_>>();
            let delta = SnapshotDeltaRecord::for_version_update(node, previous, &cached_versions);
            return (
                Self::VersionOnly(DependencySnapshotVersionDelta::new(cached_versions)),
                delta,
            );
        }

        let next = SharedDependencySnapshot::new(next);
        let delta = SnapshotDeltaRecord::between(node, previous, &next);
        (Self::Replace(next), delta)
    }

    pub fn into_committed(
        self,
        node: NodeId,
        previous_snapshot_id: DependencySnapshotId,
        previous_snapshot: &DependencySnapshot,
        shape_store: &mut DependencySnapshotShapeStore,
    ) -> CommittedSnapshotUpdate {
        match self {
            Self::Replace(shared) => {
                CommittedSnapshotUpdate::Replace(ReplacementSnapshotUpdate::from_snapshot(
                    shared.into_snapshot(),
                    shape_store,
                ))
            }
            Self::VersionOnly(delta) => {
                let shape_handle = previous_snapshot.shape().intern(shape_store);
                let basis = StableShapeSnapshotBasis {
                    node,
                    previous_snapshot_id,
                    shape_handle,
                    entry_count: delta.len(),
                };
                CommittedSnapshotUpdate::VersionOnly(
                    VersionOnlySnapshotUpdate::from_basis_and_versions(
                        basis,
                        VersionVector {
                            cached_versions: delta.cached_versions,
                        },
                    ),
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotDeltaRecord {
    pub node: NodeId,
    pub change_kind: SnapshotChangeKind,
    pub previous_entry_count: u32,
    pub next_entry_count: u32,
    pub changed_entry_count: u32,
}

impl SnapshotDeltaRecord {
    pub fn between(
        node: NodeId,
        previous: &DependencySnapshot,
        next: &SharedDependencySnapshot,
    ) -> Self {
        let previous_entries = previous.entries();
        let next_entries = next.entries();
        let mut changed_entry_count = 0_u32;
        let mut previous_index = 0usize;
        let mut next_index = 0usize;

        while previous_index < previous_entries.len() && next_index < next_entries.len() {
            let previous_entry = &previous_entries[previous_index];
            let next_entry = &next_entries[next_index];
            match previous_entry.sort_key().cmp(&next_entry.sort_key()) {
                std::cmp::Ordering::Less => {
                    changed_entry_count += 1;
                    previous_index += 1;
                }
                std::cmp::Ordering::Greater => {
                    changed_entry_count += 1;
                    next_index += 1;
                }
                std::cmp::Ordering::Equal => {
                    if previous_entry.cached_version != next_entry.cached_version {
                        changed_entry_count += 1;
                    }
                    previous_index += 1;
                    next_index += 1;
                }
            }
        }

        changed_entry_count += (previous_entries.len() - previous_index) as u32;
        changed_entry_count += (next_entries.len() - next_index) as u32;

        Self {
            node,
            change_kind: if changed_entry_count == 0 && previous_entries.len() == next_entries.len()
            {
                SnapshotChangeKind::Unchanged
            } else {
                SnapshotChangeKind::StructuralReplace
            },
            previous_entry_count: previous_entries.len() as u32,
            next_entry_count: next_entries.len() as u32,
            changed_entry_count,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed_entry_count > 0 || self.previous_entry_count != self.next_entry_count
    }

    pub fn for_version_update(
        node: NodeId,
        previous: &DependencySnapshot,
        cached_versions: &[u64],
    ) -> Self {
        debug_assert_eq!(previous.entries().len(), cached_versions.len());
        Self {
            node,
            change_kind: if previous
                .entries()
                .iter()
                .zip(cached_versions.iter().copied())
                .all(|(entry, cached_version)| entry.cached_version == cached_version)
            {
                SnapshotChangeKind::Unchanged
            } else {
                SnapshotChangeKind::StableShapeVersionOnly
            },
            previous_entry_count: previous.entries().len() as u32,
            next_entry_count: cached_versions.len() as u32,
            changed_entry_count: previous
                .entries()
                .iter()
                .zip(cached_versions.iter().copied())
                .filter(|(entry, cached_version)| entry.cached_version != *cached_version)
                .count() as u32,
        }
    }
}

/// Stable handle into graph-owned dependency snapshot storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
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

    pub(crate) fn from_semantic_fingerprint(fingerprint: u32) -> Self {
        match std::num::NonZeroU32::new(fingerprint) {
            Some(non_zero) => Self(Some(non_zero)),
            None => Self(Some(std::num::NonZeroU32::new(1).expect("1 is non-zero"))),
        }
    }
}

/// Graph-owned storage for immutable dependency snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencySnapshotStore {
    snapshots: Vec<DependencySnapshot>,
    #[serde(skip, default)]
    interner: HashMap<DependencySnapshot, DependencySnapshotId>,
    #[serde(skip, default)]
    shape_handles: Vec<SnapshotShapeHandle>,
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
        let snapshot = snapshot.canonicalize_unordered();
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

    fn rebuild_shape_handles_if_needed(
        &mut self,
        shape_store: &mut DependencySnapshotShapeStore,
    ) {
        if self.shape_handles.len() == self.snapshots.len() {
            return;
        }
        self.shape_handles.clear();
        self.shape_handles.reserve(self.snapshots.len());
        for snapshot in &self.snapshots {
            self.shape_handles.push(snapshot.shape().intern(shape_store));
        }
    }

    pub fn shape_handle_for(
        &mut self,
        id: DependencySnapshotId,
        shape_store: &mut DependencySnapshotShapeStore,
    ) -> SnapshotShapeHandle {
        let Some(index) = id.index() else {
            return SnapshotShapeHandle::EMPTY;
        };
        self.rebuild_shape_handles_if_needed(shape_store);
        self.shape_handles
            .get(index - 1)
            .copied()
            .unwrap_or(SnapshotShapeHandle::EMPTY)
    }

    pub fn insert_with_shape_handle(
        &mut self,
        snapshot: DependencySnapshot,
        shape_store: &mut DependencySnapshotShapeStore,
    ) -> (DependencySnapshotId, SnapshotShapeHandle) {
        let snapshot = snapshot.canonicalize_unordered();
        if snapshot.entries().is_empty() {
            return (DependencySnapshotId::EMPTY, SnapshotShapeHandle::EMPTY);
        }
        self.rebuild_interner_if_needed();
        self.rebuild_shape_handles_if_needed(shape_store);
        if let Some(id) = self.interner.get(&snapshot).copied() {
            let handle = self
                .shape_handles
                .get(id.index().expect("snapshot id should index") - 1)
                .copied()
                .unwrap_or_else(|| snapshot.shape().intern(shape_store));
            return (id, handle);
        }
        let shape_handle = snapshot.shape().intern(shape_store);
        self.snapshots.push(snapshot);
        self.shape_handles.push(shape_handle);
        let id = DependencySnapshotId::from_index(self.snapshots.len());
        let snapshot = self.snapshots[id.index().expect("snapshot id should index") - 1].clone();
        self.interner.insert(snapshot, id);
        (id, shape_handle)
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

fn is_strict_dependency_edge_order(edges: &[DependencyEdge]) -> bool {
    edges
        .windows(2)
        .all(|pair| pair[0].sort_key() < pair[1].sort_key())
}

fn is_strict_snapshot_entry_order(entries: &[DependencySnapshotEntry]) -> bool {
    entries.windows(2).all(|pair| {
        pair[0]
            .sort_key()
            .cmp(&pair[1].sort_key())
            .then(pair[0].cached_version.cmp(&pair[1].cached_version))
            .is_lt()
    })
}

fn is_strict_snapshot_shape_order(keys: &[DependencySortKey]) -> bool {
    keys.windows(2).all(|pair| pair[0] < pair[1])
}

fn snapshot_shape_matches(previous: &DependencySnapshot, next: &DependencySnapshot) -> bool {
    let previous_entries = previous.entries();
    let next_entries = next.entries();
    previous_entries.len() == next_entries.len()
        && previous_entries
            .iter()
            .zip(next_entries.iter())
            .all(|(left, right)| left.sort_key() == right.sort_key())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::aspect::Aspect;

    #[test]
    fn version_update_delta_marks_stable_shape_change_kind() {
        let source = NodeId::new(1, 0);
        let mut snapshot = DependencySnapshot::empty();
        snapshot.record(source, Aspect::new(0), 5, None);
        snapshot.record(source, Aspect::new(1), 9, None);

        let delta = SnapshotDeltaRecord::for_version_update(NodeId::new(0, 0), &snapshot, &[7, 9]);
        assert_eq!(delta.change_kind, SnapshotChangeKind::StableShapeVersionOnly);
        assert!(delta.changed());
    }

    #[test]
    fn stable_shape_basis_proves_against_previous_snapshot_shape() {
        let source = NodeId::new(2, 0);
        let mut snapshot = DependencySnapshot::empty();
        snapshot.record(source, Aspect::new(0), 5, None);
        let scan = DependencyInputScan::stable_shape(
            NodeId::new(0, 0),
            DependencySnapshotId::EMPTY,
            1,
            1,
            vec![8],
        );
        let mut store = DependencySnapshotShapeStore::default();
        let proof = StableShapeSnapshotBasis::prove(&scan, snapshot.shape().intern(&mut store))
            .expect("stable-shape scan should produce a proof");

        assert_eq!(proof.entry_count(), 1);
        assert_ne!(proof.shape_handle(), SnapshotShapeHandle::EMPTY);
    }
}
