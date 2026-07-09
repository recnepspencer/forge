use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use crate::retention::plans::RetentionClosureSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredRetentionMaintenanceBatch {
    batch_label: String,
    closure_summary: RetentionClosureSummary,
    compaction_declarations: Vec<LoweredCompactionDeclaration>,
    reclaim_declarations: Vec<LoweredReclaimDeclaration>,
    rebuild_declarations: Vec<LoweredRebuildDeclaration>,
}

impl LoweredRetentionMaintenanceBatch {
    pub(crate) fn new(
        batch_label: impl Into<String>,
        closure_summary: RetentionClosureSummary,
        compaction_declarations: Vec<LoweredCompactionDeclaration>,
        reclaim_declarations: Vec<LoweredReclaimDeclaration>,
        rebuild_declarations: Vec<LoweredRebuildDeclaration>,
    ) -> Self {
        Self {
            batch_label: batch_label.into(),
            closure_summary,
            compaction_declarations,
            reclaim_declarations,
            rebuild_declarations,
        }
    }

    pub fn batch_label(&self) -> &str {
        &self.batch_label
    }
    pub fn closure_summary(&self) -> &RetentionClosureSummary {
        &self.closure_summary
    }
    pub fn compaction_declarations(&self) -> &[LoweredCompactionDeclaration] {
        &self.compaction_declarations
    }
    pub fn reclaim_declarations(&self) -> &[LoweredReclaimDeclaration] {
        &self.reclaim_declarations
    }
    pub fn rebuild_declarations(&self) -> &[LoweredRebuildDeclaration] {
        &self.rebuild_declarations
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredCompactionDeclaration {
    retained_basis_label: String,
    retained_head_branch_ids: Vec<BranchId>,
    stable_basis_labels: Vec<String>,
    closure_commit_ids: Vec<CommitId>,
    frontier_commit_ids: Vec<CommitId>,
    family_labels: Vec<String>,
    superseded_families: Vec<(String, String, Option<CommitId>)>,
    rewritten_range_count: u64,
}

impl LoweredCompactionDeclaration {
    pub(crate) fn new(
        retained_basis_label: String,
        retained_head_branch_ids: Vec<BranchId>,
        stable_basis_labels: Vec<String>,
        closure_commit_ids: Vec<CommitId>,
        frontier_commit_ids: Vec<CommitId>,
        family_labels: Vec<String>,
        superseded_families: Vec<(String, String, Option<CommitId>)>,
        rewritten_range_count: u64,
    ) -> Self {
        Self {
            retained_basis_label,
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
pub enum LoweredReclaimDeclaration {
    Derived {
        retained_basis_label: String,
        artifact_family: String,
        artifact_id: String,
    },
    Authoritative {
        branch_id: BranchId,
        oldest_retained_commit_id: Option<CommitId>,
        expired_commit_ids: Vec<CommitId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredRebuildDeclaration {
    retained_basis_label: String,
    family_label: String,
    rebuild_target_id: String,
    debt_reason: String,
}

impl LoweredRebuildDeclaration {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
        debt_reason: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
            debt_reason: debt_reason.into(),
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
    pub fn debt_reason(&self) -> &str {
        &self.debt_reason
    }
}
