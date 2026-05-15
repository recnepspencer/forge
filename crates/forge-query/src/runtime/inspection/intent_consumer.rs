use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceStage,
};

use super::super::{
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecutionFailureEvidence,
    ForgeQueryIntentExecutionProvenance, ForgeQueryIntentReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentConsumerOutcomeClass {
    Admitted,
    Advisory,
    Violation,
}

pub struct ForgeQueryIntentConsumerInspection<'a> {
    intent_name: &'a str,
    outcome_class: ForgeQueryIntentConsumerOutcomeClass,
    admission_family: Option<ForgeQueryIntentAdmissionFamily>,
    covered_entrypoint: Option<ForgeQueryIntentAdmissionCoveredEntrypoint>,
    decision_trace_envelope: Option<&'a ForgeQueryIntentDecisionTraceEnvelope>,
    execution_provenance: Option<&'a ForgeQueryIntentExecutionProvenance>,
    fallback_stage: &'static str,
    fallback_cause: &'static str,
    fallback_detail: &'a str,
}

impl<'a> ForgeQueryIntentConsumerInspection<'a> {
    pub(crate) fn from_review(
        intent_name: &'a str,
        decision: &'a ForgeQueryIntentAdmissionDecision,
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        decision_trace_envelope: Option<&'a ForgeQueryIntentDecisionTraceEnvelope>,
    ) -> Self {
        match decision {
            ForgeQueryIntentAdmissionDecision::Admitted(plan) => Self {
                intent_name,
                outcome_class: ForgeQueryIntentConsumerOutcomeClass::Admitted,
                admission_family: Some(plan.family()),
                covered_entrypoint: Some(plan.entrypoint()),
                decision_trace_envelope,
                execution_provenance: None,
                fallback_stage: "admitted-decision",
                fallback_cause: "admitted_for_execution",
                fallback_detail: plan
                    .execution_seam()
                    .map(|seam| seam.as_str())
                    .unwrap_or("no-execution-handoff"),
            },
            ForgeQueryIntentAdmissionDecision::Advisory(advisory) => Self {
                intent_name,
                outcome_class: ForgeQueryIntentConsumerOutcomeClass::Advisory,
                admission_family: Some(family),
                covered_entrypoint: Some(entrypoint),
                decision_trace_envelope,
                execution_provenance: None,
                fallback_stage: "advisory-stop",
                fallback_cause: advisory.stage(),
                fallback_detail: advisory.message(),
            },
            ForgeQueryIntentAdmissionDecision::Violation(violation) => Self {
                intent_name,
                outcome_class: ForgeQueryIntentConsumerOutcomeClass::Violation,
                admission_family: Some(family),
                covered_entrypoint: Some(entrypoint),
                decision_trace_envelope,
                execution_provenance: None,
                fallback_stage: "violation-stop",
                fallback_cause: violation.stage(),
                fallback_detail: violation.message(),
            },
        }
    }

    pub(crate) fn from_receipt(receipt: &'a ForgeQueryIntentReceipt) -> Self {
        Self {
            intent_name: receipt.intent_name(),
            outcome_class: ForgeQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: Some(receipt.execution_provenance().family()),
            covered_entrypoint: Some(receipt.execution_provenance().entrypoint()),
            decision_trace_envelope: Some(receipt.decision_trace_envelope()),
            execution_provenance: Some(receipt.execution_provenance()),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: receipt.execution_kind().as_str(),
        }
    }

    pub(crate) fn from_denial(evidence: &'a ForgeQueryIntentDenialEvidence) -> Self {
        Self {
            intent_name: evidence.intent_name(),
            outcome_class: ForgeQueryIntentConsumerOutcomeClass::Violation,
            admission_family: evidence.execution_provenance().map(|p| p.family()),
            covered_entrypoint: evidence.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: evidence.decision_trace_envelope(),
            execution_provenance: evidence.execution_provenance(),
            fallback_stage: "violation-stop",
            fallback_cause: evidence.stage(),
            fallback_detail: evidence.message(),
        }
    }

    pub(crate) fn from_failure(evidence: &'a ForgeQueryIntentExecutionFailureEvidence) -> Self {
        Self {
            intent_name: evidence.intent_name(),
            outcome_class: ForgeQueryIntentConsumerOutcomeClass::Violation,
            admission_family: Some(evidence.execution_provenance().family()),
            covered_entrypoint: Some(evidence.execution_provenance().entrypoint()),
            decision_trace_envelope: Some(evidence.decision_trace_envelope()),
            execution_provenance: Some(evidence.execution_provenance()),
            fallback_stage: "violation-stop",
            fallback_cause: evidence.stage(),
            fallback_detail: evidence.message(),
        }
    }

    pub fn intent_name(&self) -> &str {
        self.intent_name
    }

    pub fn outcome_class(&self) -> ForgeQueryIntentConsumerOutcomeClass {
        self.outcome_class
    }

    pub fn decision_trace_envelope_kind(
        &self,
    ) -> Option<ForgeQueryIntentDecisionTraceEnvelopeKind> {
        self.decision_trace_envelope
            .map(ForgeQueryIntentDecisionTraceEnvelope::kind)
    }

    pub fn decision_trace_envelope(&self) -> Option<&'a ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope
    }

    pub fn admission_family(&self) -> Option<ForgeQueryIntentAdmissionFamily> {
        self.admission_family.or_else(|| {
            self.decision_trace_envelope
                .map(ForgeQueryIntentDecisionTraceEnvelope::family)
        })
    }

    pub fn covered_entrypoint(&self) -> Option<ForgeQueryIntentAdmissionCoveredEntrypoint> {
        self.covered_entrypoint.or_else(|| {
            self.decision_trace_envelope
                .map(ForgeQueryIntentDecisionTraceEnvelope::entrypoint)
        })
    }

    pub fn terminal_stage(&self) -> Option<ForgeQueryIntentDecisionTraceStage> {
        self.decision_trace_envelope
            .and_then(|envelope| envelope.rows().last().map(|row| row.stage()))
    }

    pub fn terminal_stage_label(&self) -> &str {
        self.terminal_stage()
            .map(ForgeQueryIntentDecisionTraceStage::as_str)
            .unwrap_or(self.fallback_stage)
    }

    pub fn terminal_cause(&self) -> &str {
        self.decision_trace_envelope
            .and_then(|envelope| envelope.rows().last().map(|row| row.cause()))
            .unwrap_or(self.fallback_cause)
    }

    pub fn terminal_detail(&self) -> &str {
        self.decision_trace_envelope
            .and_then(|envelope| envelope.rows().last().map(|row| row.detail()))
            .unwrap_or(self.fallback_detail)
    }

    pub fn decision_trace_digest(&self) -> Option<&str> {
        self.decision_trace_envelope
            .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest)
    }

    pub fn execution_provenance(&self) -> Option<&'a ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance
    }

    pub fn execution_provenance_chain_digest(&self) -> Option<&str> {
        self.execution_provenance
            .map(ForgeQueryIntentExecutionProvenance::execution_provenance_chain_digest)
    }
}
