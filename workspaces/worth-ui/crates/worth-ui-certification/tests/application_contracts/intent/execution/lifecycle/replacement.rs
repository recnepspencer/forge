use worth_ui::facade::intent::{
    UiIntentExecutionCancellationReason, UiIntentExecutionTransitionPosture,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture,
};

use super::provider::{AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider};
use super::{advance, dispatch, execution_census, only_transition};
use crate::intent::admission::phase3::world::AdmissionWorld;

#[test]
fn application_replacement_cancels_running_work_without_dropping_the_predecessor() {
    let (provider, observation) =
        ScriptedProvider::new(
            [ExecutionScript::running([AttemptStep::PendingBeforeEffect])
                .with_cancellations([AttemptStep::CancelledBeforeEffect])],
        );
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 40);
    let _ = only_transition(advance(&mut world, 1));
    let _ = only_transition(advance(&mut world, 2));

    world.rebind_application();
    assert_eq!(observation.counts()[6], 0);
    assert_eq!(execution_census(&world), [1, 1, 0, 0]);
    assert!(matches!(
        only_transition(advance(&mut world, 31)).posture(),
        UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. }
    ));
    assert_eq!(
        observation.cancellations()[0].reason(),
        UiIntentExecutionCancellationReason::ApplicationRebound
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 0]);
    let _ = world.session.shutdown();
}

#[test]
fn recovery_retains_the_predecessor_provider_across_application_replacement() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PartialWithoutOutcome,
    ])
    .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 40);
    let _ = only_transition(advance(&mut world, 1));
    let partial = only_transition(advance(&mut world, 2));
    let recovery = partial
        .into_recovery()
        .expect("partial effect returns affine recovery");

    world.rebind_application();
    assert_eq!(
        observation.counts()[6],
        0,
        "recovery retains predecessor provider"
    );
    assert_eq!(execution_census(&world), [1, 1, 1, 0]);
    let UiIntentRecoveryProgressOutcome::Progressed(receipt) = world
        .session
        .retry_intent_recovery(recovery, super::super::execution_reading(31))
    else {
        panic!("predecessor recovery remains callable after replacement")
    };
    assert!(matches!(
        receipt.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}
