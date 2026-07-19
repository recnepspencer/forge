use crate::intent_admission::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionDecision,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentDecisionTraceEnvelope,
    WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceStage,
};

use super::super::{
    WorthQueryDerivedInspectionReceipt, WorthQueryDerivedMaterializationReceipt,
    WorthQueryExistingTruthProbeReceipt, WorthQueryIntentDenialEvidence,
    WorthQueryIntentExecutionFailureEvidence, WorthQueryIntentExecutionProvenance,
    WorthQueryIntentReceipt, WorthQueryLiveReadReceipt, WorthQueryUnifiedInspectionReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentConsumerOutcomeClass {
    Admitted,
    Advisory,
    Violation,
}

pub struct WorthQueryIntentConsumerInspection<'a> {
    intent_name: &'a str,
    outcome_class: WorthQueryIntentConsumerOutcomeClass,
    admission_family: Option<WorthQueryIntentAdmissionFamily>,
    covered_entrypoint: Option<WorthQueryIntentAdmissionCoveredEntrypoint>,
    decision_trace_envelope: Option<&'a WorthQueryIntentDecisionTraceEnvelope>,
    execution_provenance: Option<&'a WorthQueryIntentExecutionProvenance>,
    fallback_stage: &'static str,
    fallback_cause: &'static str,
    fallback_detail: &'a str,
}

