use crate::runtime::ForgeQueryIntentExecutionProvenance;

use super::super::ForgeQueryIntentDecisionTraceEnvelope;
use super::ForgeQueryLiveReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadResult {
    payload: Vec<crate::memory_workspace::ForgeQueryEntity>,
    receipt: ForgeQueryLiveReadReceipt,
}

impl ForgeQueryLiveReadResult {
    pub fn rows(&self) -> &[crate::memory_workspace::ForgeQueryEntity] {
        &self.payload
    }

    pub fn receipt(&self) -> &ForgeQueryLiveReadReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        payload: Vec<crate::memory_workspace::ForgeQueryEntity>,
        receipt: ForgeQueryLiveReadReceipt,
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
