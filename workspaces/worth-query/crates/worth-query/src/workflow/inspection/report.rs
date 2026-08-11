use super::super::foundation::{WorkflowAuthorityTargetFamily, WorkflowPredictionDriftOutcome};
use super::super::performance::{
    WorkflowInspectionBudget, WorkflowInspectionCounters, WorkflowPredictionReport,
};
use super::model::{ConflictInspectionFamily, MergeClassAdmission, PostMergeInspectionFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictInspectionRow {
    pub(super) workflow_basis_digest: String,
    pub(super) merge_class: String,
    pub(super) merge_class_admission: MergeClassAdmission,
    pub(super) target_basis_digest: String,
    pub(super) source_basis_digest: String,
    pub(super) conflict_scope_digest: String,
    pub(super) authority_target_family: WorkflowAuthorityTargetFamily,
}

impl ConflictInspectionRow {
    pub fn workflow_basis_digest(&self) -> &str {
        &self.workflow_basis_digest
    }

    pub fn merge_class(&self) -> &str {
        &self.merge_class
    }

    pub fn merge_class_admission(&self) -> &MergeClassAdmission {
        &self.merge_class_admission
    }

    pub fn target_basis_digest(&self) -> &str {
        &self.target_basis_digest
    }

    pub fn source_basis_digest(&self) -> &str {
        &self.source_basis_digest
    }

    pub fn conflict_scope_digest(&self) -> &str {
        &self.conflict_scope_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryConflictInspectionArtifact {
    pub(super) declaration_digest: String,
    pub(super) family: ConflictInspectionFamily,
    pub(super) budget: WorkflowInspectionBudget,
    pub(super) prediction_report: WorkflowPredictionReport,
    pub(super) drift_outcome: WorkflowPredictionDriftOutcome,
    pub(super) rows: Vec<ConflictInspectionRow>,
    pub(super) counters: WorkflowInspectionCounters,
}

impl QueryConflictInspectionArtifact {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn family(&self) -> &ConflictInspectionFamily {
        &self.family
    }

    pub fn budget(&self) -> &WorkflowInspectionBudget {
        &self.budget
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn rows(&self) -> &[ConflictInspectionRow] {
        &self.rows
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostMergeInspectionRow {
    pub(super) authoritative_outcome_basis_digest: String,
    pub(super) authority_target_family: WorkflowAuthorityTargetFamily,
    pub(super) authoritative_commit_or_outcome_digest: String,
    pub(super) post_merge_scope_digest: String,
    pub(super) merge_or_writeback_origin_digest: String,
    pub(super) inspection_result_family: String,
}

impl PostMergeInspectionRow {
    pub fn authoritative_outcome_basis_digest(&self) -> &str {
        &self.authoritative_outcome_basis_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn authoritative_commit_or_outcome_digest(&self) -> &str {
        &self.authoritative_commit_or_outcome_digest
    }

    pub fn post_merge_scope_digest(&self) -> &str {
        &self.post_merge_scope_digest
    }

    pub fn merge_or_writeback_origin_digest(&self) -> &str {
        &self.merge_or_writeback_origin_digest
    }

    pub fn inspection_result_family(&self) -> &str {
        &self.inspection_result_family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPostMergeInspectionArtifact {
    pub(super) origin_digest: String,
    pub(super) family: PostMergeInspectionFamily,
    pub(super) budget: WorkflowInspectionBudget,
    pub(super) prediction_report: WorkflowPredictionReport,
    pub(super) drift_outcome: WorkflowPredictionDriftOutcome,
    pub(super) rows: Vec<PostMergeInspectionRow>,
    pub(super) counters: WorkflowInspectionCounters,
}

impl QueryPostMergeInspectionArtifact {
    pub fn origin_digest(&self) -> &str {
        &self.origin_digest
    }

    pub fn family(&self) -> &PostMergeInspectionFamily {
        &self.family
    }

    pub fn budget(&self) -> &WorkflowInspectionBudget {
        &self.budget
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn rows(&self) -> &[PostMergeInspectionRow] {
        &self.rows
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}
