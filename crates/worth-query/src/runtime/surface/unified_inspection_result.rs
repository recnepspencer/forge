use crate::runtime::{WorthQueryInspection, WorthQueryIntentExecutionProvenance};

use super::super::WorthQueryIntentDecisionTraceEnvelope;
use super::WorthQueryUnifiedInspectionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryUnifiedInspectionResult {
    inspection: WorthQueryInspection,
    receipt: WorthQueryUnifiedInspectionReceipt,
}

impl WorthQueryUnifiedInspectionResult {
    pub fn inspection(&self) -> &WorthQueryInspection {
        &self.inspection
    }

    pub fn receipt(&self) -> &WorthQueryUnifiedInspectionReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        inspection: WorthQueryInspection,
        receipt: WorthQueryUnifiedInspectionReceipt,
    ) -> Self {
        Self {
            inspection,
            receipt,
        }
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
