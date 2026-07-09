#![allow(dead_code)]

use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedHeadSet {
    branch_ids: Vec<BranchId>,
}

impl RetainedHeadSet {
    pub(crate) fn new(mut branch_ids: Vec<BranchId>) -> Self {
        branch_ids.sort();
        branch_ids.dedup();
        Self { branch_ids }
    }

    pub fn branch_ids(&self) -> &[BranchId] {
        &self.branch_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StableBasisSet {
    basis_labels: Vec<String>,
}

impl StableBasisSet {
    pub(crate) fn new(basis_labels: Vec<String>) -> Self {
        let mut basis_labels = basis_labels
            .into_iter()
            .map(|label| label.trim().to_string())
            .filter(|label| !label.is_empty())
            .collect::<Vec<_>>();
        basis_labels.sort();
        basis_labels.dedup();
        Self { basis_labels }
    }

    pub fn basis_labels(&self) -> &[String] {
        &self.basis_labels
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionClosureWitness {
    retained_heads: RetainedHeadSet,
    stable_bases: StableBasisSet,
    closure_commit_ids: Vec<CommitId>,
    frontier_commit_ids: Vec<CommitId>,
}

impl RetentionClosureWitness {
    pub(crate) fn new(
        retained_heads: RetainedHeadSet,
        stable_bases: StableBasisSet,
        mut closure_commit_ids: Vec<CommitId>,
        mut frontier_commit_ids: Vec<CommitId>,
    ) -> Self {
        closure_commit_ids.sort();
        closure_commit_ids.dedup();
        frontier_commit_ids.sort();
        frontier_commit_ids.dedup();
        Self {
            retained_heads,
            stable_bases,
            closure_commit_ids,
            frontier_commit_ids,
        }
    }

    pub fn retained_heads(&self) -> &RetainedHeadSet {
        &self.retained_heads
    }

    pub fn stable_bases(&self) -> &StableBasisSet {
        &self.stable_bases
    }

    pub fn closure_commit_ids(&self) -> &[CommitId] {
        &self.closure_commit_ids
    }

    pub fn frontier_commit_ids(&self) -> &[CommitId] {
        &self.frontier_commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyExpiredAuthorityRange {
    branch_id: BranchId,
    oldest_retained_commit_id: Option<CommitId>,
    expired_commit_ids: Vec<CommitId>,
}

impl PolicyExpiredAuthorityRange {
    pub(crate) fn new(
        branch_id: BranchId,
        oldest_retained_commit_id: Option<CommitId>,
        mut expired_commit_ids: Vec<CommitId>,
    ) -> Self {
        expired_commit_ids.sort();
        expired_commit_ids.dedup();
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
pub struct CompactionCutoverWitness {
    retained_basis_label: String,
    compaction_product_id: String,
}

impl CompactionCutoverWitness {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        compaction_product_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            compaction_product_id: compaction_product_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn compaction_product_id(&self) -> &str {
        &self.compaction_product_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReclaimEligibilityWitness {
    artifact_family: String,
    artifact_id: String,
    retained_basis_label: String,
}

impl ReclaimEligibilityWitness {
    pub(crate) fn new(
        artifact_family: impl Into<String>,
        artifact_id: impl Into<String>,
        retained_basis_label: impl Into<String>,
    ) -> Self {
        Self {
            artifact_family: artifact_family.into(),
            artifact_id: artifact_id.into(),
            retained_basis_label: retained_basis_label.into(),
        }
    }

    pub fn artifact_family(&self) -> &str {
        &self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BasisSurvivalVerdict {
    basis_label: String,
    survives: bool,
    reason: Option<String>,
}

impl BasisSurvivalVerdict {
    pub(crate) fn survives(basis_label: impl Into<String>) -> Self {
        Self {
            basis_label: basis_label.into(),
            survives: true,
            reason: None,
        }
    }

    pub(crate) fn expires(basis_label: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            basis_label: basis_label.into(),
            survives: false,
            reason: Some(reason.into()),
        }
    }

    pub fn basis_label(&self) -> &str {
        &self.basis_label
    }

    pub fn survives_basis(&self) -> bool {
        self.survives
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_head_and_stable_basis_sets_deduplicate() {
        let heads = RetainedHeadSet::new(vec![
            BranchId("main".to_string()),
            BranchId("main".to_string()),
            BranchId("feature".to_string()),
        ]);
        let bases = StableBasisSet::new(vec![
            "basis-b".to_string(),
            "basis-a".to_string(),
            "basis-a".to_string(),
        ]);

        assert_eq!(heads.branch_ids().len(), 2);
        assert_eq!(
            bases.basis_labels(),
            &["basis-a".to_string(), "basis-b".to_string()]
        );
    }
}
