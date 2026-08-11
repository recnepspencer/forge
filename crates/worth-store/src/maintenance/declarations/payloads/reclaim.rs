use serde::Serialize;

use worth_relational::facade::history::{BranchId, CommitId};

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
