use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceDeclarationClass {
    Retention,
    Compaction,
    Reclaim,
    Rebuild,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionMaintenanceDeclaration {
    batch_label: String,
    closure_commit_count: u64,
    declaration_count: u64,
}

impl RetentionMaintenanceDeclaration {
    pub(crate) fn new(
        batch_label: impl Into<String>,
        closure_commit_count: u64,
        declaration_count: u64,
    ) -> Self {
        Self {
            batch_label: batch_label.into(),
            closure_commit_count,
            declaration_count,
        }
    }

    pub fn batch_label(&self) -> &str {
        &self.batch_label
    }

    pub fn closure_commit_count(&self) -> u64 {
        self.closure_commit_count
    }

    pub fn declaration_count(&self) -> u64 {
        self.declaration_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionMaintenanceDeclaration {
    retained_basis_label: String,
    retained_head_branch_ids: Vec<BranchId>,
    stable_basis_labels: Vec<String>,
    closure_commit_ids: Vec<CommitId>,
    frontier_commit_ids: Vec<CommitId>,
    family_labels: Vec<String>,
    superseded_families: Vec<(String, String, Option<CommitId>)>,
    rewritten_range_count: u64,
}

impl CompactionMaintenanceDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        retained_head_branch_ids: Vec<BranchId>,
        stable_basis_labels: Vec<String>,
        closure_commit_ids: Vec<CommitId>,
        frontier_commit_ids: Vec<CommitId>,
        family_labels: Vec<String>,
        superseded_families: Vec<(String, String, Option<CommitId>)>,
        rewritten_range_count: u64,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            retained_head_branch_ids,
            stable_basis_labels,
            closure_commit_ids,
            frontier_commit_ids,
            family_labels,
            superseded_families,
            rewritten_range_count,
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }
    pub fn retained_head_branch_ids(&self) -> &[BranchId] {
        &self.retained_head_branch_ids
    }
    pub fn stable_basis_labels(&self) -> &[String] {
        &self.stable_basis_labels
    }
    pub fn closure_commit_ids(&self) -> &[CommitId] {
        &self.closure_commit_ids
    }
    pub fn frontier_commit_ids(&self) -> &[CommitId] {
        &self.frontier_commit_ids
    }
    pub fn family_labels(&self) -> &[String] {
        &self.family_labels
    }
    pub fn superseded_families(&self) -> &[(String, String, Option<CommitId>)] {
        &self.superseded_families
    }
    pub fn rewritten_range_count(&self) -> u64 {
        self.rewritten_range_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReclaimMaintenanceDeclaration {
    retained_basis_label: String,
    artifact_family: String,
    artifact_id: String,
}

impl ReclaimMaintenanceDeclaration {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        artifact_family: impl Into<String>,
        artifact_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            artifact_family: artifact_family.into(),
            artifact_id: artifact_id.into(),
        }
    }
    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }
    pub fn artifact_family(&self) -> &str {
        &self.artifact_family
    }
    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeReclaimMaintenanceDeclaration {
    branch_id: BranchId,
    oldest_retained_commit_id: Option<CommitId>,
    expired_commit_ids: Vec<CommitId>,
}

impl AuthoritativeReclaimMaintenanceDeclaration {
    pub(crate) fn new(
        branch_id: BranchId,
        oldest_retained_commit_id: Option<CommitId>,
        expired_commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            oldest_retained_commit_id,
            expired_commit_ids,
        }
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn oldest_retained_commit_id(&self) -> Option<CommitId> {
        self.oldest_retained_commit_id
    }
    pub fn expired_commit_ids(&self) -> &[CommitId] {
        &self.expired_commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RebuildMaintenanceDeclaration {
    retained_basis_label: String,
    family_label: String,
    rebuild_target_id: String,
    debt_link_artifact_id: Option<String>,
}

impl RebuildMaintenanceDeclaration {
    #[allow(dead_code)]
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
        debt_link_artifact_id: Option<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
            debt_link_artifact_id,
        }
    }
    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }
    pub fn family_label(&self) -> &str {
        &self.family_label
    }
    pub fn rebuild_target_id(&self) -> &str {
        &self.rebuild_target_id
    }
    pub fn debt_link_artifact_id(&self) -> Option<&str> {
        self.debt_link_artifact_id.as_deref()
    }
}
