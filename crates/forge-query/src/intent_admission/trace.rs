use crate::identity::hash_parts;
use crate::runtime::ForgeQueryIntentExecution;

use super::{
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdvisoryDecision,
    ForgeQueryIntentViolationDecision, ForgeQueryRawIntentAdmissionRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentDecisionTraceStage {
    RawIntent,
    Eligibility,
    AdmittedDecision,
    AdvisoryStop,
    ExecutionHandoff,
    ExecutionOutcome,
    ViolationStop,
}

impl ForgeQueryIntentDecisionTraceStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RawIntent => "raw-intent",
            Self::Eligibility => "eligibility",
            Self::AdmittedDecision => "admitted-decision",
            Self::AdvisoryStop => "advisory-stop",
            Self::ExecutionHandoff => "execution-handoff",
            Self::ExecutionOutcome => "execution-outcome",
            Self::ViolationStop => "violation-stop",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentDecisionTraceEnvelopeKind {
    AdmittedExecution,
    AdvisoryStop,
    ViolationStop,
}

impl ForgeQueryIntentDecisionTraceEnvelopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedExecution => "admitted-execution",
            Self::AdvisoryStop => "advisory-stop",
            Self::ViolationStop => "violation-stop",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDecisionTraceRow {
    stage: ForgeQueryIntentDecisionTraceStage,
    cause: &'static str,
    detail: String,
    artifact_digest: String,
    row_digest: String,
}

impl ForgeQueryIntentDecisionTraceRow {
    fn new(
        stage: ForgeQueryIntentDecisionTraceStage,
        cause: &'static str,
        detail: impl Into<String>,
        artifact_digest: impl Into<String>,
    ) -> Self {
        let detail = detail.into();
        let artifact_digest = artifact_digest.into();
        let row_digest = hash_parts(&[
            "forge_query_intent_decision_trace_row_v1".to_string(),
            format!("stage:{}", stage.as_str()),
            format!("cause:{cause}"),
            format!("detail:{detail}"),
            format!("artifact:{artifact_digest}"),
        ]);
        Self {
            stage,
            cause,
            detail,
            artifact_digest,
            row_digest,
        }
    }

    pub fn stage(&self) -> ForgeQueryIntentDecisionTraceStage {
        self.stage
    }

    pub fn cause(&self) -> &'static str {
        self.cause
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDecisionTraceEnvelope {
    kind: ForgeQueryIntentDecisionTraceEnvelopeKind,
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    rows: Vec<ForgeQueryIntentDecisionTraceRow>,
    trace_digest: String,
}

impl ForgeQueryIntentDecisionTraceEnvelope {
    pub(crate) fn for_admitted_execution(
        handoff: &ForgeQueryAdmittedIntentExecutionHandoff,
        execution: &ForgeQueryIntentExecution,
    ) -> Self {
        let rows = vec![
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::RawIntent,
                "raw_intent_authored",
                handoff.declaration().name(),
                handoff.request_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::Eligibility,
                "eligibility_resolved",
                handoff.entrypoint().as_str(),
                handoff.eligibility_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
                "admitted_for_execution",
                handoff.execution_seam().as_str(),
                handoff.decision_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
                "execution_handoff_minted",
                handoff.execution_seam().as_str(),
                handoff.handoff_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
                "execution_outcome_recorded",
                execution.execution_kind().as_str(),
                execution.outcome_digest(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution,
            handoff.family(),
            handoff.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_request_violation(
        request: &ForgeQueryRawIntentAdmissionRequest,
        eligibility_digest: &str,
        violation: &ForgeQueryIntentViolationDecision,
    ) -> Self {
        let rows = vec![
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::RawIntent,
                "raw_intent_authored",
                request.declaration().name(),
                request.request_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::Eligibility,
                "eligibility_resolved",
                request.entrypoint().as_str(),
                eligibility_digest,
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            request.family(),
            request.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_request_advisory(
        request: &ForgeQueryRawIntentAdmissionRequest,
        eligibility_digest: &str,
        advisory: &ForgeQueryIntentAdvisoryDecision,
    ) -> Self {
        let rows = vec![
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::RawIntent,
                "raw_intent_authored",
                request.declaration().name(),
                request.request_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::Eligibility,
                "eligibility_resolved",
                request.entrypoint().as_str(),
                eligibility_digest,
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
                advisory.stage(),
                advisory.message(),
                advisory.decision_digest(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::AdvisoryStop,
            request.family(),
            request.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_execution_violation(
        handoff: &ForgeQueryAdmittedIntentExecutionHandoff,
        execution: &ForgeQueryIntentExecution,
        violation: &ForgeQueryIntentViolationDecision,
    ) -> Self {
        let rows = vec![
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::RawIntent,
                "raw_intent_authored",
                handoff.declaration().name(),
                handoff.request_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::Eligibility,
                "eligibility_resolved",
                handoff.entrypoint().as_str(),
                handoff.eligibility_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
                "admitted_for_execution",
                handoff.execution_seam().as_str(),
                handoff.decision_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
                "execution_handoff_minted",
                handoff.execution_seam().as_str(),
                handoff.handoff_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
                "execution_outcome_recorded",
                execution.execution_kind().as_str(),
                execution.outcome_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            handoff.family(),
            handoff.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_handoff_violation(
        handoff: &ForgeQueryAdmittedIntentExecutionHandoff,
        violation: &ForgeQueryIntentViolationDecision,
    ) -> Self {
        let rows = vec![
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::RawIntent,
                "raw_intent_authored",
                handoff.declaration().name(),
                handoff.request_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::Eligibility,
                "eligibility_resolved",
                handoff.entrypoint().as_str(),
                handoff.eligibility_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
                "admitted_for_execution",
                handoff.execution_seam().as_str(),
                handoff.decision_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
                "execution_handoff_minted",
                handoff.execution_seam().as_str(),
                handoff.handoff_digest(),
            ),
            ForgeQueryIntentDecisionTraceRow::new(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            handoff.family(),
            handoff.entrypoint(),
            rows,
        )
    }

    fn new(
        kind: ForgeQueryIntentDecisionTraceEnvelopeKind,
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        rows: Vec<ForgeQueryIntentDecisionTraceRow>,
    ) -> Self {
        let trace_digest = hash_parts(&[
            "forge_query_intent_decision_trace_envelope_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!(
                "rows:{}",
                rows.iter()
                    .map(ForgeQueryIntentDecisionTraceRow::row_digest)
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            kind,
            family,
            entrypoint,
            rows,
            trace_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryIntentDecisionTraceEnvelopeKind {
        self.kind
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn rows(&self) -> &[ForgeQueryIntentDecisionTraceRow] {
        &self.rows
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }
}
