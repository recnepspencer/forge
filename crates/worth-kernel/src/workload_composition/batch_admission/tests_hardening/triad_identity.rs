use super::super::test_support::{ordinary_touched_closure, packet_backed_boundary};
use super::super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    lower_selected_batch_admission_plan, BatchAdmissionCandidate, BatchAdmissionFamilyIdentity,
    BatchAdmissionFamilyPosture, BatchAdmissionGroupedInput,
    BatchAdmissionPairwiseIndependenceProof, BatchAdmissionPlanDenialKind,
    BatchAdmissionSupportingConflictLane,
};
use crate::workload_composition::conflict_independence::owner_backed_topology_closeout_with_replay_prior_proof_posture;
use crate::workload_composition::{
    admit_topology_conflict_input, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, TopologyConflictIndependenceRequest,
    TopologyConflictInputRequest,
};
use topology::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictPriorProofPosture,
};

#[test]
fn triad_group_parallel_requires_all_unordered_pairs() {
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let middle_closure = ordinary_touched_closure(30, 21, 22);
    let right_closure = ordinary_touched_closure(40, 31, 32);
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
    let left_middle = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &middle_plan),
    );
    let left_right = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &right_plan),
    );
    let middle_right = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&middle_plan, &right_plan),
    );

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&middle_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(
                &left_middle,
            ))
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(
                &left_right,
            ))
            .with_pairwise_independence(
                BatchAdmissionPairwiseIndependenceProof::Topology(&middle_right),
            ),
        )
        .expect("group admits"),
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::ParallelAdmit);
    let mut expected_participants = vec![
        left_plan.selected_plan_digest().to_string(),
        middle_plan.selected_plan_digest().to_string(),
        right_plan.selected_plan_digest().to_string(),
    ];
    expected_participants.sort();
    assert_eq!(
        plan.participant_identities(),
        expected_participants.as_slice()
    );
    let parallel_edges = plan
        .parallel_admission_edges()
        .iter()
        .map(|edge| {
            (
                edge.left_participant_identity().to_string(),
                edge.right_participant_identity().to_string(),
                edge.proof_digest().to_string(),
            )
        })
        .collect::<Vec<_>>();
    let mut expected_parallel_edges = vec![
        (
            left_plan.selected_plan_digest().to_string(),
            middle_plan.selected_plan_digest().to_string(),
            left_middle.proof_digest().to_string(),
        ),
        (
            left_plan.selected_plan_digest().to_string(),
            right_plan.selected_plan_digest().to_string(),
            left_right.proof_digest().to_string(),
        ),
        (
            middle_plan.selected_plan_digest().to_string(),
            right_plan.selected_plan_digest().to_string(),
            middle_right.proof_digest().to_string(),
        ),
    ];
    expected_parallel_edges.sort();
    assert_eq!(parallel_edges, expected_parallel_edges);
    assert!(plan.serial_admission_edges().is_empty());
    let supporting_rows = plan
        .supporting_conflict_family_rows()
        .iter()
        .map(|row| {
            (
                row.participant_identity().to_string(),
                row.conflict_lane(),
                row.conflict_family_identity().to_string(),
                row.declaration_digest().to_string(),
            )
        })
        .collect::<Vec<_>>();
    let mut expected_supporting_rows = vec![
        (
            left_plan.selected_plan_digest().to_string(),
            BatchAdmissionSupportingConflictLane::Topology,
            left_plan.selected_families()[0]
                .identity()
                .as_str()
                .to_string(),
            left_plan.selected_families()[0]
                .declaration_digest()
                .to_string(),
        ),
        (
            middle_plan.selected_plan_digest().to_string(),
            BatchAdmissionSupportingConflictLane::Topology,
            middle_plan.selected_families()[0]
                .identity()
                .as_str()
                .to_string(),
            middle_plan.selected_families()[0]
                .declaration_digest()
                .to_string(),
        ),
        (
            right_plan.selected_plan_digest().to_string(),
            BatchAdmissionSupportingConflictLane::Topology,
            right_plan.selected_families()[0]
                .identity()
                .as_str()
                .to_string(),
            right_plan.selected_families()[0]
                .declaration_digest()
                .to_string(),
        ),
    ];
    expected_supporting_rows.sort();
    assert_eq!(supporting_rows, expected_supporting_rows);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::ParallelProjectionConsumption
    );
}

#[test]
fn selected_plan_denial_is_preserved_in_batch_plan_denial_kind() {
    let denied_closeout = owner_backed_topology_closeout_with_replay_prior_proof_posture(
        TopologyConflictPriorProofPosture::NoPriorProofRequired,
    );
    let admitted_closeout =
        current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let denied_boundary = packet_backed_boundary("phase7.batch.topology.denied");
    let admitted_boundary = packet_backed_boundary("phase7.batch.topology.admitted");
    let denied_input = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&denied_boundary),
    )
    .expect("denied input admits");
    let admitted_input = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure)
            .with_replay_boundary(&admitted_boundary),
    )
    .expect("admitted input admits");
    let denied_plan = lower_selected_topology_conflict_plan(&denied_closeout, &denied_input);
    let admitted_plan = lower_selected_topology_conflict_plan(&admitted_closeout, &admitted_input);

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&denied_plan),
            BatchAdmissionCandidate::Topology(&admitted_plan),
        ]))
        .expect("group admits"),
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::DeniedGroupedOverlap
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        BatchAdmissionPlanDenialKind::SelectedPlanDenied
    );
}
