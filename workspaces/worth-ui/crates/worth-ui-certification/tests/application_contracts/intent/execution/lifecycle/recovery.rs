use worth_ui::facade::intent::{
    UiIntentExecutionTransitionPosture, UiIntentProductOutcome, UiIntentRecoveryHandle,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture, UiIntentRecoveryProgressStop,
};

use super::provider::{
    AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider, ScriptedProviderObservation,
};
use super::{advance, dispatch, execution_census, only_transition};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::operability::EmptyOutcome;

#[test]
fn effect_uncertainty_rejects_a_late_before_effect_claim_and_recovers_in_place() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingEffectMayHaveBegun,
        AttemptStep::Completed,
    ])
    .with_cancellations([AttemptStep::CancelledBeforeEffect])
    .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 3);
    let _ = only_transition(advance(&mut world, 1));
    let pending = only_transition(advance(&mut world, 2));
    assert_eq!(
        pending.posture(),
        UiIntentExecutionTransitionPosture::PendingEffectMayHaveBegun
    );
    let uncertain = only_transition(advance(&mut world, 4));
    assert!(matches!(
        uncertain.posture(),
        UiIntentExecutionTransitionPosture::Indeterminate { detail: Some(_) }
    ));
    assert_eq!(uncertain.attempt(), pending.attempt());
    assert_eq!(uncertain.idempotency(), pending.idempotency());
    let recovery = uncertain
        .into_recovery()
        .expect("effect uncertainty must return recovery authority");
    let UiIntentRecoveryProgressOutcome::Progressed(recovered) = world
        .session
        .retry_intent_recovery(recovery, super::super::execution_reading(5))
    else {
        panic!("the retained attempt must recover in place")
    };
    assert_eq!(recovered.attempt(), pending.attempt());
    assert_eq!(recovered.idempotency(), pending.idempotency());
    assert_eq!(
        recovered.posture(),
        UiIntentRecoveryProgressPosture::Completed {
            outcome: EmptyOutcome::SCHEMA,
        }
    );
    assert!(recovered.into_continuation().is_none());
    let counts = observation.counts();
    assert_eq!(counts[1], 2, "protocol recovery polls the retained attempt");
    assert_eq!(counts[2], 1, "deadline cancellation is issued exactly once");
    assert_eq!(
        counts[3], 0,
        "a contradictory pre-effect claim cannot substitute a recovery object"
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}

#[test]
fn affine_recovery_preserves_identity_across_every_nonterminal_posture() {
    let (provider, observation) =
        ScriptedProvider::new([ExecutionScript::running([AttemptStep::PartialWithOutcome])
            .with_recovery([
                RecoveryStep::Pending,
                RecoveryStep::PartialWithoutOutcome,
                RecoveryStep::Indeterminate,
                RecoveryStep::Failed,
                RecoveryStep::Completed,
            ])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    let _ = only_transition(advance(&mut world, 1));
    let partial = only_transition(advance(&mut world, 2));
    assert!(matches!(
        partial.posture(),
        UiIntentExecutionTransitionPosture::Partial {
            outcome: Some(EmptyOutcome::SCHEMA),
            ..
        }
    ));
    let attempt = partial.attempt();
    let idempotency = partial.idempotency();
    let recovery = partial
        .into_recovery()
        .expect("partial effect is recoverable");

    let UiIntentRecoveryProgressOutcome::Stopped { reason, recovery } = world
        .session
        .retry_intent_recovery(recovery, super::super::execution_reading(1))
    else {
        panic!("regressed time must return the same affine authority")
    };
    assert_eq!(
        reason,
        UiIntentRecoveryProgressStop::MonotonicTimeRegressed {
            previous: 2,
            observed: 1,
        }
    );

    drive_recovery_sequence(&mut world, recovery, attempt, idempotency);
    assert_eq!(observation.counts()[3], 5);
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}

fn drive_recovery_sequence(
    world: &mut AdmissionWorld,
    recovery: UiIntentRecoveryHandle,
    attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
    idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
) {
    let expected = recovery_postures();
    let mut continuation = Some(recovery);
    for (offset, expected_posture) in expected.into_iter().enumerate() {
        let UiIntentRecoveryProgressOutcome::Progressed(receipt) =
            world.session.retry_intent_recovery(
                continuation.take().unwrap(),
                super::super::execution_reading(3 + offset as u64),
            )
        else {
            panic!("current recovery authority must progress")
        };
        assert_eq!(receipt.attempt(), attempt);
        assert_eq!(receipt.idempotency(), idempotency);
        assert_eq!(receipt.posture(), expected_posture);
        continuation = receipt.into_continuation();
    }
    assert!(continuation.is_none());
}

fn recovery_postures() -> [UiIntentRecoveryProgressPosture; 5] {
    [
        UiIntentRecoveryProgressPosture::Pending,
        UiIntentRecoveryProgressPosture::Partial {
            outcome: None,
            detail: worth_ui::facade::intent::UiIntentProviderStop::stable(
                "certification.recovery",
            ),
        },
        UiIntentRecoveryProgressPosture::Indeterminate {
            detail: worth_ui::facade::intent::UiIntentProviderStop::stable(
                "certification.recovery",
            ),
        },
        UiIntentRecoveryProgressPosture::Failed {
            detail: worth_ui::facade::intent::UiIntentProviderStop::stable(
                "certification.recovery",
            ),
        },
        UiIntentRecoveryProgressPosture::Completed {
            outcome: EmptyOutcome::SCHEMA,
        },
    ]
}

#[test]
fn foreign_session_recovery_is_rejected_and_returned_without_polling_either_lane() {
    let (mut left, left_observation, left_recovery) = recoverable_world();
    let (mut right, right_observation, right_recovery) = recoverable_world();
    let left_attempt = left_recovery.attempt();
    let left_idempotency = left_recovery.idempotency();

    let UiIntentRecoveryProgressOutcome::Stopped { reason, recovery } = right
        .session
        .retry_intent_recovery(left_recovery, super::super::execution_reading(3))
    else {
        panic!("a foreign session cannot poll another session's recovery lane")
    };
    assert_eq!(reason, UiIntentRecoveryProgressStop::StaleOrForeign);
    assert_eq!(recovery.attempt(), left_attempt);
    assert_eq!(recovery.idempotency(), left_idempotency);
    assert_eq!(left_observation.counts()[3], 0);
    assert_eq!(right_observation.counts()[3], 0);

    let UiIntentRecoveryProgressOutcome::Progressed(left_done) = left
        .session
        .retry_intent_recovery(recovery, super::super::execution_reading(3))
    else {
        panic!("the exact owner must accept the returned recovery authority")
    };
    assert!(matches!(
        left_done.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    let UiIntentRecoveryProgressOutcome::Progressed(right_done) = right
        .session
        .retry_intent_recovery(right_recovery, super::super::execution_reading(4))
    else {
        panic!("the foreign attempt did not disturb the local recovery lane")
    };
    assert!(matches!(
        right_done.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    assert_eq!(left_observation.counts()[3], 1);
    assert_eq!(right_observation.counts()[3], 1);
    let _ = left.session.shutdown();
    let _ = right.session.shutdown();
}

fn recoverable_world() -> (
    AdmissionWorld,
    ScriptedProviderObservation,
    UiIntentRecoveryHandle,
) {
    let (provider, observation) =
        ScriptedProvider::new([ExecutionScript::running([AttemptStep::Indeterminate])
            .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    let _ = only_transition(advance(&mut world, 1));
    let recovery = only_transition(advance(&mut world, 2))
        .into_recovery()
        .expect("indeterminate attempt exposes recovery authority");
    (world, observation, recovery)
}
