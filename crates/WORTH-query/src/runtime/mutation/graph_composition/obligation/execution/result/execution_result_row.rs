use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationVerdict,
};

use super::diagnostic_materialization::WorthQueryGraphObligationDiagnosticMaterialization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionResultRow {
    input: WorthQueryGraphObligationExecutionInput,
    status: WorthQueryGraphObligationExecutionStatus,
    verdict: Option<WorthQueryGraphObligationVerdict>,
    state_load_counters: WorthQueryGraphObligationStateLoadCounters,
    diagnostic_materialization: WorthQueryGraphObligationDiagnosticMaterialization,
    row_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutionResultRow {
    pub fn selected(input: WorthQueryGraphObligationExecutionInput) -> Self {
        Self::new(
            input,
            WorthQueryGraphObligationExecutionStatus::Selected,
            None,
            WorthQueryGraphObligationStateLoadCounters::none(),
        )
    }

    pub fn executed(
        input: WorthQueryGraphObligationExecutionInput,
        verdict: WorthQueryGraphObligationVerdict,
        state_load_counters: WorthQueryGraphObligationStateLoadCounters,
    ) -> Self {
        Self::new(
            input,
            WorthQueryGraphObligationExecutionStatus::Executed,
            Some(verdict),
            state_load_counters,
        )
    }

    pub fn status_only(
        input: WorthQueryGraphObligationExecutionInput,
        status: WorthQueryGraphObligationExecutionStatus,
    ) -> Self {
        Self::new(
            input,
            status,
            None,
            WorthQueryGraphObligationStateLoadCounters::none(),
        )
    }

    pub fn new(
        input: WorthQueryGraphObligationExecutionInput,
        status: WorthQueryGraphObligationExecutionStatus,
        verdict: Option<WorthQueryGraphObligationVerdict>,
        state_load_counters: WorthQueryGraphObligationStateLoadCounters,
    ) -> Self {
        Self::new_with_diagnostic_materialization(
            input,
            status,
            verdict,
            state_load_counters,
            WorthQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly,
        )
    }

    pub fn new_with_diagnostic_materialization(
        input: WorthQueryGraphObligationExecutionInput,
        status: WorthQueryGraphObligationExecutionStatus,
        verdict: Option<WorthQueryGraphObligationVerdict>,
        state_load_counters: WorthQueryGraphObligationStateLoadCounters,
        diagnostic_materialization: WorthQueryGraphObligationDiagnosticMaterialization,
    ) -> Self {
        let row_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationExecutionResultRow,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("input"),
            input.input_evidence_digest(),
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .optional_value(
            WorthQueryEvidenceTag::new("verdict"),
            verdict
                .as_ref()
                .map(WorthQueryGraphObligationVerdict::as_str),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("verdict_context"),
            verdict
                .as_ref()
                .and_then(WorthQueryGraphObligationVerdict::context),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("state_load_counters"),
            state_load_counters.counters_evidence_digest(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("diagnostic_materialization"),
            diagnostic_materialization.as_str(),
        )
        .seal();
        Self {
            input,
            status,
            verdict,
            state_load_counters,
            diagnostic_materialization,
            row_digest,
        }
    }

    pub fn input(&self) -> &WorthQueryGraphObligationExecutionInput {
        &self.input
    }

    pub fn status(&self) -> WorthQueryGraphObligationExecutionStatus {
        self.status
    }

    pub fn verdict(&self) -> Option<&WorthQueryGraphObligationVerdict> {
        self.verdict.as_ref()
    }

    pub fn state_load_counters(&self) -> &WorthQueryGraphObligationStateLoadCounters {
        &self.state_load_counters
    }

    pub fn diagnostic_materialization(&self) -> WorthQueryGraphObligationDiagnosticMaterialization {
        self.diagnostic_materialization
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub(crate) fn row_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.row_digest
    }
}
