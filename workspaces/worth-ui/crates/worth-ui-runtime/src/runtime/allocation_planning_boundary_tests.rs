use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis, allocation_planning,
    changed_allocation_neighborhood, changed_measurement_basis, denied_allocation_neighborhood,
    denied_measurement_basis,
};
use crate::evidence::{
    evidence_authority_binding, evidence_ref, UiEvidenceAuthorityKind, UiEvidenceFamily,
    UiEvidenceMaterializedDetail,
};
use crate::runtime::{
    WorthUiAllocationPlanningDenialReason, WorthUiExecutionPlanInput, WorthUiPlanLoweringBasis,
};
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceExpansionOutcome, UiEvidenceRichness,
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
    let first_neighborhood = admitted_allocation_neighborhood("allocation.a");
    let second_basis = changed_measurement_basis("allocation.a");
    let second_neighborhood = changed_allocation_neighborhood("allocation.a");
    let first = runtime.plan_allocation(&pending, &first_basis, &first_neighborhood);
    let second = runtime.plan_allocation(&pending, &second_basis, &second_neighborhood);

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
    let neighborhood = denied_allocation_neighborhood("allocation.denied");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
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
        .allocate_runtime_handles(&planning)
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

    let denied = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
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

#[test]
fn planning_receipt_expansion_round_trips_by_planning_identity_not_graph_node_alias() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let first_basis = admitted_measurement_basis("allocation.expand");
    let first_neighborhood = admitted_allocation_neighborhood("allocation.expand");
    let second_basis = changed_measurement_basis("allocation.expand");
    let second_neighborhood = changed_allocation_neighborhood("allocation.expand");
    let first = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input.clone(),
        &first_basis,
        &first_neighborhood,
    );
    let second = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &second_basis,
        &second_neighborhood,
    );
    let first_ref = {
        let receipt = harness.inspect_allocation_planning(&first);
        receipt.evidence_slice().refs()[0]
    };
    let second_ref = {
        let receipt = harness.inspect_allocation_planning(&second);
        receipt.evidence_slice().refs()[0]
    };

    assert_eq!(
        first_ref.handle().handle_digest(),
        first.planning_identity_digest()
    );
    assert_eq!(
        second_ref.handle().handle_digest(),
        second.planning_identity_digest()
    );
    assert_ne!(
        first
            .allocation_constraint_set()
            .expect("first planning constraint set")
            .identity()
            .identity_digest(),
        second
            .allocation_constraint_set()
            .expect("second planning constraint set")
            .identity()
            .identity_digest()
    );
    assert_ne!(first_ref.handle(), second_ref.handle());

    let first_expansion =
        harness.expand_evidence_ref(first_ref, UiEvidenceRichness::materialized_detail());
    let second_expansion =
        harness.expand_evidence_ref(second_ref, UiEvidenceRichness::materialized_detail());

    assert_eq!(
        first_expansion.outcome(),
        UiEvidenceExpansionOutcome::Available
    );
    assert_eq!(
        second_expansion.outcome(),
        UiEvidenceExpansionOutcome::Available
    );
    assert!(first_expansion.followup_query().is_none());
    assert!(second_expansion.followup_query().is_none());
    match first_expansion.materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => {
            assert_eq!(
                detail.planning_identity_digest(),
                first.planning_identity_digest()
            );
        }
        other => panic!("expected first planning detail, got {other:?}"),
    }
    match second_expansion.materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => {
            assert_eq!(
                detail.planning_identity_digest(),
                second.planning_identity_digest()
            );
        }
        other => panic!("expected second planning detail, got {other:?}"),
    }
}

#[test]
fn discarded_planning_evidence_slice_tombstones_runtime_expansion() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let measurement_basis = admitted_measurement_basis("allocation.discard");
    let neighborhood = admitted_allocation_neighborhood("allocation.discard");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
    let receipt = harness.inspect_allocation_planning(&planning);
    let evidence_ref = receipt.evidence_slice().refs()[0];
    let slice_ref = receipt.evidence_slice().slice_ref();

    assert!(harness.discard_evidence_slice(slice_ref));

    let expansion =
        harness.expand_evidence_ref(evidence_ref, UiEvidenceRichness::materialized_detail());

    assert_eq!(
        expansion.outcome(),
        UiEvidenceExpansionOutcome::Discarded {
            retention: worth_ui_inspection::UiEvidenceRetentionPosture::DiscardedWithTombstone,
        }
    );
    assert!(expansion.materialized_detail().is_none());
}

#[test]
fn re_registered_equivalent_planning_receipt_clears_discard_tombstone() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let measurement_basis = admitted_measurement_basis("allocation.rediscard");
    let neighborhood = admitted_allocation_neighborhood("allocation.rediscard");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
    let first_receipt = harness.inspect_allocation_planning(&planning);
    let evidence_ref = first_receipt.evidence_slice().refs()[0];

    assert!(harness.discard_evidence_slice(first_receipt.evidence_slice().slice_ref()));

    let second_receipt = harness.inspect_allocation_planning(&planning);
    let second_ref = second_receipt.evidence_slice().refs()[0];
    let expansion =
        harness.expand_evidence_ref(second_ref, UiEvidenceRichness::materialized_detail());

    assert_eq!(evidence_ref.handle(), second_ref.handle());
    assert_eq!(expansion.outcome(), UiEvidenceExpansionOutcome::Available);
    match expansion.materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => {
            assert_eq!(
                detail.planning_identity_digest(),
                planning.planning_identity_digest()
            );
        }
        other => panic!("expected allocation planning detail after re-registration, got {other:?}"),
    }
}

#[test]
fn stale_planning_ref_reports_wrong_generation_through_runtime_harness() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let measurement_basis = admitted_measurement_basis("allocation.stale");
    let neighborhood = admitted_allocation_neighborhood("allocation.stale");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
    let receipt = harness.inspect_allocation_planning(&planning);
    let retained_ref = receipt.evidence_slice().refs()[0];
    let stale_ref = evidence_ref(
        retained_ref.family(),
        retained_ref.identity(),
        evidence_authority_binding(
            retained_ref.authority_binding().artifact_identity().kind(),
            retained_ref
                .authority_binding()
                .artifact_identity()
                .digest(),
            UiEvidenceAuthorityGeneration::new(retained_ref.authority_generation().as_u64() + 1),
            None,
        ),
        retained_ref.materialization_posture(),
        retained_ref.retention_posture(),
        retained_ref.handle(),
    );

    let expansion =
        harness.expand_evidence_ref(stale_ref, UiEvidenceRichness::materialized_detail());

    assert_eq!(
        expansion.outcome(),
        UiEvidenceExpansionOutcome::WrongGeneration {
            requested_generation: UiEvidenceAuthorityGeneration::new(
                retained_ref.authority_generation().as_u64() + 1
            ),
            current_generation: retained_ref.authority_generation(),
        }
    );
    assert!(expansion.materialized_detail().is_none());
}
