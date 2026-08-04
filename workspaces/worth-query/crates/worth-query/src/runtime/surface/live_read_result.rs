use crate::runtime::{WorthQueryIntentExecutionProvenance, WorthQueryLiveGraphReadAccessReceipt};

use super::super::WorthQueryIntentDecisionTraceEnvelope;
use super::WorthQueryLiveReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryLiveReadResult {
    rows: Vec<crate::memory_workspace::WorthQueryEntity>,
    receipt: WorthQueryLiveReadReceipt,
}

impl WorthQueryLiveReadResult {
    pub fn rows(&self) -> &[crate::memory_workspace::WorthQueryEntity] {
        &self.rows
    }

    pub fn receipt(&self) -> &WorthQueryLiveReadReceipt {
        &self.receipt
    }

    pub fn live_graph_read_access(&self) -> Option<&WorthQueryLiveGraphReadAccessReceipt> {
        self.receipt.live_graph_read_access()
    }

    pub(in crate::runtime) fn new(
        rows: Vec<crate::memory_workspace::WorthQueryEntity>,
        receipt: WorthQueryLiveReadReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: WorthQueryIntentDecisionTraceEnvelope,
        execution_provenance: WorthQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }

    pub(in crate::runtime) fn attach_live_graph_read_access(
        &mut self,
        receipt: WorthQueryLiveGraphReadAccessReceipt,
    ) {
        self.receipt.live_graph_read_access = Some(receipt);
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        rows: Vec<crate::memory_workspace::WorthQueryEntity>,
        receipt: WorthQueryLiveReadReceipt,
    ) -> Self {
        Self { rows, receipt }
    }
}
