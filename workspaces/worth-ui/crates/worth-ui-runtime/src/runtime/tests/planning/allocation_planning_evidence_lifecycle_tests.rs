use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_measurement_basis, allocation_planning, allocation_planning_with_operator,
    planning_graph_authority,
};
use crate::evidence::{evidence_authority_binding, evidence_ref, UiEvidenceMaterializedDetail};
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceExpansionOutcome, UiEvidenceRichness,
};

#[test]
fn planning_receipt_expansion_round_trips_by_planning_identity_not_graph_node_alias() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let first = allocation_planning(&runtime, &pending, "allocation.expand");
    let second =
        allocation_planning_with_operator(&runtime, &pending, "allocation.expand", "operator:grid");
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
    assert_planning_detail_identity(&first_expansion, first.planning_identity_digest());
    assert_planning_detail_identity(&second_expansion, second.planning_identity_digest());
}

#[test]
fn discarded_planning_evidence_slice_tombstones_runtime_expansion() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let measurement_basis = admitted_measurement_basis("allocation.discard");
    let (snapshot, selected) = planning_graph_authority("allocation.discard", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("discard fixture admits through graph authority"),
    );
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
    let (snapshot, selected) = planning_graph_authority("allocation.rediscard", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("rediscard fixture admits through graph authority"),
    );
    let first_receipt = harness.inspect_allocation_planning(&planning);
    let evidence_ref = first_receipt.evidence_slice().refs()[0];
    assert!(harness.discard_evidence_slice(first_receipt.evidence_slice().slice_ref()));

    let second_receipt = harness.inspect_allocation_planning(&planning);
    let second_ref = second_receipt.evidence_slice().refs()[0];
    let expansion =
        harness.expand_evidence_ref(second_ref, UiEvidenceRichness::materialized_detail());
    assert_eq!(evidence_ref.handle(), second_ref.handle());
    assert_eq!(expansion.outcome(), UiEvidenceExpansionOutcome::Available);
    assert_planning_detail_identity(&expansion, planning.planning_identity_digest());
}

#[test]
fn stale_planning_ref_reports_wrong_generation_through_runtime_harness() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let harness = runtime.inspection_ai_harness();
    let measurement_basis = admitted_measurement_basis("allocation.stale");
    let (snapshot, selected) = planning_graph_authority("allocation.stale", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("stale-ref fixture admits through graph authority"),
    );
    let receipt = harness.inspect_allocation_planning(&planning);
    let retained_ref = receipt.evidence_slice().refs()[0];
    let stale_generation =
        UiEvidenceAuthorityGeneration::new(retained_ref.authority_generation().as_u64() + 1);
    let stale_ref = evidence_ref(
        retained_ref.family(),
        retained_ref.identity(),
        evidence_authority_binding(
            retained_ref.authority_binding().artifact_identity().kind(),
            retained_ref
                .authority_binding()
                .artifact_identity()
                .digest(),
            stale_generation,
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
            requested_generation: stale_generation,
            current_generation: retained_ref.authority_generation(),
        }
    );
    assert!(expansion.materialized_detail().is_none());
}

fn assert_planning_detail_identity(
    expansion: &crate::evidence::UiEvidenceExpansion,
    expected_planning_identity_digest: u64,
) {
    match expansion.materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => {
            assert_eq!(
                detail.planning_identity_digest(),
                expected_planning_identity_digest
            );
        }
        other => panic!("expected allocation planning detail, got {other:?}"),
    }
}
