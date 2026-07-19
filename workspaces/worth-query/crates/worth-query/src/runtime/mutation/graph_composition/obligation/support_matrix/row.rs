use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationExecutionCostClass,
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSupportMatrixRow {
    obligation_kind: WorthQueryGraphObligationKind,
    support_lane: WorthQueryGraphObligationSupportLane,
    status: WorthQueryGraphObligationSupportStatus,
    execution_budget: WorthQueryGraphObligationExecutionBudget,
    cost_class: WorthQueryGraphObligationExecutionCostClass,
    state_load_counter_policy: &'static str,
    diagnostic_artifact_policy: &'static str,
    row_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationSupportMatrixRow {
    pub fn new(
        obligation_kind: WorthQueryGraphObligationKind,
        support_lane: WorthQueryGraphObligationSupportLane,
        status: WorthQueryGraphObligationSupportStatus,
    ) -> Self {
        Self::with_budget(
            obligation_kind,
            support_lane,
            status,
            WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
            WorthQueryGraphObligationExecutionCostClass::SelectionOnly,
            "state-load counters",
            "artifact-policy-gated diagnostics",
        )
    }

    pub fn with_budget(
        obligation_kind: WorthQueryGraphObligationKind,
        support_lane: WorthQueryGraphObligationSupportLane,
        status: WorthQueryGraphObligationSupportStatus,
        execution_budget: WorthQueryGraphObligationExecutionBudget,
        cost_class: WorthQueryGraphObligationExecutionCostClass,
        state_load_counter_policy: &'static str,
        diagnostic_artifact_policy: &'static str,
    ) -> Self {
        let row_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationSupportMatrixRow)
                .field_shape(
                    WorthQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("support_lane"),
                    support_lane.as_str(),
                )
                .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution_budget"),
                    execution_budget.budget_evidence_digest(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("cost_class"),
                    cost_class.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("state_load_counter_policy"),
                    state_load_counter_policy,
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("diagnostic_artifact_policy"),
                    diagnostic_artifact_policy,
                )
                .seal();
        Self {
            obligation_kind,
            support_lane,
            status,
            execution_budget,
            cost_class,
            state_load_counter_policy,
            diagnostic_artifact_policy,
            row_digest,
        }
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn support_lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn status(&self) -> WorthQueryGraphObligationSupportStatus {
        self.status
    }

    pub fn execution_budget(&self) -> &WorthQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn cost_class(&self) -> WorthQueryGraphObligationExecutionCostClass {
        self.cost_class
    }

    pub fn state_load_counter_policy(&self) -> &'static str {
        self.state_load_counter_policy
    }

    pub fn diagnostic_artifact_policy(&self) -> &'static str {
        self.diagnostic_artifact_policy
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub(crate) fn row_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.row_digest
    }
}
