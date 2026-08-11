use serde::Serialize;

use worth_relational::facade::history::{BranchId, CommitId};

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
