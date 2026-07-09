use super::frame_activation_gate_test_support::{
    denied_query_ready_activation, lane_change_fixture,
    query_posture_drift_ready_activation_fixture, ready_activation_fixture,
    ready_activation_fixture_after_frame_advance,
};
use super::lane_meaning_parity_test_support::plan_with_command_semantics_changed;
use crate::runtime::{WorthUiActivationGateDenialReason, WorthUiRuntimeFrameEpoch};

#[test]
fn ready_activation_commits_only_at_safe_frame_boundary() {
    let fixture = ready_activation_fixture();
    let boundary = fixture.runtime.safe_frame_boundary();
    let expected_candidate_artifact_digest = fixture.plan_input.basis().candidate_artifact_digest();
    let expected_plan_digest = fixture
        .runtime
        .digest_execution_plan(&fixture.candidate_plan)
        .raw();
    let receipt = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, boundary)
        .expect("safe frame boundary activates ready plan");

    assert_eq!(
        receipt.candidate_artifact_digest(),
        expected_candidate_artifact_digest
    );
    assert_eq!(
        receipt.candidate_execution_plan_digest(),
        expected_plan_digest
    );
    assert_eq!(receipt.boundary_frame_epoch(), boundary.frame_epoch());
    assert_eq!(receipt.counters().active_state_mutation_count(), 0);
    assert_eq!(receipt.counters().semantic_replanning_count(), 0);
    assert_eq!(receipt.counters().query_replanning_count(), 0);
    assert_eq!(receipt.counters().handle_allocation_count(), 0);
}

#[test]
fn mid_frame_activation_attempt_denied_without_state_mutation() {
    let fixture = ready_activation_fixture();
    let before = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.traversal_frame_boundary_for_test();
    let denial = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, boundary)
        .expect_err("mid-frame activation is denied");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert_eq!(fixture.runtime.inspect_active(), before);
    assert_eq!(denial.counters().active_state_mutation_count(), 0);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn activation_gate_receipt_names_plan_state_and_reconciliation_basis() {
    let fixture = ready_activation_fixture();
    let active = fixture.runtime.inspect_active();
    let expected_reconciliation_basis_digest = fixture.ready.reconciliation_basis_digest();
    let expected_query_rebind_basis_digest = fixture.ready.query_rebind_basis_digest();
    let expected_node_classification_count = fixture.ready.node_classification_count();
    let expected_lane_changed_node_count = fixture.ready.lane_changed_node_count();
    let receipt = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, fixture.runtime.safe_frame_boundary())
        .expect("ready activation commits at boundary");

    assert_eq!(receipt.active_artifact_digest(), active.artifact_digest());
    assert_eq!(receipt.active_plan_digest(), active.active_plan_digest());
    assert_eq!(receipt.active_snapshot_digest(), active.snapshot_digest());
    assert_eq!(
        receipt.handle_allocation_basis_digest(),
        fixture.handle_allocation.receipt().basis_digest()
    );
    assert_eq!(
        receipt.reconciliation_basis_digest(),
        expected_reconciliation_basis_digest
    );
    assert_eq!(
        receipt.reconciliation_receipt_count(),
        fixture.reconciliation_receipt_count
    );
    assert_eq!(
        receipt.query_rebind_basis_digest(),
        expected_query_rebind_basis_digest
    );
    assert_eq!(
        receipt.query_rebind_entry_count(),
        fixture.query_rebind_entry_count
    );
    assert_eq!(
        receipt.node_classification_count(),
        expected_node_classification_count
    );
    assert_eq!(
        receipt.lane_changed_node_count(),
        expected_lane_changed_node_count
    );
}

#[test]
fn activation_gate_rejects_ready_plan_with_stale_frame_epoch() {
    let fixture = ready_activation_fixture_after_frame_advance();
    let stale_boundary = fixture
        .runtime
        .safe_frame_boundary_for_epoch_for_test(WorthUiRuntimeFrameEpoch::initial());
    let denial = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, stale_boundary)
        .expect_err("stale frame boundary cannot commit ready activation");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::StaleFrameEpoch
    );
    assert_eq!(
        denial.ready_frame_epoch().as_u64(),
        WorthUiRuntimeFrameEpoch::initial().next().as_u64()
    );
    assert_eq!(
        denial.boundary_frame_epoch(),
        WorthUiRuntimeFrameEpoch::initial()
    );
}

