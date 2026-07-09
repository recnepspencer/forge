#![forbid(unsafe_code)]

use worth_store_contracts::StableArtifactId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotId(StableArtifactId);

impl SnapshotId {
    pub const fn from_artifact_id(id: StableArtifactId) -> Self {
        Self(id)
    }
}
