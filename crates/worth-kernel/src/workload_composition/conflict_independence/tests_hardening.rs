use super::test_support::{
    ordinary_touched_closure, owner_backed_spatial_closeout_with_replay_prior_proof_posture,
    owner_backed_topology_closeout_with_replay_prior_proof_posture, packet_backed_boundary,
    spatial_evidence_route_fixture,
};
use super::{
    prove_spatial_conflict_independence, prove_topology_conflict_independence,
    SpatialConflictIndependenceDenialKind, SpatialConflictIndependenceRequest,
    TopologyConflictIndependenceDenialKind, TopologyConflictIndependenceRequest,
};
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input,
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    SpatialConflictInputRequest, TopologyConflictInputRequest,
};
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;
use topology::touched_graph_conflict::TopologyConflictPriorProofPosture;
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;
use worth_spatial::touched_graph_conflict::SpatialConflictPriorProofPosture;

#[test]
fn topology_independence_denies_selected_plan_before_overlap_reasoning() {
    let closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase6.topology.selected-plan-denied");
    let closeout = owner_backed_topology_closeout_with_replay_prior_proof_posture(
        TopologyConflictPriorProofPosture::NoPriorProofRequired,
    );
    let left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure).with_replay_boundary(&boundary),
    )
    .expect("left admits");
    let right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure).with_replay_boundary(&boundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right_admitted);

    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.denial().expect("denial").kind(),
        TopologyConflictIndependenceDenialKind::SelectedPlanDenied
    );
}

#[test]
fn topology_independence_proof_digest_is_pair_order_canonical() {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right_admitted);

    let canonical = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let reordered = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &right_plan,
        &left_plan,
    ));

    assert_eq!(canonical.disposition(), reordered.disposition());
    assert_eq!(canonical.proof_digest(), reordered.proof_digest());
}

#[test]
fn spatial_independence_denies_selected_plan_before_overlap_reasoning() {
    let boundary = packet_backed_boundary("phase6.spatial.selected-plan-denied");
    let authority = boundary
        .completed_split_handoff()
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let closeout = owner_backed_spatial_closeout_with_replay_prior_proof_posture(
        SpatialConflictPriorProofPosture::NoPriorProofRequired,
    );
    let left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("left admits");
    let right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right_admitted);

    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.denial().expect("denial").kind(),
        SpatialConflictIndependenceDenialKind::SelectedPlanDenied
    );
}

#[test]
fn spatial_independence_proof_digest_is_pair_order_canonical() {
    let left_fixture = spatial_evidence_route_fixture("phase6.spatial.canonical.left");
    let right_fixture = spatial_evidence_route_fixture("phase6.spatial.canonical.right");
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(left_fixture.authority()).with_evidence_lookup(
            left_fixture.workload_handoff(),
            left_fixture.execution_receipt(),
        ),
    )
    .expect("left admits");
    let right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(right_fixture.authority()).with_evidence_lookup(
            right_fixture.workload_handoff(),
            right_fixture.execution_receipt(),
        ),
    )
    .expect("right admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right_admitted);

    let canonical = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let reordered = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &right_plan,
        &left_plan,
    ));

    assert_eq!(canonical.disposition(), reordered.disposition());
    assert_eq!(canonical.proof_digest(), reordered.proof_digest());
}
