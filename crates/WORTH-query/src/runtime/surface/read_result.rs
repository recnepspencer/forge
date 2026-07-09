use crate::memory_workspace::WorthQueryEntity;
use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphObligationAttachmentEvidence,
    WorthQueryIntentExecutionProvenance,
};

use super::super::WorthQueryIntentDecisionTraceEnvelope;

use super::WorthQueryReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryReadResult {
    rows: Vec<WorthQueryEntity>,
    receipt: WorthQueryReadReceipt,
}

impl WorthQueryReadResult {
    pub fn rows(&self) -> &[WorthQueryEntity] {
        &self.rows
    }

    pub fn receipt(&self) -> &WorthQueryReadReceipt {
        &self.receipt
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationObligationDispatch> {
        self.receipt.graph_obligation_dispatch()
    }

    pub fn graph_obligation_evidence(&self) -> Option<WorthQueryGraphObligationAttachmentEvidence> {
        self.receipt.graph_obligation_evidence()
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.receipt.graph_obligation_envelope_digest()
    }

    pub(in crate::runtime) fn new(
        rows: Vec<WorthQueryEntity>,
        receipt: WorthQueryReadReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only(rows: Vec<WorthQueryEntity>, receipt: WorthQueryReadReceipt) -> Self {
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

    pub(in crate::runtime) fn attach_graph_obligation_dispatch(
        &mut self,
        dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    ) {
        self.receipt.graph_obligation_dispatch = dispatch;
    }

    pub(in crate::runtime) fn attach_graph_read_access_plan(
        &mut self,
        plan: crate::runtime::WorthQueryAdmittedGraphReadAccessPlan,
        plan_consumption: crate::runtime::WorthQueryGraphReadAccessPlanConsumption,
        ephemeral_graph_index_receipt: Option<crate::runtime::WorthQueryEphemeralGraphIndexReceipt>,
        graph_read_streaming_receipt: Option<crate::runtime::WorthQueryGraphReadStreamingReceipt>,
    ) {
        let graph_read_access_summary =
            crate::runtime::WorthQueryGraphReadAccessReceiptSummary::from_execution_parts(
                self.receipt.read_graph_digest(),
                &plan,
                &plan_consumption,
                ephemeral_graph_index_receipt.as_ref(),
                graph_read_streaming_receipt.as_ref(),
            );
        let graph_read_access_complexity_counters =
            crate::runtime::WorthQueryGraphReadAccessComplexityCounters::from_execution_parts(
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
