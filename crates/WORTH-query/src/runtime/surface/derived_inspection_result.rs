use crate::runtime::{WorthQueryComputedInspectionEvidence, WorthQueryIntentExecutionProvenance};

use super::super::WorthQueryIntentDecisionTraceEnvelope;
use super::WorthQueryDerivedInspectionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedInspectionResult {
    evidence: WorthQueryComputedInspectionEvidence,
    receipt: WorthQueryDerivedInspectionReceipt,
}

impl WorthQueryDerivedInspectionResult {
    pub fn evidence(&self) -> &WorthQueryComputedInspectionEvidence {
        &self.evidence
    }

    pub fn receipt(&self) -> &WorthQueryDerivedInspectionReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        evidence: WorthQueryComputedInspectionEvidence,
        receipt: WorthQueryDerivedInspectionReceipt,
    ) -> Self {
        Self { evidence, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: WorthQueryIntentDecisionTraceEnvelope,
        execution_provenance: WorthQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
