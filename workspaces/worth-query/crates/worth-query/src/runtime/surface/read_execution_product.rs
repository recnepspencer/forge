use crate::runtime::{
    WorthQueryAdmittedGraphReadAccessPlan, WorthQueryAuthoritativeMutationObligationDispatch,
    WorthQueryEphemeralGraphIndexReceipt, WorthQueryGraphReadAccessComplexityCounters,
    WorthQueryGraphReadAccessPlanConsumption, WorthQueryGraphReadAccessReceiptSummary,
    WorthQueryGraphReadStreamingReceipt, WorthQueryIntentDecisionTraceEnvelope,
    WorthQueryIntentExecutionProvenance,
};

use super::WorthQueryReadReceipt;

pub(in crate::runtime) trait WorthQueryReadExecutionProduct {
    fn receipt(&self) -> &WorthQueryReadReceipt;
    fn receipt_mut(&mut self) -> &mut WorthQueryReadReceipt;
    fn output_cardinality(&self) -> usize;

    fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: WorthQueryIntentDecisionTraceEnvelope,
        execution_provenance: WorthQueryIntentExecutionProvenance,
    ) {
        self.receipt_mut().decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt_mut().execution_provenance = Some(execution_provenance);
    }

    fn attach_graph_obligation_dispatch(
        &mut self,
        dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    ) {
        self.receipt_mut().graph_obligation_dispatch = dispatch;
    }

    fn attach_graph_read_access_plan(
        &mut self,
        plan: WorthQueryAdmittedGraphReadAccessPlan,
        plan_consumption: WorthQueryGraphReadAccessPlanConsumption,
        ephemeral_graph_index_receipt: Option<WorthQueryEphemeralGraphIndexReceipt>,
        graph_read_streaming_receipt: Option<WorthQueryGraphReadStreamingReceipt>,
    ) {
        let graph_read_access_summary =
            WorthQueryGraphReadAccessReceiptSummary::from_execution_parts(
                self.receipt().read_graph_digest(),
                &plan,
                &plan_consumption,
                ephemeral_graph_index_receipt.as_ref(),
                graph_read_streaming_receipt.as_ref(),
            );
        let graph_read_access_complexity_counters =
            WorthQueryGraphReadAccessComplexityCounters::from_execution_parts(
                &plan,
                &plan_consumption,
                ephemeral_graph_index_receipt.as_ref(),
                graph_read_streaming_receipt.as_ref(),
            );
        let receipt = self.receipt_mut();
        receipt.graph_read_access_plan = Some(plan);
        receipt.graph_read_access_plan_consumption = Some(plan_consumption);
        receipt.ephemeral_graph_index_receipt = ephemeral_graph_index_receipt;
        receipt.graph_read_streaming_receipt = graph_read_streaming_receipt;
        receipt.graph_read_access_summary = Some(graph_read_access_summary);
        receipt.graph_read_access_complexity_counters = Some(graph_read_access_complexity_counters);
    }
}
