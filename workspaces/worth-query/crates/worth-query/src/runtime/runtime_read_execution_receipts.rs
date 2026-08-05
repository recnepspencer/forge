use super::*;
use crate::intent_admission::WorthQueryReadExecutionBinding;

pub(crate) fn provision_graph_indexes_for_read_binding(
    binding: &WorthQueryReadExecutionBinding,
    snapshot_identity_digest: &str,
) -> Result<Option<WorthQueryEphemeralGraphIndexReceipt>, WorthQueryRuntimeError> {
    provision_ephemeral_graph_indexes_for_read_execution(
        binding.graph_read_access_plan(),
        snapshot_identity_digest,
    )
    .map_err(|error| {
        WorthQueryRuntimeError::ReadCompositionDenied(WorthQueryReadDenial::new(
            WorthQueryReadDenialKind::BasisPreflightDenied,
            error.as_str(),
        ))
    })
}

pub(crate) fn attach_graph_read_access_receipt<Product>(
    executed_read: &mut super::read_composition_runtime::WorthQueryExecutedReadProduct<Product>,
    binding: &WorthQueryReadExecutionBinding,
    snapshot_identity_digest: &str,
    ephemeral_graph_index_receipt: Option<WorthQueryEphemeralGraphIndexReceipt>,
) where
    Product: WorthQueryReadExecutionProduct,
{
    let graph_read_access_plan_consumption =
        WorthQueryGraphReadAccessPlanConsumption::from_plan_binding_and_execution_counters(
            binding.graph_read_access_plan(),
            binding.binding_digest(),
            executed_read.graph_read_access_execution_counters().clone(),
        );
    let graph_read_streaming_receipt = streaming_receipt_for_admitted_read_result(
        binding.graph_read_access_plan(),
        snapshot_identity_digest,
        executed_read.product().receipt().result_digest(),
        executed_read.product().output_cardinality(),
    );
    executed_read.product_mut().attach_graph_read_access_plan(
        binding.graph_read_access_plan().clone(),
        graph_read_access_plan_consumption,
        ephemeral_graph_index_receipt,
        graph_read_streaming_receipt,
    );
}

pub(crate) fn attach_read_intent_execution_evidence<Product>(
    executed_read: &mut super::read_composition_runtime::WorthQueryExecutedReadProduct<Product>,
    binding: &WorthQueryReadExecutionBinding,
    snapshot_identity: &crate::WorthQueryEvidenceIdentity,
) where
    Product: WorthQueryReadExecutionProduct,
{
    let decision_trace_envelope =
        WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
            binding.family(),
            binding.entrypoint(),
            binding.read_family().family_name(),
            binding.handoff().request_digest(),
            binding.handoff().eligibility_trace().clone(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.execution_seam(),
            binding.read_family().family_name(),
            executed_read.product().receipt().result_digest(),
            binding.execution_seam().as_str(),
        );
    let execution_provenance =
        WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            executed_read.product().receipt().result_digest(),
            snapshot_identity,
        );
    executed_read
        .product_mut()
        .attach_intent_admission_evidence(decision_trace_envelope, execution_provenance);
}
