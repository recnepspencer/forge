use crate::authority::AuthoritativeExportBundle;
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SNAPSHOT_FAMILY_VERSION: u32 = 1;
pub const SNAPSHOT_BASIS_VERSION: u32 = 1;
pub const SNAPSHOT_IMAGE_FORMAT_VERSION: u32 = 1;

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
    pub snapshot_family_version: u32,
    pub snapshot_basis_version: u32,
    pub snapshot_image_format_version: u32,
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
    pub snapshot_authority_digest: String,
    pub snapshot_image_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotImageBundle {
    snapshot_family_version: u32,
    snapshot_basis_version: u32,
    snapshot_image_format_version: u32,
    authoritative_export: AuthoritativeExportBundle,
}

impl SnapshotImageBundle {
    pub fn new(authoritative_export: AuthoritativeExportBundle) -> Self {
        Self {
            snapshot_family_version: SNAPSHOT_FAMILY_VERSION,
            snapshot_basis_version: SNAPSHOT_BASIS_VERSION,
            snapshot_image_format_version: SNAPSHOT_IMAGE_FORMAT_VERSION,
            authoritative_export: authoritative_export.into_canonicalized(),
        }
    }

    pub fn snapshot_family_version(&self) -> u32 {
        self.snapshot_family_version
    }

    pub fn snapshot_basis_version(&self) -> u32 {
        self.snapshot_basis_version
    }

    pub fn snapshot_image_format_version(&self) -> u32 {
        self.snapshot_image_format_version
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
pub struct SnapshotRestoreRequest {
    pub snapshot_id: SnapshotId,
    pub target_commit_id: CommitId,
}

impl SnapshotRestoreRequest {
    pub fn new(snapshot_id: SnapshotId, target_commit_id: CommitId) -> Self {
        Self {
            snapshot_id,
            target_commit_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestorePlan {
    snapshot_id: SnapshotId,
    snapshot_branch_id: BranchId,
    target_commit_id: CommitId,
    tail_commit_ids: Vec<CommitId>,
}

impl SnapshotRestorePlan {
    pub(crate) fn new(
        snapshot_id: SnapshotId,
        snapshot_branch_id: BranchId,
        target_commit_id: CommitId,
        tail_commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            snapshot_id,
            snapshot_branch_id,
            target_commit_id,
            tail_commit_ids,
        }
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.snapshot_id
    }

    pub fn snapshot_branch_id(&self) -> &BranchId {
        &self.snapshot_branch_id
    }

    pub fn target_commit_id(&self) -> CommitId {
        self.target_commit_id
    }

    pub fn tail_commit_ids(&self) -> &[CommitId] {
        &self.tail_commit_ids
    }
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

pub fn stable_snapshot_basis_authority_digest(
    branch_id: &BranchId,
    frontier_commit_id: CommitId,
    history_range: &[CommitId],
    canonicalization_version: u32,
) -> String {
    stable_snapshot_digest(&(
        branch_id,
        frontier_commit_id,
        history_range,
        canonicalization_version,
    ))
}
