use super::frame_activation_gate_test_support::{lane_change_fixture, ready_activation_fixture};
use super::lane_meaning_parity_test_support::plan_with_command_semantics_changed;
use crate::runtime::atomic_plan_swap::WorthUiPlanSwapFailureInjection;
use crate::runtime::WorthUiPlanSwapDenialReason;

#[test]
fn atomic_swap_replaces_artifact_plan_state_and_bindings_together() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let expected_next_artifact = fixture.ready.candidate_artifact_digest();
    let expected_next_plan = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();
    let expected_query_basis = fixture.ready.query_rebind_basis_digest();
    let expected_reconciliation_basis = fixture.ready.reconciliation_basis_digest();
    let expected_node_classification_count = fixture.ready.node_classification_count();
    let expected_lane_changed_node_count = fixture.ready.lane_changed_node_count();

    let receipt = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(fixture.ready, fixture.candidate_plan, boundary)
        .expect("ready activation swaps atomically");

    let active = fixture.runtime.inspect_active();
    assert_eq!(active.artifact_digest(), expected_next_artifact);
    assert_eq!(active.active_plan_digest(), expected_next_plan);
    assert_eq!(active.snapshot_digest(), previous.snapshot_digest());
    assert_eq!(
        receipt.previous_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        receipt.previous_active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(
        receipt.previous_active_snapshot_digest(),
        previous.snapshot_digest()
    );
    assert_eq!(
        receipt.next_active_artifact_digest(),
        expected_next_artifact
    );
    assert_eq!(receipt.next_active_plan_digest(), expected_next_plan);
    assert_eq!(
        receipt.next_active_snapshot_digest(),
        previous.snapshot_digest()
    );
    assert_eq!(receipt.readiness_frame_epoch(), previous.frame_epoch());
    assert_eq!(receipt.boundary_frame_epoch(), boundary.frame_epoch());
    assert_eq!(receipt.query_rebind_basis_digest(), expected_query_basis);
    assert_eq!(
        receipt.reconciliation_basis_digest(),
        expected_reconciliation_basis
    );
    assert_eq!(
        receipt.node_classification_count(),
        expected_node_classification_count
    );
    assert_eq!(
        receipt.lane_changed_node_count(),
        expected_lane_changed_node_count
    );
    assert_eq!(receipt.counters().active_state_mutation_count(), 1);
    assert_eq!(receipt.counters().source_reparse_count(), 0);
    assert_eq!(receipt.counters().registry_rebuild_count(), 0);
    assert_eq!(receipt.counters().semantic_replanning_count(), 0);
    assert_eq!(receipt.counters().query_replanning_count(), 0);
    assert_eq!(receipt.counters().handle_allocation_count(), 0);
}

#[test]
fn swap_failure_restores_prior_valid_plan_without_source_reparse() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let attempted_artifact = fixture.ready.candidate_artifact_digest();
    let attempted_plan = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();

    let rollback = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary_with_injection_for_test(
            fixture.ready,
            fixture.candidate_plan,
            boundary,
            WorthUiPlanSwapFailureInjection::AfterArtifactMutation,
        )
        .expect_err("injected partial mutation rolls back");

    let active = fixture.runtime.inspect_active();
    assert_eq!(active, previous);
    assert_eq!(
        rollback.reason(),
        WorthUiPlanSwapDenialReason::InjectedFailureAfterArtifactMutation
    );
    assert_eq!(
        rollback.restored_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        rollback.restored_active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(
        rollback.attempted_next_artifact_digest(),
        Some(attempted_artifact)
    );
    assert_eq!(rollback.attempted_next_plan_digest(), Some(attempted_plan));
    assert_eq!(rollback.counters().active_state_mutation_count(), 1);
    assert_eq!(rollback.counters().rollback_restore_count(), 1);
    assert_eq!(rollback.counters().source_reparse_count(), 0);
    assert_eq!(rollback.counters().registry_rebuild_count(), 0);
    assert_eq!(rollback.counters().semantic_replanning_count(), 0);
    assert_eq!(rollback.counters().query_replanning_count(), 0);
    assert_eq!(rollback.counters().handle_allocation_count(), 0);
}

#[test]
fn swap_failure_before_commit_preserves_prior_valid_without_restore_work() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let attempted_artifact = fixture.ready.candidate_artifact_digest();
    let attempted_plan = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();

    let rollback = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary_with_injection_for_test(
            fixture.ready,
            fixture.candidate_plan,
            boundary,
            WorthUiPlanSwapFailureInjection::BeforeCommit,
        )
        .expect_err("pre-commit failure preserves prior active state");

    assert_eq!(fixture.runtime.inspect_active(), previous);
    assert_eq!(
        rollback.reason(),
        WorthUiPlanSwapDenialReason::InjectedFailureBeforeCommit
    );
    assert_eq!(
        rollback.attempted_next_artifact_digest(),
        Some(attempted_artifact)
    );
    assert_eq!(rollback.attempted_next_plan_digest(), Some(attempted_plan));
    assert_eq!(rollback.counters().active_state_mutation_count(), 0);
    assert_eq!(rollback.counters().rollback_restore_count(), 0);
    assert_eq!(rollback.counters().source_reparse_count(), 0);
    assert_eq!(
        rollback.restored_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        rollback.restored_active_plan_digest(),
        previous.active_plan_digest()
    );
}

