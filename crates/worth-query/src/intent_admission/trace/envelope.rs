use crate::identity::hash_parts;
use crate::runtime::WorthQueryIntentExecution;

use super::super::{
    WorthQueryAdmittedIntentExecutionHandoff, WorthQueryIntentAdmissionCoveredEntrypoint,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentAdvisoryDecision,
    WorthQueryIntentViolationDecision, WorthQueryRawIntentAdmissionRequest,
};
use super::{
    WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceEvidence,
    WorthQueryIntentDecisionTraceRow, WorthQueryIntentDecisionTraceStage,
    WorthQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentDecisionTraceEnvelope {
    kind: WorthQueryIntentDecisionTraceEnvelopeKind,
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    rows: Vec<WorthQueryIntentDecisionTraceRow>,
    trace_digest: String,
}

impl WorthQueryIntentDecisionTraceEnvelope {
    pub(crate) fn for_admitted_execution(
        handoff: &WorthQueryAdmittedIntentExecutionHandoff,
        execution: &WorthQueryIntentExecution,
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
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        request_detail: &str,
        request_digest: &str,
        eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
        decision_digest: &str,
        handoff_digest: &str,
        execution_seam: super::super::WorthQueryIntentAdmissionExecutionSeam,
        execution_detail: &str,
        outcome_digest: &str,
        execution_kind: &str,
    ) -> Self {
        Self::for_admitted_execution_parts_with_obligation_dispatch(
            family,
            entrypoint,
            request_detail,
            request_digest,
            eligibility_trace,
            decision_digest,
            handoff_digest,
            execution_seam,
            None,
            execution_detail,
            outcome_digest,
            execution_kind,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn for_admitted_execution_parts_with_obligation_dispatch(
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        request_detail: &str,
        request_digest: &str,
        eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
        decision_digest: &str,
        handoff_digest: &str,
        execution_seam: super::super::WorthQueryIntentAdmissionExecutionSeam,
        obligation_dispatch_envelope_digest: Option<&str>,
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
        ];
        let rows = rows
            .into_iter()
            .chain(obligation_dispatch_envelope_digest.map(obligation_dispatch_row))
            .chain(std::iter::once(execution_outcome_row(
                execution_detail,
                outcome_digest.to_string(),
                execution_kind.to_string(),
            )))
            .collect::<Vec<_>>();
        Self::new(
            WorthQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution,
            family,
            entrypoint,
            rows,
        )
    }

    pub(crate) fn for_request_violation(
        request: &WorthQueryRawIntentAdmissionRequest,
        eligibility: &WorthQueryIntentEligibilityTraceEvidence,
        violation: &WorthQueryIntentViolationDecision,
    ) -> Self {
        let rows = vec![
            request_row(request.intent_name(), request.request_digest().to_string()),
            eligibility_row(request.entrypoint().as_str(), eligibility.clone()),
            non_admitted_row(
                WorthQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
            ),
        ];
        Self::new(
            WorthQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            request.family(),
            request.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_request_advisory(
        request: &WorthQueryRawIntentAdmissionRequest,
        eligibility: &WorthQueryIntentEligibilityTraceEvidence,
        advisory: &WorthQueryIntentAdvisoryDecision,
    ) -> Self {
        let rows = vec![
            request_row(request.intent_name(), request.request_digest().to_string()),
            eligibility_row(request.entrypoint().as_str(), eligibility.clone()),
            non_admitted_row(
                WorthQueryIntentDecisionTraceStage::AdvisoryStop,
                advisory.stage(),
                advisory.message(),
                advisory.decision_digest().to_string(),
            ),
        ];
        Self::new(
            WorthQueryIntentDecisionTraceEnvelopeKind::AdvisoryStop,
            request.family(),
            request.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_execution_violation(
        handoff: &WorthQueryAdmittedIntentExecutionHandoff,
        execution: &WorthQueryIntentExecution,
        violation: &WorthQueryIntentViolationDecision,
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
                WorthQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
            ),
        ];
        Self::new(
            WorthQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            handoff.family(),
            handoff.entrypoint(),
            rows,
        )
    }

    pub(crate) fn for_handoff_violation(
        handoff: &WorthQueryAdmittedIntentExecutionHandoff,
        violation: &WorthQueryIntentViolationDecision,
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
                WorthQueryIntentDecisionTraceStage::ViolationStop,
                violation.stage(),
                violation.message(),
                violation.decision_digest().to_string(),
            ),
        ];
        Self::new(
            WorthQueryIntentDecisionTraceEnvelopeKind::ViolationStop,
            handoff.family(),
            handoff.entrypoint(),
            rows,
        )
    }

    fn new(
        kind: WorthQueryIntentDecisionTraceEnvelopeKind,
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        rows: Vec<WorthQueryIntentDecisionTraceRow>,
    ) -> Self {
        let trace_digest = hash_parts(&[
            "worth_query_intent_decision_trace_envelope_v2".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!(
                "rows:{}",
                rows.iter()
                    .map(WorthQueryIntentDecisionTraceRow::row_digest)
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

    pub fn kind(&self) -> WorthQueryIntentDecisionTraceEnvelopeKind {
        self.kind
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn rows(&self) -> &[WorthQueryIntentDecisionTraceRow] {
        &self.rows
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.rows.iter().find_map(|row| match row.evidence() {
            WorthQueryIntentDecisionTraceEvidence::ObligationDispatch { envelope_digest } => {
                Some(envelope_digest.as_str())
            }
            _ => None,
        })
    }
}

fn request_row(
    detail: impl Into<String>,
    request_digest: String,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::RawIntent,
        "raw_intent_authored",
        detail,
        WorthQueryIntentDecisionTraceEvidence::Request { request_digest },
    )
}

fn eligibility_row(
    detail: impl Into<String>,
    eligibility: WorthQueryIntentEligibilityTraceEvidence,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::Eligibility,
        "eligibility_resolved",
        detail,
        WorthQueryIntentDecisionTraceEvidence::Eligibility(eligibility),
    )
}

fn admitted_decision_row(
    detail: impl Into<String>,
    decision_digest: String,
    execution_seam: super::super::WorthQueryIntentAdmissionExecutionSeam,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::AdmittedDecision,
        "admitted_for_execution",
        detail,
        WorthQueryIntentDecisionTraceEvidence::AdmittedDecision {
            decision_digest,
            execution_seam,
        },
    )
}

fn execution_handoff_row(
    detail: impl Into<String>,
    handoff_digest: String,
    execution_seam: super::super::WorthQueryIntentAdmissionExecutionSeam,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::ExecutionHandoff,
        "execution_handoff_minted",
        detail,
        WorthQueryIntentDecisionTraceEvidence::ExecutionHandoff {
            handoff_digest,
            execution_seam,
        },
    )
}

fn obligation_dispatch_row(envelope_digest: &str) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::AdmittedDecision,
        "obligation_dispatch_materialized",
        "graph-obligation-dispatch",
        WorthQueryIntentDecisionTraceEvidence::ObligationDispatch {
            envelope_digest: envelope_digest.to_string(),
        },
    )
}

fn execution_outcome_row(
    detail: impl Into<String>,
    outcome_digest: String,
    execution_kind: String,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        WorthQueryIntentDecisionTraceStage::ExecutionOutcome,
        "execution_outcome_recorded",
        detail,
        WorthQueryIntentDecisionTraceEvidence::ExecutionOutcome {
            outcome_digest,
            execution_kind,
        },
    )
}

fn non_admitted_row(
    stage: WorthQueryIntentDecisionTraceStage,
    cause: &'static str,
    detail: impl Into<String>,
    decision_digest: String,
) -> WorthQueryIntentDecisionTraceRow {
    WorthQueryIntentDecisionTraceRow::new(
        stage,
        cause,
        detail,
        WorthQueryIntentDecisionTraceEvidence::NonAdmittedDecision { decision_digest },
    )
}
