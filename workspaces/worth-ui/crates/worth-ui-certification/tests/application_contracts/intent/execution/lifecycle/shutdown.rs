use worth_ui::facade::intent::{
    UiIntentAdmissionShutdownReport, UiIntentExecutionCancellationReason,
    UiIntentExecutionShutdownReport, UiIntentExecutionTransitionPosture,
};

use super::provider::{AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider};
use super::{advance, dispatch, execution_census};
use crate::intent::admission::phase3::world::AdmissionWorld;

#[test]
fn shutdown_disposes_every_execution_phase_once_without_claiming_false_rollback() {
    let (provider, observation) = ScriptedProvider::new([
        ExecutionScript::running([AttemptStep::PendingBeforeEffect])
            .with_cancellations([AttemptStep::CancelledBeforeEffect]),
        ExecutionScript::running([AttemptStep::PendingEffectMayHaveBegun])
            .with_cancellations([AttemptStep::PendingEffectMayHaveBegun]),
        ExecutionScript::running([AttemptStep::PartialWithoutOutcome])
            .with_recovery([RecoveryStep::Pending]),
        ExecutionScript::running([AttemptStep::Completed]),
    ]);
    let mut world = AdmissionWorld::launch_with_provider(5, provider);
    for target in 0..4 {
        dispatch(&mut world, target, 40);
    }
    assert_eq!(advance(&mut world, 1).transitions().len(), 4);
    let mut transitions = advance(&mut world, 2).into_transitions().into_vec();
    assert!(matches!(
        transitions[0].posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    ));
    assert!(matches!(
        transitions[1].posture(),
        UiIntentExecutionTransitionPosture::PendingEffectMayHaveBegun
    ));
    assert!(matches!(
        transitions[2].posture(),
        UiIntentExecutionTransitionPosture::Partial { .. }
    ));
    assert!(matches!(
        transitions[3].posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    let _completed = transitions.pop().unwrap();
    let recovery = transitions
        .pop()
        .unwrap()
        .into_recovery()
        .expect("the partial phase exposes affine recovery before shutdown");
    dispatch(&mut world, 4, 40);
    assert_eq!(execution_census(&world), [4, 4, 1, 1]);

    let shutdown = world.session.shutdown();
    assert_shutdown_report(shutdown.intent_execution(), shutdown.intent_admission());

    let cancellations = observation.cancellations();
    assert_eq!(cancellations.len(), 2);
    assert!(cancellations.iter().all(|context| {
        context.tick() == 2 && context.reason() == UiIntentExecutionCancellationReason::Shutdown
    }));
    assert_eq!(observation.counts(), [4, 4, 2, 0, 4, 1, 1]);
    drop(recovery);
}

fn assert_shutdown_report(
    execution: UiIntentExecutionShutdownReport,
    admission: UiIntentAdmissionShutdownReport,
) {
    assert_eq!(execution.execution_entries_disposed(), 5);
    assert_eq!(execution.reservation_backed_entries_disposed(), 4);
    assert_eq!(execution.provider_cancellation_calls(), 2);
    assert_eq!(execution.before_effect_disposals(), 2);
    assert_eq!(execution.completed_outcomes_disposed(), 1);
    assert_eq!(execution.partial_effect_disposals(), 1);
    assert_eq!(execution.indeterminate_effect_disposals(), 1);
    assert_eq!(execution.recovery_lanes_disposed(), 1);
    assert_eq!(execution.consequence_pending_outcomes_disposed(), 1);
    assert_eq!(execution.active_after(), 0);
    assert_eq!(admission.settled_attempts(), 4);
    assert_eq!(admission.active_after(), 0);
}
