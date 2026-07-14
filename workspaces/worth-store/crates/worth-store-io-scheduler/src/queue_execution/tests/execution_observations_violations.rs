use super::super::execute_admitted_queue_plan;
use super::super::test_support::{admitted_plan, completion_for_plan, speculative_scope};
use super::super::QueueExecutionObservation;
use crate::{
    execute_ready_queue_plan, QueueDurabilityClass, QueueExecutionOutcome,
    QueueExecutionProgression,
};

#[test]
fn mechanical_execution_cannot_reclassify_or_change_durability() {
    let plan = admitted_plan();
    let outcome = execute_admitted_queue_plan(
        plan,
        QueueExecutionObservation::empty()
            .with_attempted_durability_class(QueueDurabilityClass::PlatformDurable),
    );

    let QueueExecutionOutcome::Violation(violation) = outcome else {
        panic!("durability strengthening must be a violation");
    };
    assert_eq!(violation.counters().violation_events(), 1);
    assert_eq!(
        violation.plan().progression(),
        QueueExecutionProgression::Executed
    );
}

#[test]
fn certification_completion_executes_inside_admitted_envelope() {
    let plan = admitted_plan();
    let scope = speculative_scope(&plan);
    let completion_builder =
        completion_for_plan(&plan, 1, Some(scope), 0, None).observe_mechanical_adaptation(1, 1, 1);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Executed(executed) = outcome else {
        panic!("backend completion should execute inside admitted envelope");
    };
    assert_eq!(executed.counters().grouped_writes(), 0);
    assert_eq!(executed.counters().read_ahead_units(), 1);
    assert_eq!(executed.counters().mechanical_retries(), 1);
    assert_eq!(executed.counters().partial_read_events(), 1);
    assert_eq!(executed.counters().short_write_events(), 1);
}