#[test]
fn activation_gate_rejects_ready_plan_with_future_frame_epoch() {
    let fixture = ready_activation_fixture();
    let before = fixture.runtime.inspect_active();
    let future_boundary = fixture
        .runtime
        .safe_frame_boundary_for_epoch_for_test(WorthUiRuntimeFrameEpoch::initial().next());
    let denial = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, future_boundary)
        .expect_err("future frame boundary cannot commit ready activation");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::FutureFrameEpochMismatch
    );
    assert_eq!(fixture.runtime.inspect_active(), before);
    assert_eq!(
        denial.ready_frame_epoch(),
        WorthUiRuntimeFrameEpoch::initial()
    );
    assert_eq!(
        denial.boundary_frame_epoch(),
        WorthUiRuntimeFrameEpoch::initial().next()
    );
}

#[test]
fn activation_gate_rejects_boundary_from_prior_runtime_epoch() {
    let mut fixture = ready_activation_fixture();
    let before = fixture.runtime.inspect_active();
    let original_epoch_boundary = fixture.runtime.safe_frame_boundary();
    fixture.runtime.advance_frame_epoch_for_test();
    let denial = fixture
        .runtime
        .activate_ready_at_frame_boundary(fixture.ready, original_epoch_boundary)
        .expect_err("boundary from prior runtime epoch cannot commit ready activation");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch
    );
    let after = fixture.runtime.inspect_active();
    assert_eq!(after.artifact_digest(), before.artifact_digest());
    assert_eq!(after.active_plan_digest(), before.active_plan_digest());
    assert_eq!(after.snapshot_digest(), before.snapshot_digest());
    assert_eq!(
        denial.ready_frame_epoch(),
        WorthUiRuntimeFrameEpoch::initial()
    );
    assert_eq!(
        denial.boundary_frame_epoch(),
        WorthUiRuntimeFrameEpoch::initial()
    );
}

#[test]
fn query_rebind_basis_digest_changes_for_query_posture_drift() {
    let preserved_fixture = ready_activation_fixture();
    let drift_fixture = query_posture_drift_ready_activation_fixture();
    let expected_drift_query_basis_digest = drift_fixture.ready.query_rebind_basis_digest();

    assert_ne!(
        preserved_fixture.ready.query_rebind_basis_digest(),
        expected_drift_query_basis_digest
    );
    assert!(drift_fixture.ready.query_rebind_entry_count() > 0);

    let receipt = drift_fixture
        .runtime
        .activate_ready_at_frame_boundary(
            drift_fixture.ready,
            drift_fixture.runtime.safe_frame_boundary(),
        )
        .expect("query posture drift ready activation commits at safe boundary");

    assert_eq!(
        receipt.query_rebind_basis_digest(),
        expected_drift_query_basis_digest
    );
}

#[test]
fn query_denial_receipt_blocks_ready_activation_without_query_replanning() {
    let denial = denied_query_ready_activation();

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::QueryRebindDenied
    );
    assert_eq!(denial.counters().query_rebind_entry_check_count(), 1);
    assert_eq!(denial.counters().query_replanning_count(), 0);
    assert_eq!(denial.counters().semantic_replanning_count(), 0);
}

#[test]
fn lane_changing_ready_activation_requires_lane_parity_report() {
    let fixture = lane_change_fixture(false);
    let denial = fixture
        .runtime
        .prepare_ready_activation(
            fixture.pending,
            &fixture.plan_input,
            &fixture.handle_allocation,
            &fixture.candidate_plan,
            None,
        )
        .expect_err("lane-changing activation requires semantic parity report");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::MissingLaneParityReport
    );
    assert_eq!(denial.counters().lane_parity_check_count(), 1);
}

#[test]
fn lane_parity_digest_must_match_candidate_execution_plan() {
    let fixture = lane_change_fixture(true);
    let mismatched_candidate = plan_with_command_semantics_changed(&fixture.candidate_plan);
    let denial = fixture
        .runtime
        .prepare_ready_activation(
            fixture.pending,
            &fixture.plan_input,
            &fixture.handle_allocation,
            &mismatched_candidate,
            fixture.parity_report.as_ref(),
        )
        .expect_err("candidate plan digest must match lane parity report");

    assert_eq!(
        denial.reason(),
        WorthUiActivationGateDenialReason::LaneParityDigestMismatch
    );
}

#[test]
fn lane_change_ready_activation_keeps_local_replanning_breadth_explicit() {
    let fixture = lane_change_fixture(true);
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
        .activate_ready_at_frame_boundary(ready, fixture.runtime.safe_frame_boundary())
        .expect("lane-change candidate activates at safe boundary");

    assert!(receipt.node_classification_count() > 0);
    assert!(receipt.lane_changed_node_count() > 0);
    assert!(receipt.lane_changed_node_count() <= receipt.node_classification_count());
}
