use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis, allocation_planning,
    changed_measurement_basis, denied_measurement_basis, planning_graph_authority,
};
use crate::evidence::{UiEvidenceAuthorityKind, UiEvidenceFamily, UiEvidenceMaterializedDetail};
use crate::runtime::{
    WorthUiAllocationPlanningDenialReason, WorthUiExecutionPlanInput, WorthUiPlanLoweringBasis,
};

#[test]
fn equivalent_measurement_basis_and_plan_input_converge_on_one_planning_identity() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let left_basis = admitted_measurement_basis("allocation.left");
    let left_neighborhood = admitted_allocation_neighborhood("allocation.left");
    let right_basis = admitted_measurement_basis("allocation.left");
    let right_neighborhood = admitted_allocation_neighborhood("allocation.left");
    let left = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input.clone(),
        &left_basis,
        &left_neighborhood,
    );
    let right = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &right_basis,
        &right_neighborhood,
    );

    assert!(left.is_admitted());
    assert!(right.is_admitted());
    assert_eq!(
        left.planning_identity_digest(),
        right.planning_identity_digest()
    );
    assert_eq!(
        left.allocation_neighborhood(),
        right.allocation_neighborhood()
    );
    assert_eq!(left.counters(), right.counters());
}

#[test]
fn changed_measurement_basis_changes_planning_identity_before_handles_exist() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let first_basis = admitted_measurement_basis("allocation.a");
    let second_basis = changed_measurement_basis("allocation.a");
    let (first_snapshot, first_selected) =
        planning_graph_authority("allocation.a", "operator:stack");
    let (second_snapshot, second_selected) =
        planning_graph_authority("allocation.a", "operator:grid");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &first_snapshot, first_basis, &first_selected)
            .expect("first planning basis admits through graph authority"),
    );
    let second = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &second_snapshot, second_basis, &second_selected)
            .expect("changed planning basis admits through graph authority"),
    );

    assert_ne!(
        first.measurement_basis().identity_digest(),
        second.measurement_basis().identity_digest()
    );
    assert_ne!(
        first.planning_identity_digest(),
        second.planning_identity_digest()
    );
}

#[test]
fn denied_measurement_basis_still_materializes_inspectable_planning_output() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = denied_measurement_basis("allocation.denied");
    let (snapshot, selected) = planning_graph_authority("allocation.denied", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("denied measurement posture still admits its graph authority"),
    );
    let inspection = runtime.inspect_allocation_planning(&planning);

    assert!(!planning.is_admitted());
    assert!(planning.denial_posture().is_some());
    assert!(inspection.denial().is_some());
    assert_eq!(
        inspection.planning_identity_digest(),
        planning.planning_identity_digest()
    );
    assert_eq!(
        inspection.measurement_basis_identity_digest(),
        planning.measurement_basis().identity_digest()
    );
    assert_eq!(
        inspection.family(),
        crate::evidence::UiAllocationPlanningEvidenceFamily::Planning
    );
    assert_eq!(inspection.denial(), planning.denial_posture());
    assert_eq!(inspection.evidence_slice().refs().len(), 1);
    assert_eq!(
        inspection.evidence_slice().refs()[0].family(),
        UiEvidenceFamily::Planning
    );
    assert_eq!(
        inspection.evidence_slice().refs()[0]
            .authority_binding()
            .artifact_identity()
            .kind(),
        UiEvidenceAuthorityKind::AllocationPlanning
    );
    match inspection.evidence_slice().materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => {
            assert_eq!(
                detail.planning_identity_digest(),
                inspection.planning_identity_digest()
            );
            assert_eq!(
                detail.measurement_basis_identity_digest(),
                inspection.measurement_basis_identity_digest()
            );
            assert_eq!(detail.denial(), inspection.denial());
        }
        other => panic!("expected allocation planning evidence detail, got {other:?}"),
    }
    assert_eq!(
        inspection.cost().counters().measurement_basis_read_count(),
        1
    );
    assert_eq!(inspection.cost().counters().lowering_read_count(), 0);
}

#[test]
fn handle_allocation_must_consume_planning_not_raw_plan_input() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "allocation.handles");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handle allocation succeeds");

    assert_eq!(
        allocation.basis().allocation_planning_identity_digest(),
        planning.planning_identity_digest()
    );
}

#[test]
fn denied_plan_lowering_still_materializes_typed_planning_denial() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let measurement_basis = admitted_measurement_basis("allocation.lowering");
    let neighborhood = admitted_allocation_neighborhood("allocation.lowering");
    let planning = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &measurement_basis,
        &neighborhood,
    );
    runtime.advance_frame_epoch_for_test();

    let (snapshot, selected) = planning_graph_authority("allocation.lowering", "operator:stack");
    let denied = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis.clone(), &selected)
            .expect("stale-frame scenario admits through graph authority"),
    );
    let inspection = runtime.inspect_allocation_planning(&denied);

    assert!(planning.is_admitted());
    assert!(!denied.is_admitted());
    assert!(inspection.denial().is_some());
    assert!(inspection
        .denial()
        .and_then(|denial| denial.plan_lowering_denial())
        .is_some());
    assert_eq!(
        inspection.cost().counters().measurement_basis_read_count(),
        1
    );
    assert_eq!(inspection.cost().counters().lowering_read_count(), 0);
}

#[test]
fn mismatched_lowering_input_is_denied_at_planning_boundary() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let mut tampered_nodes = plan_input.node_inputs().to_vec();
    tampered_nodes[0] = tampered_nodes[0]
        .clone()
        .with_identity_basis_for_test("allocation.mismatch.tampered");
    let tampered = WorthUiExecutionPlanInput::new(
        WorthUiPlanLoweringBasis::new(
            plan_input.basis().active_artifact_digest(),
            plan_input.basis().candidate_artifact_digest(),
            plan_input.basis().frame_epoch(),
            plan_input.basis().staged_node_classification_count(),
            plan_input.basis().staged_reconciliation_receipt_count(),
            plan_input.basis().staged_query_rebind_entry_count(),
        ),
        plan_input.context().clone(),
        tampered_nodes,
        plan_input.counters(),
    );
    let denied = runtime.plan_allocation_for_pending_and_lowered_input_for_test(
        &pending,
        tampered,
        &admitted_measurement_basis("allocation.mismatch"),
        &admitted_allocation_neighborhood("allocation.mismatch"),
    );
    let inspection = runtime.inspect_allocation_planning(&denied);

    assert!(!denied.is_admitted());
    let denial = inspection.denial().expect("planning denial expected");
    assert_eq!(
        denial.reason(),
        WorthUiAllocationPlanningDenialReason::LoweringAdmissionMismatch
    );
    let mismatch = denial.lowering_mismatch().expect("mismatch payload");
    assert_eq!(mismatch.expected(), mismatch.observed());
    assert_ne!(
        mismatch.expected_witness_digest(),
        mismatch.observed_witness_digest()
    );
    assert_eq!(
        inspection.cost().counters().measurement_basis_read_count(),
        1
    );
    assert_eq!(inspection.cost().counters().lowering_read_count(), 1);
}
