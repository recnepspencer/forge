use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;
use crate::identity::data::VersionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotReadPolicy {
    ImmutablePinned,
    ImmutablePinnedNoLazyMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotHandle {
    pub runtime_instance_id: u64,
    pub branch_id: BranchId,
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub read_policy: SnapshotReadPolicy,
}

impl SnapshotHandle {
    pub fn new(snapshot_id: u64, version_id: u64, branch_id: BranchId) -> Self {
        Self {
            runtime_instance_id: 0,
            branch_id,
            snapshot_id: SnapshotId(snapshot_id),
            version_id: VersionId(version_id),
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotInspectionSummary {
    pub version_id: VersionId,
    pub entity_count: usize,
    pub relation_count: usize,
    pub pinned_entity_count: usize,
    pub pinned_relation_count: usize,
}
