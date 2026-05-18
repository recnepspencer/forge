use crate::memory_workspace::ForgeQueryEntity;
use crate::runtime::ForgeQueryIntentExecutionProvenance;

use super::super::ForgeQueryIntentDecisionTraceEnvelope;

use super::ForgeQueryReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadResult {
    payload: Vec<ForgeQueryEntity>,
    receipt: ForgeQueryReadReceipt,
}

impl ForgeQueryReadResult {
    pub fn payload(&self) -> &[ForgeQueryEntity] {
        &self.payload
    }

    pub fn rows(&self) -> &[ForgeQueryEntity] {
        &self.payload
    }

    pub fn receipt(&self) -> &ForgeQueryReadReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        payload: Vec<ForgeQueryEntity>,
        receipt: ForgeQueryReadReceipt,
    ) -> Self {
        Self { payload, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        payload: Vec<ForgeQueryEntity>,
        receipt: ForgeQueryReadReceipt,
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
