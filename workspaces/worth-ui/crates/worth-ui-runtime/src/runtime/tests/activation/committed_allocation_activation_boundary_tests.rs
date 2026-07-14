use super::activation_staging_test_support::activation_staging_inputs;
use crate::runtime::{UiAllocationFrameDispatcherState, UiAllocationFramePauseReason};

fn ordinary_activation_inputs() -> (
    crate::runtime::WorthUiRuntime,
    crate::runtime::WorthUiPendingActivation,
    crate::graph::UiAdmittedAllocationCatalogBasisSet,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "atomic-plan-swap.ordinary",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits the complete ordinary catalog");
    (runtime, pending, admitted)
}

#[test]
fn ordinary_activation_replaces_all_live_families_and_seals_terminal_receipt() {
    let (mut runtime, pending, admitted) = ordinary_activation_inputs();
    let previous = runtime.inspect_active();
    let expected_artifact = pending.staged_replacement().candidate_artifact_digest();
    let boundary = runtime.safe_frame_boundary();

    let receipt = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect("ordinary activation commits once");
    let active = runtime.inspect_active();

    assert_eq!(active.artifact_digest(), expected_artifact);
    assert_eq!(active.snapshot_digest(), previous.snapshot_digest());
    assert_eq!(
        receipt.previous_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        receipt.previous_active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(receipt.next_active_artifact_digest(), expected_artifact);
    assert_eq!(
        receipt.next_active_plan_digest(),
        active.active_plan_digest()
    );
    assert_eq!(receipt.committed_allocation().receipts().len(), 2);
    assert_eq!(receipt.counters().live_mutation_count(), 1);
    assert_eq!(receipt.counters().active_successor_builds(), 1);
    assert_eq!(
        receipt
            .allocation_frame_replacement()
            .queue_disposition()
            .reason(),
        UiAllocationFramePauseReason::Replacement
    );
    assert_eq!(
        runtime.allocation_frame_dispatcher_state(),
        UiAllocationFrameDispatcherState::Open(active.frame_epoch())
    );
}

#[test]
fn ordinary_activation_records_previous_active_as_last_valid_in_same_commit() {
    let (mut runtime, pending, admitted) = ordinary_activation_inputs();
    let previous = runtime.inspect_active();
    let boundary = runtime.safe_frame_boundary();

    let receipt = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect("ordinary activation commits");
    let last_valid = runtime.last_valid();

    assert_eq!(
        receipt.prior_valid_plan().artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(last_valid.artifact_digest(), previous.artifact_digest());
    assert_eq!(
        last_valid.active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(last_valid.recorded_frame_epoch(), previous.frame_epoch());
}

#[test]
fn unsafe_boundary_denial_leaves_every_live_family_unchanged() {
    let (mut runtime, pending, admitted) = ordinary_activation_inputs();
    let active_before = runtime.inspect_active();
    let last_valid_before = runtime.last_valid();
    let dispatcher_before = runtime.allocation_frame_dispatcher_state();
    let boundary = runtime.traversal_frame_boundary_for_test();

    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("unsafe boundary denies before resource acquisition");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries the canonical attempt")
    };
    assert!(matches!(
        denial.reason(),
        crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(_)
    ));
    assert!(denial.evidence().live_state_unchanged());
    assert_eq!(runtime.inspect_active(), active_before);
    assert_eq!(runtime.last_valid(), last_valid_before);
    assert_eq!(
        runtime.allocation_frame_dispatcher_state(),
        dispatcher_before
    );
}

#[test]
fn success_and_denial_inspection_share_the_structural_attempt_lineage() {
    let (mut successful_runtime, successful_pending, successful_catalog) =
        ordinary_activation_inputs();
    let successful_boundary = successful_runtime.safe_frame_boundary();
    let receipt = successful_runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            successful_pending,
            successful_catalog,
            successful_boundary,
            None,
        )
        .expect("canonical attempt commits at a safe boundary");

    let (mut denied_runtime, denied_pending, denied_catalog) = ordinary_activation_inputs();
    let denied_boundary = denied_runtime.traversal_frame_boundary_for_test();
    let denial = denied_runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            denied_pending,
            denied_catalog,
            denied_boundary,
            None,
        )
        .expect_err("the same structural attempt denies at an unsafe boundary");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial retains canonical attempt evidence")
    };

    let committed = receipt.inspection();
    let denied = denial.inspection();
    assert_eq!(
        committed.outcome(),
        crate::runtime::UiCommittedAllocationActivationInspectionOutcome::Committed
    );
    assert_eq!(
        denied.outcome(),
        crate::runtime::UiCommittedAllocationActivationInspectionOutcome::Denied
    );
    assert_eq!(
        committed.attempt_identity_digest(),
        denied.attempt_identity_digest()
    );
    assert_eq!(
        committed.committed_row_count(),
        denied.committed_row_count()
    );
    assert!(denied.live_state_unchanged());
}
