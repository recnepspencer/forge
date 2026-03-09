use serde::{Deserialize, Serialize};

use crate::data::identity::VersionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotReadPolicy {
    ImmutablePinned,
    ImmutablePinnedNoLazyMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotHandle {
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub read_policy: SnapshotReadPolicy,
}

impl SnapshotHandle {
    pub const fn new(snapshot_id: u64, version_id: u64) -> Self {
        Self {
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
}
