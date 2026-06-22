use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationVerdict,
};

use super::diagnostic_materialization::ForgeQueryGraphObligationDiagnosticMaterialization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutionResultRow {
    input: ForgeQueryGraphObligationExecutionInput,
    status: ForgeQueryGraphObligationExecutionStatus,
    verdict: Option<ForgeQueryGraphObligationVerdict>,
    state_load_counters: ForgeQueryGraphObligationStateLoadCounters,
    diagnostic_materialization: ForgeQueryGraphObligationDiagnosticMaterialization,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutionResultRow {
    pub fn selected(input: ForgeQueryGraphObligationExecutionInput) -> Self {
        Self::new(
            input,
            ForgeQueryGraphObligationExecutionStatus::Selected,
            None,
            ForgeQueryGraphObligationStateLoadCounters::none(),
        )
    }

    pub fn executed(
        input: ForgeQueryGraphObligationExecutionInput,
        verdict: ForgeQueryGraphObligationVerdict,
        state_load_counters: ForgeQueryGraphObligationStateLoadCounters,
    ) -> Self {
        Self::new(
            input,
            ForgeQueryGraphObligationExecutionStatus::Executed,
            Some(verdict),
            state_load_counters,
        )
    }

    pub fn status_only(
        input: ForgeQueryGraphObligationExecutionInput,
        status: ForgeQueryGraphObligationExecutionStatus,
    ) -> Self {
        Self::new(
            input,
            status,
            None,
            ForgeQueryGraphObligationStateLoadCounters::none(),
        )
    }

    pub fn new(
        input: ForgeQueryGraphObligationExecutionInput,
        status: ForgeQueryGraphObligationExecutionStatus,
        verdict: Option<ForgeQueryGraphObligationVerdict>,
        state_load_counters: ForgeQueryGraphObligationStateLoadCounters,
    ) -> Self {
        Self::new_with_diagnostic_materialization(
            input,
            status,
            verdict,
            state_load_counters,
            ForgeQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly,
        )
    }

    pub fn new_with_diagnostic_materialization(
        input: ForgeQueryGraphObligationExecutionInput,
        status: ForgeQueryGraphObligationExecutionStatus,
        verdict: Option<ForgeQueryGraphObligationVerdict>,
        state_load_counters: ForgeQueryGraphObligationStateLoadCounters,
        diagnostic_materialization: ForgeQueryGraphObligationDiagnosticMaterialization,
    ) -> Self {
        let row_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationExecutionResultRow,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("input"),
            input.input_evidence_digest(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .optional_value(
            ForgeQueryEvidenceTag::new("verdict"),
            verdict
                .as_ref()
                .map(ForgeQueryGraphObligationVerdict::as_str),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("verdict_context"),
            verdict
                .as_ref()
                .and_then(ForgeQueryGraphObligationVerdict::context),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("state_load_counters"),
            state_load_counters.counters_evidence_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("diagnostic_materialization"),
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

    pub fn input(&self) -> &ForgeQueryGraphObligationExecutionInput {
        &self.input
    }

    pub fn status(&self) -> ForgeQueryGraphObligationExecutionStatus {
        self.status
    }

    pub fn verdict(&self) -> Option<&ForgeQueryGraphObligationVerdict> {
        self.verdict.as_ref()
    }

    pub fn state_load_counters(&self) -> &ForgeQueryGraphObligationStateLoadCounters {
        &self.state_load_counters
    }

    pub fn diagnostic_materialization(&self) -> ForgeQueryGraphObligationDiagnosticMaterialization {
        self.diagnostic_materialization
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub(crate) fn row_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}
