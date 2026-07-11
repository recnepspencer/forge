use forge_store_physical_backend::{BackendQueueExecutionPlanBinding, BackendTargetProfile};

use super::super::test_support::{
    admitted_plan, completion_for_binding, completion_for_plan, speculative_scope,
};
use crate::{
    execute_grouped_ready_queue_plans, execute_ready_queue_plan, group_ready_queue_pair,
    QueueExecutionOutcome, QueueGroupingOutcome,
};

#[test]
fn backend_completion_binding_rejects_different_ready_plan() {
    let plan = admitted_plan();
    let completion_builder = completion_for_binding(
        BackendQueueExecutionPlanBinding::from_store_replay_binding(
            plan.backend_completion_binding()
                .backend_execution_binding()
                .primary(),
            None,
            BackendTargetProfile::SimulatedStrictDurable,
            plan.backend_evidence_class(),
            0,
        ),
        1,
        Some(speculative_scope(&plan)),
        0,
        None,
    )
    .observe_mechanical_adaptation(2, 1, 1);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Violation(violation) = outcome else {
        panic!("wrong backend completion binding must violate");
    };
    assert_eq!(violation.counters().violation_events(), 1);
    assert_eq!(violation.counters().read_ahead_units(), 1);
    assert_eq!(violation.counters().mechanical_retries(), 2);
}

#[test]
fn grouped_ready_execution_rejects_single_plan_backend_completion() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_plan(grouped.first(), 1, Some(scope), 0, None);
    let execution = execute_grouped_ready_queue_plans(grouped, completion_builder.complete());

    let QueueExecutionOutcome::Violation(violation) = execution else {
        panic!("single-plan backend completion must not claim grouped execution");
    };
    assert_eq!(violation.counters().violation_events(), 1);
}
