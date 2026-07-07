use worth_spatial::facade::replay_undo_semantic_graph::{
    boolean_event_ledger_query_required_sibling_spatial_boundary_fixture,
    boolean_event_ledger_spatial_boundary_fixture,
};
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

use crate::workload_composition::{
    admit_spatial_conflict_input, lower_selected_spatial_conflict_plan,
    prove_spatial_conflict_independence, SpatialConflictIndependenceRequest,
    SpatialConflictInputRequest,
};

use super::spatial_batch_execution_slice::DerivedSpatialBatchExecutionSlice;

pub(crate) fn denied_same_participant_spatial_batch_execution_slice(
) -> DerivedSpatialBatchExecutionSlice {
    let left_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let right_fixture = boolean_event_ledger_query_required_sibling_spatial_boundary_fixture();
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(left_fixture.authority()).with_evidence_lookup(
            left_fixture.workload_handoff(),
            left_fixture.execution_receipt(),
        ),
    )
    .expect("left spatial conflict input admits");
    let right = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(right_fixture.authority()).with_evidence_lookup(
            right_fixture.workload_handoff(),
            right_fixture.execution_receipt(),
        ),
    )
    .expect("right spatial conflict input admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right);
    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    DerivedSpatialBatchExecutionSlice::from_spatial_proof(
        &left_plan,
        &right_plan,
        proof.disposition(),
        None,
    )
}
