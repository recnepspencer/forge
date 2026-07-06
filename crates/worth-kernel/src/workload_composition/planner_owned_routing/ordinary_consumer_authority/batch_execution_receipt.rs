use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

use super::cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use super::lookup_route_authority::WorthWorkloadCurrentLookupConsumedRouteAuthority;
use super::route_witness::WorthWorkloadOrdinaryConsumerCurrentRouteWitness;
use crate::workload_composition::compiled_product_consumer_cutover::vertical_slice::lookup_consumed::LookupConsumedVerticalSlice;
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    execute_selected_batch_admission_plan, lower_selected_batch_admission_plan,
    lower_selected_spatial_conflict_plan, prove_spatial_conflict_independence,
    BatchAdmissionCandidate, BatchAdmissionExecutionReceipt, BatchAdmissionGroupedInput,
    BatchAdmissionPairwiseIndependenceProof, SpatialConflictIndependenceRequest,
};

pub fn current_worth_workload_ordinary_consumer_batch_execution_receipt(
    route_witnesses: &[WorthWorkloadOrdinaryConsumerCurrentRouteWitness],
) -> Result<BatchAdmissionExecutionReceipt, WorthWorkloadOrdinaryConsumerCutoverError> {
    trace_scope(
        "current_worth_workload_ordinary_consumer_batch_execution_receipt",
        || {
            let lookup_route_authority =
                WorthWorkloadOrdinaryConsumerCurrentRouteWitness::require_same_lookup_route_authority(
                    route_witnesses,
                )?;
            lower_batch_execution_receipt_from_lookup_route(&lookup_route_authority)
        },
    )
}

fn lower_batch_execution_receipt_from_lookup_route(
    lookup_route_authority: &WorthWorkloadCurrentLookupConsumedRouteAuthority,
) -> Result<BatchAdmissionExecutionReceipt, WorthWorkloadOrdinaryConsumerCutoverError> {
    trace_scope("lower_batch_execution_receipt_from_lookup_route", || {
        let left = lookup_route_authority.left_boundary();
        let right = lookup_route_authority.right_boundary();
        let left_slice = trace_scope("left_lookup_slice_admit", || {
            LookupConsumedVerticalSlice::admit(left).map_err(current_proof_error)
        })?;
        let left_resolved = trace_scope("left_lookup_resolve_prior_product", || {
            left_slice
                .resolve_prior_lookup_product(left.index_product())
                .map_err(current_proof_error)
        })?;
        let left_input = trace_scope("left_lookup_admit_spatial_conflict_input", || {
            left_resolved
                .admit_spatial_conflict_input()
                .map_err(current_proof_error)
        })?;
        let right_slice = trace_scope("right_lookup_slice_admit", || {
            LookupConsumedVerticalSlice::admit(right).map_err(current_proof_error)
        })?;
        let right_resolved = trace_scope("right_lookup_resolve_prior_product", || {
            right_slice
                .resolve_prior_lookup_product(right.index_product())
                .map_err(current_proof_error)
        })?;
        let right_input = trace_scope("right_lookup_admit_spatial_conflict_input", || {
            right_resolved
                .admit_spatial_conflict_input()
                .map_err(current_proof_error)
        })?;
        let closeout = trace_scope("current_spatial_conflict_family_catalog_closeout", || {
            current_spatial_conflict_family_catalog_closeout().map_err(current_proof_error)
        })?;
        let left_plan = trace_scope("lower_left_selected_spatial_conflict_plan", || {
            lower_selected_spatial_conflict_plan(&closeout, &left_input)
        });
        let right_plan = trace_scope("lower_right_selected_spatial_conflict_plan", || {
            lower_selected_spatial_conflict_plan(&closeout, &right_input)
        });
        let proof = trace_scope("prove_spatial_conflict_independence", || {
            prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
                &left_plan,
                &right_plan,
            ))
        });
        let grouped_input = BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Spatial(&left_plan),
            BatchAdmissionCandidate::Spatial(&right_plan),
        ])
        .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Spatial(&proof));
        let admitted = trace_scope("admit_batch_admission_grouped_input", || {
            admit_batch_admission_grouped_input(grouped_input).map_err(current_proof_error)
        })?;
        let selected_plan = trace_scope("lower_selected_batch_admission_plan", || {
            lower_selected_batch_admission_plan(
                &current_batch_admission_family_catalog_closeout(),
                &admitted,
            )
        });
        Ok(trace_scope("execute_selected_batch_admission_plan", || {
            execute_selected_batch_admission_plan(&selected_plan)
        }))
    })
}

fn current_proof_error<E: std::fmt::Debug>(error: E) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current ordinary cutover proof did not assemble: {error:?}"),
    )
}
