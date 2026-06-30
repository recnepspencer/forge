use super::test_support::ordinary_touched_closure;
use super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    lower_selected_batch_admission_plan, BatchAdmissionCandidate, BatchAdmissionFamilyPosture,
    BatchAdmissionGroupedInput, BatchAdmissionPairwiseIndependenceProof,
    BatchAdmissionSupportingConflictLane,
};
use crate::workload_composition::{
    admit_topology_conflict_input, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, TopologyConflictIndependenceRequest,
    TopologyConflictInputRequest,
};
use topology::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictFamilyIdentity,
};

#[test]
fn same_conflict_inputs_produce_same_batch_plan_digest() {
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
    let family_closeout = current_batch_admission_family_catalog_closeout();

    let first = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("first group admits"),
    );
    let second = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("second group admits"),
    );

    assert_eq!(first.posture(), BatchAdmissionFamilyPosture::ParallelAdmit);
    assert_eq!(first.selected_plan_digest(), second.selected_plan_digest());
}

#[test]
fn selected_batch_plan_carries_supporting_conflict_family_rows() {
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

    let supporting_rows = plan.supporting_conflict_family_rows();
    assert_eq!(supporting_rows.len(), 2);
    assert!(supporting_rows
        .iter()
        .all(|row| row.conflict_lane() == BatchAdmissionSupportingConflictLane::Topology));
    assert!(supporting_rows.iter().all(|row| {
        row.conflict_family_identity() == TopologyConflictFamilyIdentity::AspectSelection.as_str()
    }));
    assert!(supporting_rows.iter().any(|row| {
        row.participant_identity() == left_plan.selected_plan_digest()
            && row.declaration_digest() == left_plan.selected_families()[0].declaration_digest()
    }));
    assert!(supporting_rows.iter().any(|row| {
        row.participant_identity() == right_plan.selected_plan_digest()
            && row.declaration_digest() == right_plan.selected_families()[0].declaration_digest()
    }));
}

#[test]
fn selected_batch_plan_digest_changes_when_posture_changes() {
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
    let family_closeout = current_batch_admission_family_catalog_closeout();

    let denied = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&left_plan),
            BatchAdmissionCandidate::Topology(&right_plan),
        ]))
        .expect("denied group admits"),
    );
    let parallel = lower_selected_batch_admission_plan(
        &family_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("parallel group admits"),
    );

    assert_ne!(denied.posture(), parallel.posture());
    assert_ne!(
        denied.selected_plan_digest(),
        parallel.selected_plan_digest()
    );
}
