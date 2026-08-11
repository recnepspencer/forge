use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use super::{
    DependencySnapshot, DependencySnapshotId, DependencySnapshotShapeStore,
    SharedDependencySnapshot, SnapshotDeltaRecord, SnapshotShapeHandle, StableShapeSnapshotBasis,
    VersionOnlySnapshotUpdate, VersionVector,
};

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
            Self::VersionOnly(update) => SharedDependencySnapshot::new(
                previous.with_updated_versions(update.versions().as_slice()),
            ),
            Self::Replace(update) => update.snapshot,
        }
    }

    pub fn change_kind(&self) -> super::SnapshotChangeKind {
        match self {
            Self::VersionOnly(_) => super::SnapshotChangeKind::StableShapeVersionOnly,
            Self::Replace(_) => super::SnapshotChangeKind::StructuralReplace,
        }
    }
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
        // The proof-bearing runtime path uses `CommittedSnapshotUpdate::between(...)`.
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
            Self::Replace(shared) => CommittedSnapshotUpdate::Replace(
                ReplacementSnapshotUpdate::from_snapshot(shared.into_snapshot(), shape_store),
            ),
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

fn snapshot_shape_matches(previous: &DependencySnapshot, next: &DependencySnapshot) -> bool {
    let previous_entries = previous.entries();
    let next_entries = next.entries();
    previous_entries.len() == next_entries.len()
        && previous_entries
            .iter()
            .zip(next_entries.iter())
            .all(|(left, right)| left.sort_key() == right.sort_key())
}
