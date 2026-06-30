use super::super::test_support::{
    alternate_packet_backed_boundary, ordinary_touched_closure, packet_backed_boundary,
};
use super::super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    execute_selected_batch_admission_plan, lower_selected_batch_admission_plan,
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionCandidate, BatchAdmissionFamilyIdentity,
    BatchAdmissionFamilyPosture, BatchAdmissionGroupedInput,
    BatchAdmissionPairwiseIndependenceProof, BatchAdmissionPlanDenialKind,
};
use crate::workload_composition::conflict_independence::{
    owner_backed_topology_closeout_with_aspect_routing_posture,
    owner_backed_topology_closeout_with_replay_prior_proof_posture,
};
use crate::workload_composition::{
    admit_topology_conflict_input, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, TopologyConflictIndependenceRequest,
    TopologyConflictInputRequest,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;
use topology::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictPriorProofPosture,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn execution_receipt_binds_selected_batch_plan_and_independence_proof_chain() {
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
    let selected_plan = lower_selected_batch_admission_plan(
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

    let receipt = execute_selected_batch_admission_plan(&selected_plan);

    assert_eq!(
        receipt.posture(),
        BatchAdmissionFamilyPosture::ParallelAdmit
    );
    assert_eq!(
        receipt.selected_batch_plan_digest(),
        selected_plan.selected_plan_digest()
    );
    assert_eq!(
        receipt.participant_identities(),
        selected_plan.participant_identities()
    );
    assert_eq!(
        receipt.selected_conflict_plan_identities(),
        selected_plan.participant_identities()
    );
    assert_eq!(
        receipt.independence_proof_identities(),
        &[proof.proof_digest().to_string()]
    );
    assert_eq!(
        receipt.selected_family_rows(),
        selected_plan.selected_family_rows()
    );
    assert_eq!(
        receipt.supporting_conflict_family_rows(),
        selected_plan.supporting_conflict_family_rows()
    );
    assert_eq!(receipt.advisory(), selected_plan.advisory());
    assert_eq!(receipt.denial(), selected_plan.denial());
    assert_eq!(
        receipt.evidence_stage(),
        WorkloadEvidenceStage::BatchAdmissionExecution
    );
    assert_eq!(receipt.counters().participant_identity_count(), 2);
    assert_eq!(receipt.counters().selected_conflict_plan_count(), 2);
    assert_eq!(receipt.counters().parallel_independence_proof_count(), 1);
    assert_eq!(receipt.counters().serial_independence_proof_count(), 0);
    assert_eq!(receipt.counters().parallel_edge_breadth(), 1);
    assert_eq!(receipt.counters().serial_edge_breadth(), 0);
}

#[test]
fn serial_execution_receipt_keeps_semantic_breadth_and_denial_counters_honest() {
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
    let selected_plan = lower_selected_batch_admission_plan(
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

    let receipt = execute_selected_batch_admission_plan(&selected_plan);

    assert_eq!(receipt.posture(), BatchAdmissionFamilyPosture::SerialAdmit);
    assert_eq!(
        receipt.independence_proof_identities(),
        &[proof.proof_digest().to_string()]
    );
    assert_eq!(
        receipt.selected_family_rows(),
        selected_plan.selected_family_rows()
    );
    assert_eq!(
        receipt.supporting_conflict_family_rows(),
        selected_plan.supporting_conflict_family_rows()
    );
    assert_eq!(receipt.advisory(), selected_plan.advisory());
    assert_eq!(receipt.denial(), selected_plan.denial());
    assert_eq!(receipt.counters().parallel_independence_proof_count(), 0);
    assert_eq!(receipt.counters().serial_independence_proof_count(), 1);
    assert_eq!(
        receipt
            .counters()
            .topology_supporting_conflict_family_row_count(),
        2
    );
    assert_eq!(
        receipt
            .counters()
            .spatial_supporting_conflict_family_row_count(),
        0
    );
    assert_eq!(receipt.counters().selected_plan_denial_count(), 0);
    assert_eq!(receipt.counters().declared_denied_proof_count(), 0);
    assert_eq!(receipt.counters().advisory_query_boundary_count(), 0);
}

#[test]
fn selected_plan_denied_execution_receipt_preserves_exact_denial_and_rows() {
    let denied_closeout = owner_backed_topology_closeout_with_replay_prior_proof_posture(
        TopologyConflictPriorProofPosture::NoPriorProofRequired,
    );
    let admitted_closeout =
        current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let denied_boundary = packet_backed_boundary("phase9.batch.receipt.topology.denied");
    let admitted_boundary = packet_backed_boundary("phase9.batch.receipt.topology.admitted");
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
    let selected_plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Topology(&denied_plan),
            BatchAdmissionCandidate::Topology(&admitted_plan),
        ]))
        .expect("group admits"),
    );

    let receipt = execute_selected_batch_admission_plan(&selected_plan);

    assert_eq!(receipt.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        receipt.selected_family_rows(),
        selected_plan.selected_family_rows()
    );
    assert_eq!(
        receipt.supporting_conflict_family_rows(),
        selected_plan.supporting_conflict_family_rows()
    );
    assert_eq!(receipt.advisory(), selected_plan.advisory());
    assert_eq!(receipt.denial(), selected_plan.denial());
    assert_eq!(
        receipt.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::DeniedGroupedOverlap
    );
    assert_eq!(
        receipt.denial().expect("denial row").kind(),
        BatchAdmissionPlanDenialKind::SelectedPlanDenied
    );
    assert_eq!(receipt.counters().selected_plan_denial_count(), 1);
    assert_eq!(receipt.counters().declared_denied_proof_count(), 0);
    assert_eq!(receipt.counters().advisory_query_boundary_count(), 0);
}

