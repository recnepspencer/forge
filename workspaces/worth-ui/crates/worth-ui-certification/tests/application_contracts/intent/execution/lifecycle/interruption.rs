use worth_ui::facade::intent::{
    UiIntentExecutionCancellationReason, UiIntentExecutionTransitionPosture,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture,
};

use super::provider::{AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider};
use super::{advance, dispatch, execution_census, only_transition};
use crate::intent::admission::phase3::world::AdmissionWorld;

#[test]
fn mounted_removal_cancels_running_pre_effect_work_at_the_next_safe_point() {
    let (provider, observation) =
        ScriptedProvider::new(
            [ExecutionScript::running([AttemptStep::PendingBeforeEffect])
                .with_cancellations([AttemptStep::CancelledBeforeEffect])],
        );
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    let _ = only_transition(advance(&mut world, 1));
    let _ = only_transition(advance(&mut world, 2));

    world.unmount(0).expect("the running target unmounts");
    assert_eq!(
        execution_census(&world),
        [1, 1, 0, 0],
        "unmount selects cancellation without silently dropping the attempt"
    );
    assert!(matches!(
        only_transition(advance(&mut world, 3)).posture(),
        UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. }
    ));
    let cancellations = observation.cancellations();
    assert_eq!(cancellations.len(), 1);
    assert_eq!(cancellations[0].tick(), 3);
    assert_eq!(
        cancellations[0].reason(),
        UiIntentExecutionCancellationReason::MountedInstanceRemoved
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 0]);
    let _ = world.session.shutdown();
}

#[test]
fn mounted_removal_after_effect_uncertainty_retains_affine_recovery() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingEffectMayHaveBegun,
        AttemptStep::Completed,
    ])
    .with_cancellations([AttemptStep::CancelledBeforeEffect])
    .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    let _ = only_transition(advance(&mut world, 1));
    let pending = only_transition(advance(&mut world, 2));
    world.unmount(0).expect("the uncertain target unmounts");

    let uncertain = only_transition(advance(&mut world, 3));
    assert!(matches!(
        uncertain.posture(),
        UiIntentExecutionTransitionPosture::Indeterminate { .. }
    ));
    assert_eq!(uncertain.attempt(), pending.attempt());
    let recovery = uncertain
        .into_recovery()
        .expect("effect uncertainty survives target removal");
    let UiIntentRecoveryProgressOutcome::Progressed(receipt) = world
        .session
        .retry_intent_recovery(recovery, super::super::execution_reading(4))
    else {
        panic!("unmounted uncertain work must remain recoverable")
    };
    assert!(matches!(
        receipt.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    assert_eq!(
        observation.cancellations()[0].reason(),
        UiIntentExecutionCancellationReason::MountedInstanceRemoved
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}
