use crate::runtime::{ForgeQueryComputedInspectionEvidence, ForgeQueryIntentExecutionProvenance};

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::ForgeQueryDerivedInspectionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedInspectionResult {
    evidence: ForgeQueryComputedInspectionEvidence,
    receipt: ForgeQueryDerivedInspectionReceipt,
}

impl ForgeQueryDerivedInspectionResult {
    pub fn evidence(&self) -> &ForgeQueryComputedInspectionEvidence {
        &self.evidence
    }

    pub fn receipt(&self) -> &ForgeQueryDerivedInspectionReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        evidence: ForgeQueryComputedInspectionEvidence,
        receipt: ForgeQueryDerivedInspectionReceipt,
    ) -> Self {
        Self { evidence, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
