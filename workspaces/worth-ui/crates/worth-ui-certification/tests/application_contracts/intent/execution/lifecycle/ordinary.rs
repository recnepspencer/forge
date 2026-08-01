use worth_ui::facade::intent::{UiIntentExecutionTransitionPosture, UiIntentProductOutcome};

use super::provider::{AttemptStep, ExecutionScript, ScriptedProvider};
use super::{advance, dispatch, execution_census, only_transition};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::operability::EmptyOutcome;

#[test]
fn framework_starts_polls_and_accepts_only_the_first_terminal_settlement() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingBeforeEffect,
        AttemptStep::Completed,
        AttemptStep::FailedBeforeEffect,
    ])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);

    let started = only_transition(advance(&mut world, 1));
    assert_eq!(
        started.posture(),
        UiIntentExecutionTransitionPosture::Started
    );
    let requests = observation.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].attempt, started.attempt());
    assert_eq!(requests[0].idempotency, started.idempotency());
    assert_eq!(requests[0].deadline.tick(), 20);

    assert_eq!(
        only_transition(advance(&mut world, 2)).posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    );
    assert_eq!(
        only_transition(advance(&mut world, 3)).posture(),
        UiIntentExecutionTransitionPosture::Completed {
            outcome: EmptyOutcome::SCHEMA,
        }
    );
    assert!(advance(&mut world, 4).transitions().is_empty());
    assert_eq!(
        observation.counts()[1],
        2,
        "terminal attempts are never polled again"
    );
    assert_eq!(
        observation.counts()[4],
        1,
        "the attempt is disposed exactly once"
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}

#[test]
fn deadline_and_provider_rejection_stop_before_external_effect() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([])]);
    let mut deadline_world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut deadline_world, 0, 5);
    assert!(matches!(
        only_transition(advance(&mut deadline_world, 6)).posture(),
        UiIntentExecutionTransitionPosture::TimedOutBeforeEffect { .. }
    ));
    assert_eq!(
        observation.counts()[0],
        0,
        "expired work never reaches begin"
    );
    assert_eq!(execution_census(&deadline_world), [0, 0, 0, 0]);
    let _ = deadline_world.session.shutdown();

    let (provider, observation) = ScriptedProvider::new([ExecutionScript::rejected()]);
    let mut rejected_world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut rejected_world, 0, 5);
    assert!(matches!(
        only_transition(advance(&mut rejected_world, 1)).posture(),
        UiIntentExecutionTransitionPosture::RejectedBeforeEffect { .. }
    ));
    assert_eq!(observation.counts()[0], 1);
    assert_eq!(observation.counts()[4..6], [0, 0]);
    assert_eq!(execution_census(&rejected_world), [0, 0, 0, 0]);
    let _ = rejected_world.session.shutdown();
}

#[test]
fn deadline_cancellation_before_effect_settles_without_recovery() {
    let (provider, observation) =
        ScriptedProvider::new(
            [ExecutionScript::running([AttemptStep::PendingBeforeEffect])
                .with_cancellations([AttemptStep::TimedOutBeforeEffect])],
        );
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 3);
    let _ = only_transition(advance(&mut world, 1));
    let _ = only_transition(advance(&mut world, 2));
    assert!(matches!(
        only_transition(advance(&mut world, 4)).posture(),
        UiIntentExecutionTransitionPosture::TimedOutBeforeEffect { .. }
    ));
    assert_eq!(observation.counts()[2], 1);
    assert_eq!(observation.counts()[5], 0);
    assert_eq!(execution_census(&world), [0, 0, 0, 0]);
    let _ = world.session.shutdown();
}
