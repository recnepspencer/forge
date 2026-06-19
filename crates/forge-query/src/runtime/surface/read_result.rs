use crate::memory_workspace::ForgeQueryEntity;
use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryGraphObligationAttachmentEvidence,
    ForgeQueryIntentExecutionProvenance,
};

use super::super::ForgeQueryIntentDecisionTraceEnvelope;

use super::ForgeQueryReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadResult {
    rows: Vec<ForgeQueryEntity>,
    receipt: ForgeQueryReadReceipt,
}

impl ForgeQueryReadResult {
    pub fn rows(&self) -> &[ForgeQueryEntity] {
        &self.rows
    }

    pub fn receipt(&self) -> &ForgeQueryReadReceipt {
        &self.receipt
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.receipt.graph_obligation_dispatch()
    }

    pub fn graph_obligation_evidence(&self) -> Option<ForgeQueryGraphObligationAttachmentEvidence> {
        self.receipt.graph_obligation_evidence()
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.receipt.graph_obligation_envelope_digest()
    }

    pub(in crate::runtime) fn new(
        rows: Vec<ForgeQueryEntity>,
        receipt: ForgeQueryReadReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only(rows: Vec<ForgeQueryEntity>, receipt: ForgeQueryReadReceipt) -> Self {
        Self { rows, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }

    pub(in crate::runtime) fn attach_graph_obligation_dispatch(
        &mut self,
        dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    ) {
        self.receipt.graph_obligation_dispatch = dispatch;
    }
}
