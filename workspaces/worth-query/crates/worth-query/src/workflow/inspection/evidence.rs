use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::foundation::{
    WorkflowAuthorityTargetFamily, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowPredictionDriftOutcome,
};
use super::super::performance::{
    WorkflowBudgetOutcome, WorkflowLoweringCounters, WorkflowPredictionReport,
};
use super::model::{
    WorkflowAuthorityOutcomeFamily, WorkflowExplicitRebindArtifact, WorkflowStalenessOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowAuthorityOutcomeArtifact {
    pub(super) family: WorkflowAuthorityOutcomeFamily,
    pub(super) authority_target_family: WorkflowAuthorityTargetFamily,
    pub(super) source_query_identity: WorthQueryEvidenceIdentity,
    pub(super) source_plan_identity: WorthQueryEvidenceIdentity,
    pub(super) source_basis_identity: WorthQueryEvidenceIdentity,
    pub(super) source_declaration_identity: WorthQueryEvidenceIdentity,
    pub(super) authority_request_identity: WorthQueryEvidenceIdentity,
    pub(super) authoritative_outcome_identity: WorthQueryEvidenceIdentity,
    pub(super) cost_class: WorkflowCostClass,
    pub(super) budget_class: WorkflowBudgetClass,
    pub(super) budget_outcome: WorkflowBudgetOutcome,
    pub(super) prediction_report: WorkflowPredictionReport,
    pub(super) prediction_drift_outcome: WorkflowPredictionDriftOutcome,
    pub(super) freshness_outcome: WorkflowStalenessOutcome,
    pub(super) explicit_rebind: Option<WorkflowExplicitRebindArtifact>,
    pub(super) realized_width: usize,
    pub(super) counters: WorkflowLoweringCounters,
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
    pub(super) bundle_identity: WorthQueryEvidenceIdentity,
    pub(super) query_identity: WorthQueryEvidenceIdentity,
    pub(super) plan_identity: WorthQueryEvidenceIdentity,
    pub(super) basis_identity: WorthQueryEvidenceIdentity,
    pub(super) declaration_identity: WorthQueryEvidenceIdentity,
    pub(super) authority_target_family: WorkflowAuthorityTargetFamily,
    pub(super) authority_request_identity: WorthQueryEvidenceIdentity,
    pub(super) authoritative_outcome_identity: WorthQueryEvidenceIdentity,
    pub(super) delivery_or_failure_identity: WorthQueryEvidenceIdentity,
    pub(super) counters: WorkflowLoweringCounters,
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
