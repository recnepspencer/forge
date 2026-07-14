use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactionProductRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub retained_basis_label: String,
    pub compacted_family_labels: Vec<String>,
    pub product_digest: String,
    #[serde(default)]
    pub closure_record_artifact_id: String,
    #[serde(default)]
    pub basis_record_artifact_ids: Vec<String>,
    #[serde(default)]
    pub rewritten_range_count: u64,
    #[serde(default)]
    pub superseded_families: Vec<String>,
    #[serde(default)]
    pub superseded_artifact_ids: Vec<String>,
    #[serde(default)]
    pub parity_verified: bool,
    #[serde(default)]
    pub cutover_committed: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionBasisRecord {
    pub artifact_id: String,
    pub basis_label: String,
    pub branch_id: Option<BranchId>,
    pub basis_commit_id: Option<CommitId>,
    #[serde(default)]
    pub family_version: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionClosureRecord {
    pub artifact_id: String,
    pub retained_basis_label: String,
    pub retained_head_branch_ids: Vec<BranchId>,
    pub stable_basis_labels: Vec<String>,
    pub closure_commit_ids: Vec<CommitId>,
    pub frontier_commit_ids: Vec<CommitId>,
    #[serde(default)]
    pub family_version: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebuildDebtRecord {
    pub artifact_id: String,
    pub family_label: String,
    pub retained_basis_label: String,
    pub rebuild_target_id: String,
    pub debt_reason: String,
    #[serde(default)]
    pub family_version: u32,
    #[serde(default)]
    pub cleared: bool,
}
