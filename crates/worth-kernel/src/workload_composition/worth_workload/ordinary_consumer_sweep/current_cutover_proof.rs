use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

use super::current_cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use super::current_route_authority::WorthWorkloadCurrentLookupConsumedRouteAuthority;
use super::current_route_witness::WorthWorkloadOrdinaryConsumerCurrentRouteWitness;
use crate::workload_composition::{
    admit_batch_admission_grouped_input,
    compiled_product_consumer_cutover::vertical_slice::lookup_consumed::LookupConsumedVerticalSlice,
    current_batch_admission_family_catalog_closeout, execute_selected_batch_admission_plan,
    lower_selected_batch_admission_plan, lower_selected_spatial_conflict_plan,
    prove_spatial_conflict_independence, BatchAdmissionCandidate, BatchAdmissionExecutionReceipt,
    BatchAdmissionGroupedInput, BatchAdmissionPairwiseIndependenceProof,
    SpatialConflictIndependenceRequest,
};

pub(crate) fn current_worth_workload_ordinary_consumer_batch_execution_receipt(
    route_witnesses: &[WorthWorkloadOrdinaryConsumerCurrentRouteWitness],
) -> Result<BatchAdmissionExecutionReceipt, WorthWorkloadOrdinaryConsumerCutoverError> {
    let lookup_route_authority =
        WorthWorkloadOrdinaryConsumerCurrentRouteWitness::require_same_lookup_route_authority(
            route_witnesses,
        )?;
    lower_batch_execution_receipt_from_lookup_route(&lookup_route_authority)
}

fn lower_batch_execution_receipt_from_lookup_route(
    lookup_route_authority: &WorthWorkloadCurrentLookupConsumedRouteAuthority,
) -> Result<BatchAdmissionExecutionReceipt, WorthWorkloadOrdinaryConsumerCutoverError> {
    let left = lookup_route_authority.left_boundary();
    let right = lookup_route_authority.right_boundary();
    let left_slice = LookupConsumedVerticalSlice::admit(left).map_err(current_proof_error)?;
    let left_resolved = left_slice
        .resolve_prior_lookup_product(left.index_product())
        .map_err(current_proof_error)?;
    let left_input = left_resolved
        .admit_spatial_conflict_input()
        .map_err(current_proof_error)?;
    let right_slice = LookupConsumedVerticalSlice::admit(right).map_err(current_proof_error)?;
    let right_resolved = right_slice
        .resolve_prior_lookup_product(right.index_product())
        .map_err(current_proof_error)?;
    let right_input = right_resolved
        .admit_spatial_conflict_input()
        .map_err(current_proof_error)?;
    let closeout =
        current_spatial_conflict_family_catalog_closeout().map_err(current_proof_error)?;
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left_input);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right_input);
    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let grouped_input = BatchAdmissionGroupedInput::new([
        BatchAdmissionCandidate::Spatial(&left_plan),
        BatchAdmissionCandidate::Spatial(&right_plan),
    ])
    .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Spatial(&proof));
    let admitted =
        admit_batch_admission_grouped_input(grouped_input).map_err(current_proof_error)?;
    let selected_plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admitted,
    );
    Ok(execute_selected_batch_admission_plan(&selected_plan))
}

fn current_proof_error<E: std::fmt::Debug>(error: E) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current ordinary cutover proof did not assemble: {error:?}"),
    )
}
