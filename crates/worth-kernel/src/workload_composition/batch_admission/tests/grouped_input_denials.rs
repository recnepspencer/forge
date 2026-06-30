use super::super::test_support::ordinary_touched_closure;
use super::super::{
    admit_batch_admission_grouped_input, BatchAdmissionCandidate, BatchAdmissionGroupedInput,
    BatchAdmissionGroupedInputAdmissionErrorKind, BatchAdmissionPairwiseIndependenceProof,
};
use crate::workload_composition::conflict_independence::owner_backed_topology_closeout_with_aspect_routing_posture;
use crate::workload_composition::{
    admit_topology_conflict_input, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, TopologyConflictIndependenceRequest,
    TopologyConflictInputRequest,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;

#[test]
fn duplicate_selected_plan_identity_is_rejected_at_group_admission() {
    let closure = ordinary_touched_closure(20, 10, 11);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
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
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);

    let error = match admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
        BatchAdmissionCandidate::Topology(&left_plan),
        BatchAdmissionCandidate::Topology(&right_plan),
    ])) {
        Ok(_) => panic!("duplicate selected plan identities must deny grouped admission"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        BatchAdmissionGroupedInputAdmissionErrorKind::DuplicateSelectedPlanIdentity
    );
}

#[test]
fn duplicate_pairwise_proof_coverage_is_rejected_at_group_admission() {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::ProvenIndependent,
    );
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

    let error = match admit_batch_admission_grouped_input(
        BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&left_plan),
            BatchAdmissionCandidate::Topology(&right_plan),
        ])
        .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof))
        .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
    ) {
        Ok(_) => panic!("duplicate unordered pair proof coverage must deny grouped admission"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        BatchAdmissionGroupedInputAdmissionErrorKind::DuplicatePairwiseProofCoverage
    );
}

#[test]
fn self_bound_pairwise_proof_is_rejected_at_group_admission() {
    let shared_closure = ordinary_touched_closure(20, 10, 11);
    let disjoint_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let shared = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&shared_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("shared admits");
    let disjoint = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&disjoint_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("disjoint admits");
    let shared_plan = lower_selected_topology_conflict_plan(&closeout, &shared);
    let disjoint_plan = lower_selected_topology_conflict_plan(&closeout, &disjoint);
    let self_bound_proof = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&shared_plan, &shared_plan),
    );

    let error = match admit_batch_admission_grouped_input(
        BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&shared_plan),
            BatchAdmissionCandidate::Topology(&disjoint_plan),
        ])
        .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(
            &self_bound_proof,
        )),
    ) {
        Ok(_) => panic!("self-bound pairwise proof must deny grouped admission"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        BatchAdmissionGroupedInputAdmissionErrorKind::ProofDoesNotBindDistinctParticipants
    );
}

#[test]
fn out_of_group_pairwise_proof_is_rejected_at_group_admission() {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let middle_closure = ordinary_touched_closure(30, 21, 22);
    let right_closure = ordinary_touched_closure(40, 31, 32);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let middle = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&middle_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("middle admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let middle_plan = lower_selected_topology_conflict_plan(&closeout, &middle);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let out_of_group_proof = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &right_plan),
    );

    let error = match admit_batch_admission_grouped_input(
        BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&left_plan),
            BatchAdmissionCandidate::Topology(&middle_plan),
        ])
        .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(
            &out_of_group_proof,
        )),
    ) {
        Ok(_) => panic!("out-of-group pairwise proof must deny grouped admission"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        BatchAdmissionGroupedInputAdmissionErrorKind::ProofEndpointNotInGroup
    );
    assert_ne!(
        right_plan.selected_plan_digest(),
        middle_plan.selected_plan_digest()
    );
}