impl<'a> WorthQueryIntentConsumerInspection<'a> {
    pub(crate) fn from_review(
        intent_name: &'a str,
        decision: &'a WorthQueryIntentAdmissionDecision,
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        decision_trace_envelope: Option<&'a WorthQueryIntentDecisionTraceEnvelope>,
    ) -> Self {
        match decision {
            WorthQueryIntentAdmissionDecision::Admitted(plan) => Self {
                intent_name,
                outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
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
            WorthQueryIntentAdmissionDecision::Advisory(advisory) => Self {
                intent_name,
                outcome_class: WorthQueryIntentConsumerOutcomeClass::Advisory,
                admission_family: Some(family),
                covered_entrypoint: Some(entrypoint),
                decision_trace_envelope,
                execution_provenance: None,
                fallback_stage: "advisory-stop",
                fallback_cause: advisory.stage(),
                fallback_detail: advisory.message(),
            },
            WorthQueryIntentAdmissionDecision::Violation(violation) => Self {
                intent_name,
                outcome_class: WorthQueryIntentConsumerOutcomeClass::Violation,
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

    pub(crate) fn from_receipt(receipt: &'a WorthQueryIntentReceipt) -> Self {
        Self {
            intent_name: receipt.intent_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: Some(receipt.execution_provenance().family()),
            covered_entrypoint: Some(receipt.execution_provenance().entrypoint()),
            decision_trace_envelope: Some(receipt.decision_trace_envelope()),
            execution_provenance: Some(receipt.execution_provenance()),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: receipt.execution_kind().as_str(),
        }
    }

    pub(crate) fn from_denial(evidence: &'a WorthQueryIntentDenialEvidence) -> Self {
        Self {
            intent_name: evidence.intent_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Violation,
            admission_family: evidence.execution_provenance().map(|p| p.family()),
            covered_entrypoint: evidence.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: evidence.decision_trace_envelope(),
            execution_provenance: evidence.execution_provenance(),
            fallback_stage: "violation-stop",
            fallback_cause: evidence.stage(),
            fallback_detail: evidence.message(),
        }
    }

    pub(crate) fn from_failure(evidence: &'a WorthQueryIntentExecutionFailureEvidence) -> Self {
        Self {
            intent_name: evidence.intent_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Violation,
            admission_family: Some(evidence.execution_provenance().family()),
            covered_entrypoint: Some(evidence.execution_provenance().entrypoint()),
            decision_trace_envelope: Some(evidence.decision_trace_envelope()),
            execution_provenance: Some(evidence.execution_provenance()),
            fallback_stage: "violation-stop",
            fallback_cause: evidence.stage(),
            fallback_detail: evidence.message(),
        }
    }

    pub(crate) fn from_mutation_receipt(
        intent_name: &'a str,
        execution_provenance: &'a WorthQueryIntentExecutionProvenance,
        decision_trace_envelope: &'a WorthQueryIntentDecisionTraceEnvelope,
    ) -> Self {
        Self {
            intent_name,
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: Some(execution_provenance.family()),
            covered_entrypoint: Some(execution_provenance.entrypoint()),
            decision_trace_envelope: Some(decision_trace_envelope),
            execution_provenance: Some(execution_provenance),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "mutation-write",
        }
    }

    pub(crate) fn from_live_read_receipt(receipt: &'a WorthQueryLiveReadReceipt) -> Self {
        Self {
            intent_name: receipt.view_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: receipt.execution_provenance().map(|p| p.family()),
            covered_entrypoint: receipt.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: receipt.decision_trace_envelope(),
            execution_provenance: receipt.execution_provenance(),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "live-view-read",
        }
    }

    pub(crate) fn from_derived_materialization_receipt(
        receipt: &'a WorthQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self {
            intent_name: receipt.view_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: receipt.execution_provenance().map(|p| p.family()),
            covered_entrypoint: receipt.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: receipt.decision_trace_envelope(),
            execution_provenance: receipt.execution_provenance(),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "derived-view-materialization",
        }
    }

    pub(crate) fn from_derived_inspection_receipt(
        receipt: &'a WorthQueryDerivedInspectionReceipt,
    ) -> Self {
        Self {
            intent_name: receipt.view_name(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: receipt.execution_provenance().map(|p| p.family()),
            covered_entrypoint: receipt.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: receipt.decision_trace_envelope(),
            execution_provenance: receipt.execution_provenance(),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "derived-view-inspection",
        }
    }

    pub(crate) fn from_existing_truth_probe_receipt(
        receipt: &'a WorthQueryExistingTruthProbeReceipt,
    ) -> Self {
        Self {
            intent_name: receipt.authoritative_identity(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: receipt.execution_provenance().map(|p| p.family()),
            covered_entrypoint: receipt.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: receipt.decision_trace_envelope(),
            execution_provenance: receipt.execution_provenance(),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "existing-truth-probe",
        }
    }

    pub(crate) fn from_unified_inspection_receipt(
        receipt: &'a WorthQueryUnifiedInspectionReceipt,
    ) -> Self {
        Self {
            intent_name: receipt.target_label(),
            outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
            admission_family: receipt.execution_provenance().map(|p| p.family()),
            covered_entrypoint: receipt.execution_provenance().map(|p| p.entrypoint()),
            decision_trace_envelope: receipt.decision_trace_envelope(),
            execution_provenance: receipt.execution_provenance(),
            fallback_stage: "execution-outcome",
            fallback_cause: "execution_outcome_recorded",
            fallback_detail: "unified-inspection",
        }
    }

    pub fn intent_name(&self) -> &str {
        self.intent_name
    }

    pub fn outcome_class(&self) -> WorthQueryIntentConsumerOutcomeClass {
        self.outcome_class
    }

    pub fn decision_trace_envelope_kind(
        &self,
    ) -> Option<WorthQueryIntentDecisionTraceEnvelopeKind> {
        self.decision_trace_envelope
            .map(WorthQueryIntentDecisionTraceEnvelope::kind)
    }

    pub fn decision_trace_envelope(&self) -> Option<&'a WorthQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope
    }

    pub fn admission_family(&self) -> Option<WorthQueryIntentAdmissionFamily> {
        self.admission_family.or_else(|| {
            self.decision_trace_envelope
                .map(WorthQueryIntentDecisionTraceEnvelope::family)
        })
    }

    pub fn covered_entrypoint(&self) -> Option<WorthQueryIntentAdmissionCoveredEntrypoint> {
        self.covered_entrypoint.or_else(|| {
            self.decision_trace_envelope
                .map(WorthQueryIntentDecisionTraceEnvelope::entrypoint)
        })
    }

    pub fn terminal_stage(&self) -> Option<WorthQueryIntentDecisionTraceStage> {
        self.decision_trace_envelope
            .and_then(|envelope| envelope.rows().last().map(|row| row.stage()))
    }

    pub fn terminal_stage_label(&self) -> &str {
        self.terminal_stage()
            .map(WorthQueryIntentDecisionTraceStage::as_str)
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
            .map(WorthQueryIntentDecisionTraceEnvelope::trace_digest)
    }

    pub fn execution_provenance(&self) -> Option<&'a WorthQueryIntentExecutionProvenance> {
        self.execution_provenance
    }

    pub fn execution_provenance_chain_digest(&self) -> Option<&str> {
        self.execution_provenance
            .map(WorthQueryIntentExecutionProvenance::execution_provenance_chain_digest)
    }
}
