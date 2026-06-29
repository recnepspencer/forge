use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;

use super::test_support::{
    ordinary_touched_closure, owner_backed_spatial_closeout_with_evidence_routing_posture,
    owner_backed_topology_closeout_with_aspect_routing_posture, spatial_evidence_route_fixture,
};
use super::{
    prove_spatial_conflict_independence, prove_topology_conflict_independence,
    ConflictIndependenceDisposition, SpatialConflictIndependenceDenialKind,
    SpatialConflictIndependenceRequest, TopologyConflictIndependenceDenialKind,
    TopologyConflictIndependenceRequest,
};
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input,
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    SpatialConflictInputRequest, TopologyConflictInputRequest,
};
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;
use worth_spatial::facade::replay_undo_semantic_graph::boolean_event_ledger_spatial_boundary_fixture;
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

#[test]
fn topology_independence_proves_disjoint_locality() {
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

    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.disposition(),
        ConflictIndependenceDisposition::Disjoint
    );
    assert!(proof.denial().is_none());
}

#[test]
fn compatible_aspect_overlap_stays_distinct_from_disjointness() {
    let shared_closure = ordinary_touched_closure(20, 10, 11);
    let compatible_closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::ProvenIndependent,
    );
    let compatible_left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&shared_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("compatible left admits");
    let compatible_right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&shared_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("compatible right admits");
    let compatible_left =
        lower_selected_topology_conflict_plan(&compatible_closeout, &compatible_left_admitted);
    let compatible_right =
        lower_selected_topology_conflict_plan(&compatible_closeout, &compatible_right_admitted);
    let compatible = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&compatible_left, &compatible_right),
    );

    let disjoint_left = ordinary_touched_closure(40, 31, 32);
    let disjoint_right = ordinary_touched_closure(50, 41, 42);
    let disjoint_closeout =
        current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let disjoint_left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&disjoint_left)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("disjoint left admits");
    let disjoint_right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&disjoint_right)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("disjoint right admits");
    let disjoint_left_plan =
        lower_selected_topology_conflict_plan(&disjoint_closeout, &disjoint_left_admitted);
    let disjoint_right_plan =
        lower_selected_topology_conflict_plan(&disjoint_closeout, &disjoint_right_admitted);
    let disjoint = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &disjoint_left_plan,
        &disjoint_right_plan,
    ));

    assert_eq!(
        compatible.disposition(),
        ConflictIndependenceDisposition::CompatibleAspectOverlap
    );
    assert_eq!(
        disjoint.disposition(),
        ConflictIndependenceDisposition::Disjoint
    );
    assert_ne!(compatible.proof_digest(), disjoint.proof_digest());
}

#[test]
fn topology_independence_proves_serializable_only_overlap() {
    let closure = ordinary_touched_closure(20, 10, 11);
    let closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::SerializableOnly,
    );
    let left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right_admitted);

    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.disposition(),
        ConflictIndependenceDisposition::SerializableOnly
    );
    assert!(proof.denial().is_none());
}

#[test]
fn executor_cannot_fabricate_independence_from_selected_plan_success() {
    let closure = ordinary_touched_closure(20, 10, 11);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right_admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right_admitted);

    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(proof.disposition(), ConflictIndependenceDisposition::Denied);
    assert_eq!(
        proof.denial().expect("denial").kind(),
        TopologyConflictIndependenceDenialKind::MissingPositiveProof
    );
}

#[test]
fn spatial_independence_proves_disjoint_locality() {
    let left_fixture = spatial_evidence_route_fixture("phase6.spatial.left");
    let right_fixture = spatial_evidence_route_fixture("phase6.spatial.right");
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

    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.disposition(),
        ConflictIndependenceDisposition::Disjoint
    );
    assert!(proof.denial().is_none());
}

#[test]
fn spatial_compatible_overlap_stays_distinct_from_disjointness() {
    let compatible_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let compatible_closeout = owner_backed_spatial_closeout_with_evidence_routing_posture(
        ConflictRoutingPosture::ProvenIndependent,
    );
    let compatible_left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(compatible_fixture.authority()).with_evidence_lookup(
            compatible_fixture.workload_handoff(),
            compatible_fixture.execution_receipt(),
        ),
    )
    .expect("compatible left admits");
    let compatible_right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(compatible_fixture.authority()).with_evidence_lookup(
            compatible_fixture.workload_handoff(),
            compatible_fixture.execution_receipt(),
        ),
    )
    .expect("compatible right admits");
    let compatible_left =
        lower_selected_spatial_conflict_plan(&compatible_closeout, &compatible_left_admitted);
    let compatible_right =
        lower_selected_spatial_conflict_plan(&compatible_closeout, &compatible_right_admitted);
    let compatible = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &compatible_left,
        &compatible_right,
    ));

    let disjoint_left_fixture = spatial_evidence_route_fixture("phase6.spatial.disjoint.left");
    let disjoint_right_fixture = spatial_evidence_route_fixture("phase6.spatial.disjoint.right");
    let disjoint_closeout =
        current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let disjoint_left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(disjoint_left_fixture.authority()).with_evidence_lookup(
            disjoint_left_fixture.workload_handoff(),
            disjoint_left_fixture.execution_receipt(),
        ),
    )
    .expect("disjoint left admits");
    let disjoint_right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(disjoint_right_fixture.authority()).with_evidence_lookup(
            disjoint_right_fixture.workload_handoff(),
            disjoint_right_fixture.execution_receipt(),
        ),
    )
    .expect("disjoint right admits");
    let disjoint_left_plan =
        lower_selected_spatial_conflict_plan(&disjoint_closeout, &disjoint_left_admitted);
    let disjoint_right_plan =
        lower_selected_spatial_conflict_plan(&disjoint_closeout, &disjoint_right_admitted);
    let disjoint = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &disjoint_left_plan,
        &disjoint_right_plan,
    ));

    assert_eq!(
        compatible.disposition(),
        ConflictIndependenceDisposition::CompatibleAspectOverlap
    );
    assert_eq!(
        disjoint.disposition(),
        ConflictIndependenceDisposition::Disjoint
    );
    assert_ne!(compatible.proof_digest(), disjoint.proof_digest());
}

#[test]
fn spatial_independence_proves_serializable_only_overlap() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let closeout = owner_backed_spatial_closeout_with_evidence_routing_posture(
        ConflictRoutingPosture::SerializableOnly,
    );
    let left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("left admits");
    let right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("right admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right_admitted);

    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(
        proof.disposition(),
        ConflictIndependenceDisposition::SerializableOnly
    );
    assert!(proof.denial().is_none());
}

#[test]
fn spatial_independence_denies_without_positive_proof() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let left_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("left admits");
    let right_admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("right admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left_admitted);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right_admitted);

    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    assert_eq!(proof.disposition(), ConflictIndependenceDisposition::Denied);
    assert_eq!(
        proof.denial().expect("denial").kind(),
        SpatialConflictIndependenceDenialKind::MissingPositiveProof
    );
}