#[test]
fn declared_denied_execution_receipt_preserves_exact_denial_and_counters() {
    let denied_closeout =
        owner_backed_topology_closeout_with_aspect_routing_posture(ConflictRoutingPosture::Denied);
    let admitted_closeout = owner_backed_topology_closeout_with_aspect_routing_posture(
        ConflictRoutingPosture::ProvenIndependent,
    );
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&denied_closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&admitted_closeout, &right);
    let denied_proof = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &right_plan),
    );
    let selected_plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(
                BatchAdmissionPairwiseIndependenceProof::Topology(&denied_proof),
            ),
        )
        .expect("group admits"),
    );

    let receipt = execute_selected_batch_admission_plan(&selected_plan);

    assert_eq!(receipt.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        receipt.independence_proof_identities(),
        &[denied_proof.proof_digest().to_string()]
    );
    assert_eq!(
        receipt.selected_family_rows(),
        selected_plan.selected_family_rows()
    );
    assert_eq!(
        receipt.supporting_conflict_family_rows(),
        selected_plan.supporting_conflict_family_rows()
    );
    assert_eq!(receipt.advisory(), selected_plan.advisory());
    assert_eq!(receipt.denial(), selected_plan.denial());
    assert_eq!(
        receipt.denial().expect("denial row").kind(),
        BatchAdmissionPlanDenialKind::DeclaredDenied
    );
    assert_eq!(receipt.counters().selected_plan_denial_count(), 0);
    assert_eq!(receipt.counters().declared_denied_proof_count(), 1);
    assert_eq!(receipt.counters().advisory_query_boundary_count(), 0);
}

#[test]
fn advisory_serial_execution_receipt_preserves_advisory_rows_and_counters() {
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(21, 12, 13);
    let left_boundary = packet_backed_boundary("phase9.batch.receipt.replay.left");
    let right_boundary = alternate_packet_backed_boundary("phase9.batch.receipt.replay.right");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure).with_replay_boundary(&left_boundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure).with_replay_boundary(&right_boundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let selected_plan = lower_selected_batch_admission_plan(
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

    let receipt = execute_selected_batch_admission_plan(&selected_plan);

    assert_eq!(
        receipt.posture(),
        BatchAdmissionFamilyPosture::AdvisorySerialAdmit
    );
    assert_eq!(
        receipt.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::AdvisoryQueryBoundaryParallel
    );
    assert_eq!(
        receipt.selected_family_rows(),
        selected_plan.selected_family_rows()
    );
    assert_eq!(
        receipt.supporting_conflict_family_rows(),
        selected_plan.supporting_conflict_family_rows()
    );
    assert_eq!(receipt.advisory(), selected_plan.advisory());
    assert_eq!(receipt.denial(), selected_plan.denial());
    assert_eq!(
        receipt
            .advisory()
            .expect("advisory witness")
            .witness_shape(),
        BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination
    );
    assert_eq!(receipt.counters().selected_plan_denial_count(), 0);
    assert_eq!(receipt.counters().declared_denied_proof_count(), 0);
    assert_eq!(receipt.counters().advisory_query_boundary_count(), 1);
}