#[test]
fn plan_swap_receipt_binds_previous_and_next_active_digests() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let expected_next_plan = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();

    let receipt = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(fixture.ready, fixture.candidate_plan, boundary)
        .expect("ready activation swaps");

    let prior = receipt.prior_valid_plan();
    assert_eq!(prior.artifact_digest(), previous.artifact_digest());
    assert_eq!(prior.active_plan_digest(), previous.active_plan_digest());
    assert_eq!(prior.snapshot_digest(), previous.snapshot_digest());
    assert_eq!(
        receipt.activation_gate_receipt().active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(receipt.next_active_plan_digest(), expected_next_plan);
}

#[test]
fn successful_swap_preserves_previous_active_as_host_last_valid_basis() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();

    fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(fixture.ready, fixture.candidate_plan, boundary)
        .expect("ready activation swaps");

    let active = fixture.runtime.inspect_active();
    let last_valid = fixture.runtime.last_valid();
    assert_ne!(last_valid.active_plan_digest(), active.active_plan_digest());
    assert_eq!(last_valid.artifact_digest(), previous.artifact_digest());
    assert_eq!(
        last_valid.active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(last_valid.recorded_frame_epoch(), previous.frame_epoch());
}

#[test]
fn partial_swap_injection_leaves_no_mixed_active_state() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();

    let rollback = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary_with_injection_for_test(
            fixture.ready,
            fixture.candidate_plan,
            boundary,
            WorthUiPlanSwapFailureInjection::AfterArtifactMutation,
        )
        .expect_err("partial mutation is rolled back");

    let active = fixture.runtime.inspect_active();
    assert_eq!(active.artifact_digest(), previous.artifact_digest());
    assert_eq!(active.active_plan_digest(), previous.active_plan_digest());
    assert_eq!(active.snapshot_digest(), previous.snapshot_digest());
    assert_ne!(
        active.active_plan_digest(),
        rollback
            .attempted_next_plan_digest()
            .expect("attempted next plan recorded")
    );
}

#[test]
fn candidate_plan_drift_after_readiness_denies_without_mutation() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let drifted_plan = plan_with_command_semantics_changed(&fixture.candidate_plan);
    let attempted_artifact = fixture.ready.candidate_artifact_digest();
    let attempted_plan = fixture.runtime.digest_execution_plan(&drifted_plan).raw();

    let rollback = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(fixture.ready, drifted_plan, boundary)
        .expect_err("candidate plan drift is denied before mutation");

    assert_eq!(fixture.runtime.inspect_active(), previous);
    assert_eq!(
        rollback.reason(),
        WorthUiPlanSwapDenialReason::CandidateExecutionPlanDigestMismatch
    );
    assert_eq!(rollback.counters().active_state_mutation_count(), 0);
    assert_eq!(rollback.counters().rollback_restore_count(), 0);
    assert_eq!(rollback.counters().source_reparse_count(), 0);
    assert_eq!(
        rollback.restored_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        rollback.restored_active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(
        rollback.attempted_next_artifact_digest(),
        Some(attempted_artifact)
    );
    assert_eq!(rollback.attempted_next_plan_digest(), Some(attempted_plan));
}

#[test]
fn stale_gate_denial_does_not_touch_active_state() {
    let mut fixture = ready_activation_fixture();
    let previous = fixture.runtime.inspect_active();
    let stale_boundary = fixture.runtime.safe_frame_boundary();
    fixture.runtime.advance_frame_epoch_for_test();
    let after_frame_advance = fixture.runtime.inspect_active();

    let rollback = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(
            fixture.ready,
            fixture.candidate_plan,
            stale_boundary,
        )
        .expect_err("stale boundary is denied by activation gate");

    assert_eq!(fixture.runtime.inspect_active(), after_frame_advance);
    assert_eq!(
        after_frame_advance.artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        after_frame_advance.active_plan_digest(),
        previous.active_plan_digest()
    );
    assert_eq!(
        rollback.reason(),
        WorthUiPlanSwapDenialReason::ActivationGateDenied(
            crate::runtime::WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch,
        )
    );
    assert_eq!(rollback.counters().active_state_mutation_count(), 0);
    assert_eq!(rollback.counters().rollback_restore_count(), 0);
    assert_eq!(
        rollback.restored_active_artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        rollback.restored_active_plan_digest(),
        previous.active_plan_digest()
    );
}

#[test]
fn lane_change_swap_receipt_preserves_candidate_breadth_without_commit_shortcut() {
    let mut fixture = lane_change_fixture(true);
    let previous = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let expected_next_plan = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();

    let ready = fixture
        .runtime
        .prepare_ready_activation(
            fixture.pending,
            &fixture.plan_input,
            &fixture.handle_allocation,
            &fixture.candidate_plan,
            fixture.parity_report.as_ref(),
        )
        .expect("lane-change candidate becomes ready");

    let receipt = fixture
        .runtime
        .swap_ready_activation_at_frame_boundary(ready, fixture.candidate_plan, boundary)
        .expect("lane-change candidate swaps through atomic commit");

    assert_eq!(receipt.previous_active_plan_digest(), previous.active_plan_digest());
    assert_eq!(receipt.next_active_plan_digest(), expected_next_plan);
    assert!(receipt.node_classification_count() > 0);
    assert!(receipt.lane_changed_node_count() > 0);
}
