use crate::runtime::ForgeQueryIntentExecutionProvenance;

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::ForgeQueryDerivedMaterializationReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationResult {
    payload: Vec<serde_json::Value>,
    receipt: ForgeQueryDerivedMaterializationReceipt,
}

impl ForgeQueryDerivedMaterializationResult {
    pub fn rows(&self) -> &[serde_json::Value] {
        &self.payload
    }

    pub fn receipt(&self) -> &ForgeQueryDerivedMaterializationReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        payload: Vec<serde_json::Value>,
        receipt: ForgeQueryDerivedMaterializationReceipt,
    ) -> Self {
        Self { payload, receipt }
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
