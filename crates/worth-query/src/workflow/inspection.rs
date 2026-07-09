use super::*;
use crate::evidence_identity::WorthQueryEvidenceIdentity;

mod identities;
mod operations;
pub use operations::{
    build_workflow_replay_bundle, inspect_merge_conflicts, inspect_post_merge_outcome,
    shape_merge_authority_outcome, shape_mutation_authority_outcome,
    shape_writeback_authority_outcome,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowStalenessOutcome {
    StillFresh,
    StaleDenied,
    ExplicitRebindRequired,
}

impl WorkflowStalenessOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StillFresh => "still_fresh",
            Self::StaleDenied => "stale_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowExplicitRebindArtifact {
    declaration_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    rebind_reason: &'static str,
    digest: String,
}

impl WorkflowExplicitRebindArtifact {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn rebind_reason(&self) -> &'static str {
        self.rebind_reason
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowInspectionFailureClass {
    UnsupportedInspectionFamily,
    RelationalInspectionMismatch,
    NonAuthoritativeOutcomeForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowInspectionError {
    failure_class: WorkflowInspectionFailureClass,
    message: &'static str,
    counters: WorkflowInspectionCounters,
}

impl WorkflowInspectionError {
    fn new(
        failure_class: WorkflowInspectionFailureClass,
        message: &'static str,
        counters: WorkflowInspectionCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> &WorkflowInspectionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConflictInspectionFamily {
    MergeWorkflowNarrow,
}

impl ConflictInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MergeWorkflowNarrow => "merge_workflow_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeClassAdmission {
    ExecutionAdmissible,
    ExecutionDenied,
}

impl MergeClassAdmission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExecutionAdmissible => "execution_admissible",
            Self::ExecutionDenied => "execution_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictInspectionRow {
    workflow_basis_digest: String,
    merge_class: String,
    merge_class_admission: MergeClassAdmission,
    target_basis_digest: String,
    source_basis_digest: String,
    conflict_scope_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
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
    declaration_digest: String,
    family: ConflictInspectionFamily,
    budget: WorkflowInspectionBudget,
    prediction_report: WorkflowPredictionReport,
    drift_outcome: WorkflowPredictionDriftOutcome,
    rows: Vec<ConflictInspectionRow>,
    counters: WorkflowInspectionCounters,
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PostMergeInspectionFamily {
    AuthoritativeOutcomeNarrow,
}

impl PostMergeInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeOutcomeNarrow => "authoritative_outcome_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostMergeInspectionRow {
    authoritative_outcome_basis_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    authoritative_commit_or_outcome_digest: String,
    post_merge_scope_digest: String,
    merge_or_writeback_origin_digest: String,
    inspection_result_family: String,
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
    origin_digest: String,
    family: PostMergeInspectionFamily,
    budget: WorkflowInspectionBudget,
    prediction_report: WorkflowPredictionReport,
    drift_outcome: WorkflowPredictionDriftOutcome,
    rows: Vec<PostMergeInspectionRow>,
    counters: WorkflowInspectionCounters,
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowAuthorityOutcomeFamily {
    MutationLoweringAdmitted,
    MergeLoweringAdmitted,
    WritebackLoweringAdmitted,
}

impl WorkflowAuthorityOutcomeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationLoweringAdmitted => "mutation_lowering_admitted",
            Self::MergeLoweringAdmitted => "merge_lowering_admitted",
            Self::WritebackLoweringAdmitted => "writeback_lowering_admitted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowAuthorityOutcomeArtifact {
    family: WorkflowAuthorityOutcomeFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    source_query_identity: WorthQueryEvidenceIdentity,
    source_plan_identity: WorthQueryEvidenceIdentity,
    source_basis_identity: WorthQueryEvidenceIdentity,
    source_declaration_identity: WorthQueryEvidenceIdentity,
    authority_request_identity: WorthQueryEvidenceIdentity,
    authoritative_outcome_identity: WorthQueryEvidenceIdentity,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    budget_outcome: WorkflowBudgetOutcome,
    prediction_report: WorkflowPredictionReport,
    prediction_drift_outcome: WorkflowPredictionDriftOutcome,
    freshness_outcome: WorkflowStalenessOutcome,
    explicit_rebind: Option<WorkflowExplicitRebindArtifact>,
    realized_width: usize,
    counters: WorkflowLoweringCounters,
}

impl WorkflowAuthorityOutcomeArtifact {
    pub fn family(&self) -> &WorkflowAuthorityOutcomeFamily {
        &self.family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn source_query_digest(&self) -> &str {
        self.source_query_identity.as_str()
    }

    pub fn source_query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_query_identity
    }

    pub fn source_plan_digest(&self) -> &str {
        self.source_plan_identity.as_str()
    }

    pub fn source_plan_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_plan_identity
    }

    pub fn source_basis_digest(&self) -> &str {
        self.source_basis_identity.as_str()
    }

    pub fn source_basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_basis_identity
    }

    pub fn source_declaration_digest(&self) -> &str {
        self.source_declaration_identity.as_str()
    }

    pub fn source_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_declaration_identity
    }

    pub fn authority_request_digest(&self) -> &str {
        self.authority_request_identity.as_str()
    }

    pub fn authority_request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authority_request_identity
    }

    pub fn authoritative_outcome_digest(&self) -> &str {
        self.authoritative_outcome_identity.as_str()
    }

    pub fn authoritative_outcome_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authoritative_outcome_identity
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn budget_outcome(&self) -> &WorkflowBudgetOutcome {
        &self.budget_outcome
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn prediction_drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.prediction_drift_outcome
    }

    pub fn freshness_outcome(&self) -> &WorkflowStalenessOutcome {
        &self.freshness_outcome
    }

    pub fn explicit_rebind(&self) -> Option<&WorkflowExplicitRebindArtifact> {
        self.explicit_rebind.as_ref()
    }

    pub fn realized_width(&self) -> usize {
        self.realized_width
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowReplayBundle {
    bundle_identity: WorthQueryEvidenceIdentity,
    query_identity: WorthQueryEvidenceIdentity,
    plan_identity: WorthQueryEvidenceIdentity,
    basis_identity: WorthQueryEvidenceIdentity,
    declaration_identity: WorthQueryEvidenceIdentity,
    authority_target_family: WorkflowAuthorityTargetFamily,
    authority_request_identity: WorthQueryEvidenceIdentity,
    authoritative_outcome_identity: WorthQueryEvidenceIdentity,
    delivery_or_failure_identity: WorthQueryEvidenceIdentity,
    counters: WorkflowLoweringCounters,
}

impl WorkflowReplayBundle {
    pub fn bundle_digest(&self) -> &str {
        self.bundle_identity.as_str()
    }

    pub fn bundle_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bundle_identity
    }

    pub fn query_digest(&self) -> &str {
        self.query_identity.as_str()
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn plan_digest(&self) -> &str {
        self.plan_identity.as_str()
    }

    pub fn plan_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.plan_identity
    }

    pub fn basis_digest(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_identity.as_str()
    }

    pub fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.declaration_identity
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn authority_request_digest(&self) -> &str {
        self.authority_request_identity.as_str()
    }

    pub fn authority_request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authority_request_identity
    }

    pub fn authoritative_outcome_digest(&self) -> &str {
        self.authoritative_outcome_identity.as_str()
    }

    pub fn authoritative_outcome_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authoritative_outcome_identity
    }

    pub fn delivery_or_failure_digest(&self) -> &str {
        self.delivery_or_failure_identity.as_str()
    }

    pub fn delivery_or_failure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_or_failure_identity
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}
