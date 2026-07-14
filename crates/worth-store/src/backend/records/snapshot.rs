use crate::snapshot::{SnapshotId, SnapshotImageBundle};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotBasisRecord {
    pub snapshot_id: SnapshotId,
    pub snapshot_family_version: u32,
    pub snapshot_basis_version: u32,
    pub snapshot_image_format_version: u32,
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
    pub snapshot_history_range: Vec<CommitId>,
    pub snapshot_canonicalization_version: u32,
    pub snapshot_authority_digest: String,
    pub snapshot_image_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotImageRecord {
    pub snapshot_id: SnapshotId,
    pub image: SnapshotImageBundle,
}
