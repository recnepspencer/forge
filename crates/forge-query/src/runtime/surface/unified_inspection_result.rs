use crate::runtime::{ForgeQueryInspection, ForgeQueryIntentExecutionProvenance};

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::ForgeQueryUnifiedInspectionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryUnifiedInspectionResult {
    inspection: ForgeQueryInspection,
    receipt: ForgeQueryUnifiedInspectionReceipt,
}

impl ForgeQueryUnifiedInspectionResult {
    pub fn inspection(&self) -> &ForgeQueryInspection {
        &self.inspection
    }

    pub fn receipt(&self) -> &ForgeQueryUnifiedInspectionReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        inspection: ForgeQueryInspection,
        receipt: ForgeQueryUnifiedInspectionReceipt,
    ) -> Self {
        Self {
            inspection,
            receipt,
        }
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
