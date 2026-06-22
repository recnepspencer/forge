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

    pub(in crate::runtime) fn attach_graph_read_access_plan(
        &mut self,
        plan: crate::runtime::ForgeQueryAdmittedGraphReadAccessPlan,
        plan_consumption: crate::runtime::ForgeQueryGraphReadAccessPlanConsumption,
        ephemeral_graph_index_receipt: Option<crate::runtime::ForgeQueryEphemeralGraphIndexReceipt>,
        graph_read_streaming_receipt: Option<crate::runtime::ForgeQueryGraphReadStreamingReceipt>,
    ) {
        let graph_read_access_summary =
            crate::runtime::ForgeQueryGraphReadAccessReceiptSummary::from_execution_parts(
                self.receipt.read_graph_digest(),
                &plan,
                &plan_consumption,
                ephemeral_graph_index_receipt.as_ref(),
                graph_read_streaming_receipt.as_ref(),
            );
        let graph_read_access_complexity_counters =
            crate::runtime::ForgeQueryGraphReadAccessComplexityCounters::from_execution_parts(
                &plan,
                &plan_consumption,
                ephemeral_graph_index_receipt.as_ref(),
                graph_read_streaming_receipt.as_ref(),
            );
        self.receipt.graph_read_access_plan = Some(plan);
        self.receipt.graph_read_access_plan_consumption = Some(plan_consumption);
        self.receipt.ephemeral_graph_index_receipt = ephemeral_graph_index_receipt;
        self.receipt.graph_read_streaming_receipt = graph_read_streaming_receipt;
        self.receipt.graph_read_access_summary = Some(graph_read_access_summary);
        self.receipt.graph_read_access_complexity_counters =
            Some(graph_read_access_complexity_counters);
    }
}
