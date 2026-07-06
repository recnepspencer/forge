use worth_kernel::workload_composition::{
    current_completed_split_batch_execution_cluster_witness,
    current_lookup_consumed_batch_execution_cluster_witness,
    current_replay_undo_boundary_batch_execution_cluster_witness,
    current_worth_workload_ordinary_consumer_batch_execution_receipt, trace_scope,
    BooleanSplitReplayUndoBoundaryRequest, PlanarBooleanLoopReconstructionCloseoutInput,
    WorkloadCompositionError,
};
use worth_spatial::facade::replay_family_catalog::{
    admit_spatial_replay_family_identity, current_spatial_replay_family_catalog,
    SpatialReplayFamilyIdentityAuthority,
};
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    SpatialReplaySemanticGraphPreparationRequest,
};

use super::continuation_contract_support;
use super::metaboss_support::MetabossEventExtractionSubject;

#[allow(clippy::too_many_arguments)]
pub(super) fn complete_replay_undo_chain_from_boundary(
    subject: &MetabossEventExtractionSubject,
    completed_split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    boundary_request: BooleanSplitReplayUndoBoundaryRequest<'_>,
    replay_parity_receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayParityReceipt,
    replay_receipts: &worth_spatial::facade::retained_replay_workload::ReplayReceiptSet,
    decision_log_receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionLogReceipt,
    validation: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitChainValidationReceipt,
    naming: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitPersistentNamingReceipt,
    ledger: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedger,
    vertices: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentitySet,
    fragments: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
    chains: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapEdgeChainSet,
    split_request: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<
    worth_kernel::workload_composition::BooleanChainReplayUndoBoundaryHandoff,
    WorkloadCompositionError,
> {
    trace_scope("complete_replay_undo_chain_from_boundary", || {
        let recovered_source_carriers =
            continuation_contract_support::recovered_source_carriers(subject, split_request);
        completed_split_handoff
            .admit_batch_execution_cluster()?
            .admit_boolean_split_replay_undo_boundary(boundary_request)?
            .complete_boolean_chain_integration(PlanarBooleanLoopReconstructionCloseoutInput::new(
                decision_log_receipt,
                validation,
                naming,
                replay_parity_receipt,
                ledger,
                &recovered_source_carriers,
                vertices,
                fragments,
                chains,
                replay_receipts,
                matrix,
                validators,
            ))
    })
}

pub(super) fn with_matching_spatial_scope_products<T>(
    subject: &MetabossEventExtractionSubject,
    completed_split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    f: impl for<'a> FnOnce(
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialReplayScopeProduct<'a>,
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialUndoScopeProduct<'a>,
    ) -> T,
) -> T {
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split authority");
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet");
    let request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
            ),
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        )
        .with_retained_replay_receipt(
            completed_split_handoff
                .completed_workload()
                .retained_replay(),
        ),
    )
    .expect("prepared replay request");
    let admitted = admit_prepared_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        &request,
    )
    .expect("admitted replay input");
    let replay_scope =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("replay scope");
    let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
        BooleanEventLedgerRollbackRequest::new(
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff
                .completed_workload()
                .evidence_ledger()
                .stage_index(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        ),
    )
    .expect("undo scope");

    f(&replay_scope, &undo_scope)
}

pub(super) fn current_ordinary_consumer_batch_execution_receipt() -> Result<
    worth_kernel::workload_composition::BatchAdmissionExecutionReceipt,
    WorkloadCompositionError,
> {
    let route_witnesses = [
        current_lookup_consumed_batch_execution_cluster_witness().map_err(current_cutover_error)?,
        current_completed_split_batch_execution_cluster_witness().map_err(current_cutover_error)?,
        current_replay_undo_boundary_batch_execution_cluster_witness()
            .map_err(current_cutover_error)?,
    ];
    current_worth_workload_ordinary_consumer_batch_execution_receipt(&route_witnesses)
        .map_err(current_cutover_error)
}

fn current_cutover_error(error: impl std::fmt::Debug) -> WorkloadCompositionError {
    WorkloadCompositionError::BooleanChainHandoff(format!(
        "current ordinary-consumer batch execution receipt did not assemble: {error:?}"
    ))
}
