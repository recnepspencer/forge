use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::data::RelationalRuntimeProfile;
use crate::history::data::{CommitId, RelationalCommitReceipt};
use crate::identity::data::VersionId;
use crate::schema::data::SchemaVersionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurabilityMode {
    InMemoryCanonical,
    PersistedSegmentedLocalFs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableStoreLayout {
    pub root_path: PathBuf,
    pub segment_commit_capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DurableSegmentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DurableCheckpointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurableIntegrityStatus {
    Verified,
    Corrupt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointCoverage {
    pub up_to_commit: Option<RelationalCommitReceipt>,
    pub up_to_version: Option<VersionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableSegmentManifest {
    pub segment_id: DurableSegmentId,
    pub path: PathBuf,
    pub first_commit_id: Option<CommitId>,
    pub last_commit_id: Option<CommitId>,
    pub commit_count: usize,
    pub runtime_name: String,
    pub profile: RelationalRuntimeProfile,
    pub schema_version: SchemaVersionId,
    pub integrity: DurableIntegrityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpointManifest {
    pub checkpoint_id: DurableCheckpointId,
    pub path: PathBuf,
    pub coverage: CheckpointCoverage,
    pub partition_count: usize,
    pub runtime_name: String,
    pub profile: RelationalRuntimeProfile,
    pub schema_version: SchemaVersionId,
    pub integrity: DurableIntegrityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableStore {
    pub layout: DurableStoreLayout,
    pub segments: Vec<DurableSegmentManifest>,
    pub checkpoints: Vec<DurableCheckpointManifest>,
}
