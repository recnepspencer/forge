mod grouped_input_denials;

use super::test_support::{ordinary_touched_closure, spatial_evidence_route_fixture};
use super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    lower_selected_batch_admission_plan, BatchAdmissionCandidate, BatchAdmissionFamilyIdentity,
    BatchAdmissionFamilyPosture, BatchAdmissionGroupedInput,
    BatchAdmissionPairwiseIndependenceProof,
};
use crate::workload_composition::conflict_independence::owner_backed_topology_closeout_with_aspect_routing_posture;
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input,
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    prove_spatial_conflict_independence, prove_topology_conflict_independence,
    SpatialConflictIndependenceRequest, SpatialConflictInputRequest,
    TopologyConflictIndependenceRequest, TopologyConflictInputRequest,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;
use worth_spatial::facade::replay_undo_semantic_graph::boolean_event_ledger_spatial_boundary_fixture;
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

#[test]
fn declared_once_batch_family_serves_multiple_grouped_consumers() {
    let topology_left_closure = ordinary_touched_closure(20, 10, 11);
    let topology_right_closure = ordinary_touched_closure(30, 21, 22);
    let topology_closeout =
        current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let topology_left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&topology_left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("topology left admits");
    let topology_right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&topology_right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("topology right admits");
    let topology_left_plan =
        lower_selected_topology_conflict_plan(&topology_closeout, &topology_left);
    let topology_right_plan =
        lower_selected_topology_conflict_plan(&topology_closeout, &topology_right);
    let topology_proof = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&topology_left_plan, &topology_right_plan),
    );

    let spatial_left_fixture = spatial_evidence_route_fixture();
    let spatial_right_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let spatial_closeout =
        current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let spatial_left = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(spatial_left_fixture.authority()).with_evidence_lookup(
            spatial_left_fixture.workload_handoff(),
            spatial_left_fixture.execution_receipt(),
        ),
    )
    .expect("spatial left admits");
    let spatial_right = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(spatial_right_fixture.authority()).with_evidence_lookup(
            spatial_right_fixture.workload_handoff(),
            spatial_right_fixture.execution_receipt(),
        ),
    )
    .expect("spatial right admits");
    let spatial_left_plan = lower_selected_spatial_conflict_plan(&spatial_closeout, &spatial_left);
    let spatial_right_plan =
        lower_selected_spatial_conflict_plan(&spatial_closeout, &spatial_right);
    let spatial_proof = prove_spatial_conflict_independence(
        SpatialConflictIndependenceRequest::new(&spatial_left_plan, &spatial_right_plan),
    );

    let family_closeout = current_batch_admission_family_catalog_closeout();
    let topology_plan = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&topology_left_plan),
                BatchAdmissionCandidate::Topology(&topology_right_plan),
            ])
            .with_pairwise_independence(
                BatchAdmissionPairwiseIndependenceProof::Topology(&topology_proof),
            ),
        )
        .expect("topology group admits"),
    );
    let spatial_plan = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Spatial(&spatial_left_plan),
                BatchAdmissionCandidate::Spatial(&spatial_right_plan),
            ])
            .with_pairwise_independence(
                BatchAdmissionPairwiseIndependenceProof::Spatial(&spatial_proof),
            ),
        )
        .expect("spatial group admits"),
    );

    assert_eq!(
        topology_plan.posture(),
        BatchAdmissionFamilyPosture::ParallelAdmit
    );
    assert_eq!(
        spatial_plan.posture(),
        BatchAdmissionFamilyPosture::ParallelAdmit
    );
    assert_eq!(
        topology_plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::ParallelProjectionConsumption
    );
    assert_eq!(
        spatial_plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::ParallelProjectionConsumption
    );
}

#[test]
fn parallel_admission_requires_explicit_independence_proof() {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let admitted = admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
        BatchAdmissionCandidate::Topology(&left_plan),
        BatchAdmissionCandidate::Topology(&right_plan),
    ]))
    .expect("group admits");

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admitted,
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::DeniedGroupedOverlap
    );
}

#[test]
fn serializable_overlap_selects_serial_family() {
    let closure = ordinary_touched_closure(20, 10, 11);
    let left_closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::SerializableOnly,
    );
    let right_closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::ProvenIndependent,
    );
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&left_closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&right_closeout, &right);
    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("group admits"),
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::SerialAdmit);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::SerializableGroupedOverlap
    );
}

#[test]
fn caller_ordering_cannot_mint_batch_plan_identity() {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let catalog_closeout = current_batch_admission_family_catalog_closeout();

    let first = lower_selected_batch_admission_plan(
        &catalog_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("group admits"),
    );
    let second = lower_selected_batch_admission_plan(
        &catalog_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&right_plan),
                BatchAdmissionCandidate::Topology(&left_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("group admits"),
    );

    assert_eq!(first.posture(), second.posture());
    assert_eq!(first.selected_plan_digest(), second.selected_plan_digest());
    assert_eq!(
        first.selected_family_rows()[0].identity(),
        second.selected_family_rows()[0].identity()
    );
}
