use worth_store_physical_backend::{BackendQueueExecutionBackpressure, BackendTargetProfile};

use super::super::test_support::{
    admitted_plan, admitted_plan_for_backend_profile, completion_for_group, speculative_scope,
};
use crate::{
    execute_grouped_ready_queue_plans, group_ready_queue_pair, QueueExecutionOutcome,
    QueueGroupingDenial, QueueGroupingOutcome,
};

#[test]
fn grouping_denies_backend_capability_mismatch() {
    let outcome = group_ready_queue_pair(
        admitted_plan(),
        admitted_plan_for_backend_profile(BackendTargetProfile::SimulatedStrictDurable),
    );

    let QueueGroupingOutcome::Denied(denied) = outcome else {
        panic!("mixed backend profile plans must not group");
    };
    assert_eq!(
        denied.denial(),
        QueueGroupingDenial::BackendCapabilityMismatch
    );
}

#[test]
fn grouped_ready_execution_proves_and_counts_both_plans() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let expected_units = grouped
        .first()
        .admitted_budget()
        .queue_slots()
        .saturating_add(grouped.first().admitted_budget().bandwidth_tokens())
        .saturating_add(grouped.first().admitted_budget().read_ahead_window())
        .saturating_add(grouped.first().admitted_budget().worker_permits())
        .saturating_add(grouped.first().admitted_budget().cache_residency_hints())
        * 2;
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_group(&grouped, 1, Some(scope), 0, None)
        .observe_mechanical_adaptation(2, 1, 1);
    let execution = execute_grouped_ready_queue_plans(*grouped, completion_builder.complete());

    let QueueExecutionOutcome::Executed(executed) = execution else {
        panic!("grouped ready plans should execute with scheduler grouping evidence");
    };
    assert_eq!(executed.counters().grouped_writes(), 2);
    assert_eq!(executed.counters().submitted_units(), expected_units);
    assert_eq!(executed.counters().admitted_units(), expected_units);
    assert!(executed.secondary_plan().is_some());
    assert_eq!(executed.counters().read_ahead_units(), 1);
    assert_eq!(executed.counters().mechanical_retries(), 2);
    assert_eq!(executed.counters().partial_read_events(), 1);
    assert_eq!(executed.counters().short_write_events(), 1);
}

#[test]
fn grouped_backpressure_preserves_grouping_speculation_and_adaptation_counters() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_group(&grouped, 1, Some(scope), 0, None)
        .observe_mechanical_adaptation(2, 1, 1)
        .observe_backpressure(BackendQueueExecutionBackpressure::QueueDepthSaturated);
    let execution = execute_grouped_ready_queue_plans(*grouped, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = execution else {
        panic!("backend saturation should be typed backpressure");
    };
    assert_eq!(backpressured.counters().grouped_writes(), 2);
    assert_eq!(backpressured.counters().read_ahead_units(), 1);
    assert_eq!(backpressured.counters().mechanical_retries(), 2);
    assert_eq!(backpressured.counters().partial_read_events(), 1);
    assert_eq!(backpressured.counters().short_write_events(), 1);
}

#[test]
fn ready_queue_grouping_is_typed_and_replay_visible() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());

    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    assert_eq!(grouped.grouped_writes(), 2);
    assert_eq!(
        grouped.replay_identities()[0],
        grouped.first().replay_identity()
    );
    assert_eq!(
        grouped.replay_identities()[1],
        grouped.second().replay_identity()
    );
}
