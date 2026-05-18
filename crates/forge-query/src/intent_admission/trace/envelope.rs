use crate::identity::hash_parts;
use crate::runtime::ForgeQueryIntentExecution;

use super::super::{
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdvisoryDecision,
    ForgeQueryIntentViolationDecision, ForgeQueryRawIntentAdmissionRequest,
};
use super::{
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceEvidence,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentDecisionTraceStage,
    ForgeQueryIntentEligibilityTraceEvidence,
};

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
        Self::for_admitted_execution_parts(
            handoff.family(),
            handoff.entrypoint(),
            handoff.declaration().name(),
            handoff.request_digest(),
            handoff.eligibility_trace().clone(),
            handoff.decision_digest(),
            handoff.handoff_digest(),
            handoff.execution_seam(),
            execution.execution_kind().as_str(),
            execution.outcome_digest(),
            execution.execution_kind().as_str(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn for_admitted_execution_parts(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        request_detail: &str,
        request_digest: &str,
        eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
        decision_digest: &str,
        handoff_digest: &str,
        execution_seam: super::super::ForgeQueryIntentAdmissionExecutionSeam,
        execution_detail: &str,
        outcome_digest: &str,
        execution_kind: &str,
    ) -> Self {
        let rows = vec![
            request_row(request_detail, request_digest.to_string()),
            eligibility_row(entrypoint.as_str(), eligibility_trace),
            admitted_decision_row(
                execution_seam.as_str(),
                decision_digest.to_string(),
                execution_seam,
            ),
            execution_handoff_row(
                execution_seam.as_str(),
                handoff_digest.to_string(),
                execution_seam,
            ),
            execution_outcome_row(
                execution_detail,
                outcome_digest.to_string(),
                execution_kind.to_string(),
            ),
        ];
        Self::new(
            ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution,
            family,
            entrypoint,
            rows,
        )
    }

    pub(crate) fn for_request_violation(
        request: &ForgeQueryRawIntentAdmissionRequest,
        eligibility: &ForgeQueryIntentEligibilityTraceEvidence,
        violation: &ForgeQueryIntentViolationDecision,
    ) -> Self {
        let rows = vec![
            request_row(request.intent_name(), request.request_digest().to_string()),
            eligibility_row(request.entrypoint().as_str(), eligibility.clone()),
            non_admitted_row(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
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
        eligibility: &ForgeQueryIntentEligibilityTraceEvidence,
        advisory: &ForgeQueryIntentAdvisoryDecision,
    ) -> Self {
        let rows = vec![
            request_row(request.intent_name(), request.request_digest().to_string()),
            eligibility_row(request.entrypoint().as_str(), eligibility.clone()),
            non_admitted_row(
                ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
                advisory.stage(),
                advisory.message(),
                advisory.decision_digest().to_string(),
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
            request_row(
                handoff.declaration().name(),
                handoff.request_digest().to_string(),
            ),
            eligibility_row(
                handoff.entrypoint().as_str(),
                handoff.eligibility_trace().clone(),
            ),
            admitted_decision_row(
                handoff.execution_seam().as_str(),
                handoff.decision_digest().to_string(),
                handoff.execution_seam(),
            ),
            execution_handoff_row(
                handoff.execution_seam().as_str(),
                handoff.handoff_digest().to_string(),
                handoff.execution_seam(),
            ),
            execution_outcome_row(
                execution.execution_kind().as_str(),
                execution.outcome_digest().to_string(),
                execution.execution_kind().as_str().to_string(),
            ),
            non_admitted_row(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
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
            request_row(
                handoff.declaration().name(),
                handoff.request_digest().to_string(),
            ),
            eligibility_row(
                handoff.entrypoint().as_str(),
                handoff.eligibility_trace().clone(),
            ),
            admitted_decision_row(
                handoff.execution_seam().as_str(),
                handoff.decision_digest().to_string(),
                handoff.execution_seam(),
            ),
            execution_handoff_row(
                handoff.execution_seam().as_str(),
                handoff.handoff_digest().to_string(),
                handoff.execution_seam(),
            ),
            non_admitted_row(
                ForgeQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
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
            "forge_query_intent_decision_trace_envelope_v2".to_string(),
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

fn request_row(
    detail: impl Into<String>,
    request_digest: String,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        ForgeQueryIntentDecisionTraceStage::RawIntent,
        "raw_intent_authored",
        detail,
        ForgeQueryIntentDecisionTraceEvidence::Request { request_digest },
    )
}

fn eligibility_row(
    detail: impl Into<String>,
    eligibility: ForgeQueryIntentEligibilityTraceEvidence,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        ForgeQueryIntentDecisionTraceStage::Eligibility,
        "eligibility_resolved",
        detail,
        ForgeQueryIntentDecisionTraceEvidence::Eligibility(eligibility),
    )
}

fn admitted_decision_row(
    detail: impl Into<String>,
    decision_digest: String,
    execution_seam: super::super::ForgeQueryIntentAdmissionExecutionSeam,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
        "admitted_for_execution",
        detail,
        ForgeQueryIntentDecisionTraceEvidence::AdmittedDecision {
            decision_digest,
            execution_seam,
        },
    )
}

fn execution_handoff_row(
    detail: impl Into<String>,
    handoff_digest: String,
    execution_seam: super::super::ForgeQueryIntentAdmissionExecutionSeam,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
        "execution_handoff_minted",
        detail,
        ForgeQueryIntentDecisionTraceEvidence::ExecutionHandoff {
            handoff_digest,
            execution_seam,
        },
    )
}

fn execution_outcome_row(
    detail: impl Into<String>,
    outcome_digest: String,
    execution_kind: String,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        "execution_outcome_recorded",
        detail,
        ForgeQueryIntentDecisionTraceEvidence::ExecutionOutcome {
            outcome_digest,
            execution_kind,
        },
    )
}

fn non_admitted_row(
    stage: ForgeQueryIntentDecisionTraceStage,
    cause: &'static str,
    detail: impl Into<String>,
    decision_digest: String,
) -> ForgeQueryIntentDecisionTraceRow {
    ForgeQueryIntentDecisionTraceRow::new(
        stage,
        cause,
        detail,
        ForgeQueryIntentDecisionTraceEvidence::NonAdmittedDecision { decision_digest },
    )
}
