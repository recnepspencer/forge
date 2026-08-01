use worth_ui::facade::intent::{
    UiIntentExecutionTransitionPosture, UiIntentProductOutcome, UiIntentRecoveryHandle,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture,
};

use super::provider::{AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider};
use super::{advance, dispatch, execution_census};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::operability::EmptyOutcome;

#[test]
fn provider_terminal_and_recovery_matrix_keeps_each_branch_semantically_distinct() {
    let (provider, observation) = ScriptedProvider::new([
        ExecutionScript::running([AttemptStep::RejectedBeforeEffect]),
        ExecutionScript::running([AttemptStep::PartialWithoutOutcome])
            .with_recovery([RecoveryStep::PartialWithOutcome, RecoveryStep::Completed]),
        ExecutionScript::running([AttemptStep::Indeterminate])
            .with_recovery([RecoveryStep::Completed]),
    ]);
    let mut world = AdmissionWorld::launch_with_provider(3, provider);
    for target in 0..3 {
        dispatch(&mut world, target, 20);
    }
    assert_eq!(advance(&mut world, 1).transitions().len(), 3);

    let mut terminals = advance(&mut world, 2).into_transitions().into_vec();
    assert!(matches!(
        terminals[0].posture(),
        UiIntentExecutionTransitionPosture::RejectedBeforeEffect { .. }
    ));
    assert!(matches!(
        terminals[1].posture(),
        UiIntentExecutionTransitionPosture::Partial { outcome: None, .. }
    ));
    assert_eq!(
        terminals[2].posture(),
        UiIntentExecutionTransitionPosture::Indeterminate { detail: None }
    );
    assert_eq!(execution_census(&world), [2, 2, 2, 0]);

    let indeterminate = terminals
        .pop()
        .unwrap()
        .into_recovery()
        .expect("indeterminate settlement retains recovery authority");
    let partial = terminals
        .pop()
        .unwrap()
        .into_recovery()
        .expect("partial settlement retains recovery authority");
    complete_partial_recovery(&mut world, partial);

    let UiIntentRecoveryProgressOutcome::Progressed(completed) = world
        .session
        .retry_intent_recovery(indeterminate, super::super::execution_reading(5))
    else {
        panic!("indeterminate recovery must complete")
    };
    assert!(matches!(
        completed.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    assert_eq!(observation.counts()[3], 3);
    assert_eq!(execution_census(&world), [0, 0, 0, 2]);
    let _ = world.session.shutdown();
}

fn complete_partial_recovery(world: &mut AdmissionWorld, partial: UiIntentRecoveryHandle) {
    let UiIntentRecoveryProgressOutcome::Progressed(partial_receipt) = world
        .session
        .retry_intent_recovery(partial, super::super::execution_reading(3))
    else {
        panic!("partial recovery must progress")
    };
    assert!(matches!(
        partial_receipt.posture(),
        UiIntentRecoveryProgressPosture::Partial {
            outcome: Some(EmptyOutcome::SCHEMA),
            ..
        }
    ));
    let partial = partial_receipt
        .into_continuation()
        .expect("partial recovery remains affine");
    let UiIntentRecoveryProgressOutcome::Progressed(completed) = world
        .session
        .retry_intent_recovery(partial, super::super::execution_reading(4))
    else {
        panic!("partial recovery must complete")
    };
    assert!(matches!(
        completed.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
}
