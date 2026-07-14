use worth_store_physical_backend::{
    BackendQueueExecutionBackpressure, BackendQueueSpeculativeScope,
};

use super::super::test_support::{
    admitted_plan, admitted_write_back_plan, completion_for_plan, speculative_scope,
};
use crate::{execute_ready_queue_plan, QueueBackpressureCause, QueueExecutionOutcome};

#[test]
fn backpressure_is_typed_and_counter_visible_after_admission() {
    let plan = admitted_plan();
    let completion_builder = completion_for_plan(&plan, 0, None, 0, None)
        .observe_queue_depth(4)
        .observe_foreground_wait_events(1)
        .observe_backpressure(BackendQueueExecutionBackpressure::QueueDepthSaturated);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("expected typed backpressure outcome");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::QueueDepthSaturated
    );
    assert_eq!(backpressured.counters().backpressure_events(), 1);
    assert_eq!(backpressured.counters().foreground_wait_events(), 1);
    assert_eq!(
        backpressured.counters().backpressure_cause(),
        Some(QueueBackpressureCause::QueueDepthSaturated)
    );
}

#[test]
fn over_budget_read_ahead_is_a_typed_denial() {
    let plan = admitted_plan();
    let scope = speculative_scope(&plan);
    let completion_builder = completion_for_plan(&plan, 2, Some(scope), 0, None);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Denied(denied) = outcome else {
        panic!("over-budget read-ahead must be denied");
    };
    assert_eq!(denied.cause(), QueueBackpressureCause::ReadAheadDenied);
    assert_eq!(denied.counters().denied_units(), 1);
    assert_eq!(denied.counters().read_ahead_units(), 2);
}

#[test]
fn queue_depth_above_admitted_slots_is_backpressure() {
    let plan = admitted_plan();
    let completion_builder = completion_for_plan(&plan, 0, None, 0, None).observe_queue_depth(2);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("queue-depth saturation must be backpressure");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::QueueDepthSaturated
    );
    assert_eq!(backpressured.counters().backpressure_events(), 1);
}

#[test]
fn write_back_with_cross_key_scope_is_backpressured_and_counter_visible() {
    let plan = admitted_write_back_plan();
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
    );
    let completion_builder = completion_for_plan(&plan, 0, None, 1, Some(wrong_scope));
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("cross-key write-back must be typed backpressure");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::WriteBackWindowSaturated
    );
    assert_eq!(backpressured.counters().write_back_units(), 1);
}

#[test]
fn backend_completion_with_cross_scope_read_ahead_is_denied() {
    let plan = admitted_plan();
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
    );
    let completion_builder = completion_for_plan(&plan, 1, Some(wrong_scope), 0, None);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Denied(denied) = outcome else {
        panic!("cross-scope speculative read-ahead must deny");
    };
    assert_eq!(denied.cause(), QueueBackpressureCause::ReadAheadDenied);
}
