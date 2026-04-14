use crate::authority::AuthoritativeExportBundle;
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SnapshotId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotCaptureRequest {
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
}

impl SnapshotCaptureRequest {
    pub fn new(snapshot_branch_id: BranchId, snapshot_frontier_commit_id: CommitId) -> Self {
        Self {
            snapshot_branch_id,
            snapshot_frontier_commit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotReadMode {
    PureSnapshot,
    SnapshotPlusTail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotReadRequest {
    pub snapshot_id: SnapshotId,
    pub target_commit_id: CommitId,
    pub mode: SnapshotReadMode,
}

impl SnapshotReadRequest {
    pub fn pure_snapshot(snapshot_id: SnapshotId, target_commit_id: CommitId) -> Self {
        Self {
            snapshot_id,
            target_commit_id,
            mode: SnapshotReadMode::PureSnapshot,
        }
    }

    pub fn snapshot_plus_tail(snapshot_id: SnapshotId, target_commit_id: CommitId) -> Self {
        Self {
            snapshot_id,
            target_commit_id,
            mode: SnapshotReadMode::SnapshotPlusTail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedSnapshotHandle {
    pub snapshot_id: SnapshotId,
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
    pub snapshot_authority_digest: String,
    pub snapshot_image_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotImageBundle {
    authoritative_export: AuthoritativeExportBundle,
}

impl SnapshotImageBundle {
    pub fn new(authoritative_export: AuthoritativeExportBundle) -> Self {
        Self {
            authoritative_export: authoritative_export.into_canonicalized(),
        }
    }

    pub fn authoritative_export(&self) -> &AuthoritativeExportBundle {
        &self.authoritative_export
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("snapshot image serialization")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotReadResult {
    pub snapshot_id: SnapshotId,
    pub target_commit_id: CommitId,
    pub mode: SnapshotReadMode,
    pub image: SnapshotImageBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestoreOutcome {
    pub snapshot_id: SnapshotId,
    pub restored_branch_id: BranchId,
    pub restored_frontier_commit_id: CommitId,
    pub restored_image: SnapshotImageBundle,
}

pub fn stable_snapshot_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("snapshot digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
