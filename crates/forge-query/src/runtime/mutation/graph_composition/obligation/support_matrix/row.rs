use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationExecutionCostClass,
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSupportMatrixRow {
    obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
    status: ForgeQueryGraphObligationSupportStatus,
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
    cost_class: ForgeQueryGraphObligationExecutionCostClass,
    state_load_counter_policy: &'static str,
    diagnostic_artifact_policy: &'static str,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationSupportMatrixRow {
    pub fn new(
        obligation_kind: ForgeQueryGraphObligationKind,
        support_lane: ForgeQueryGraphObligationSupportLane,
        status: ForgeQueryGraphObligationSupportStatus,
    ) -> Self {
        Self::with_budget(
            obligation_kind,
            support_lane,
            status,
            ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
            ForgeQueryGraphObligationExecutionCostClass::SelectionOnly,
            "state-load counters",
            "artifact-policy-gated diagnostics",
        )
    }

    pub fn with_budget(
        obligation_kind: ForgeQueryGraphObligationKind,
        support_lane: ForgeQueryGraphObligationSupportLane,
        status: ForgeQueryGraphObligationSupportStatus,
        execution_budget: ForgeQueryGraphObligationExecutionBudget,
        cost_class: ForgeQueryGraphObligationExecutionCostClass,
        state_load_counter_policy: &'static str,
        diagnostic_artifact_policy: &'static str,
    ) -> Self {
        let row_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationSupportMatrixRow)
                .field_shape(
                    ForgeQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("support_lane"),
                    support_lane.as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_budget"),
                    execution_budget.budget_evidence_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("cost_class"),
                    cost_class.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("state_load_counter_policy"),
                    state_load_counter_policy,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("diagnostic_artifact_policy"),
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

    pub fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn status(&self) -> ForgeQueryGraphObligationSupportStatus {
        self.status
    }

    pub fn execution_budget(&self) -> &ForgeQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn cost_class(&self) -> ForgeQueryGraphObligationExecutionCostClass {
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

    pub(crate) fn row_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}
