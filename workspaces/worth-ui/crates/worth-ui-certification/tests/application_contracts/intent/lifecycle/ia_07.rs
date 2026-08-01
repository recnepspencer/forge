use worth_ui::facade::intent::{
    UiIntentConsequenceContract, UiIntentConsequencePublicationOutcome,
    UiIntentConsequenceStopReason, UiIntentExecutionCancellationReason,
    UiIntentExecutionDispatchOutcome, UiIntentExecutionTransitionPosture,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture,
};
use worth_ui::facade::rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest};

use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::execution::consequence::ConsequenceWorld;
use crate::intent::execution::lifecycle::{
    advance, dispatch, execution_census, only_transition, AttemptStep, ExecutionScript,
    RecoveryStep, ScriptedProvider,
};

#[test]
fn phase_crossings_preserve_exact_terminal_authority_and_zero_execution_residue() {
    prove_phase_crossings_preserve_exact_terminal_authority_and_zero_execution_residue();
}

pub(in crate::intent) fn prove_phase_crossings_preserve_exact_terminal_authority_and_zero_execution_residue(
) {
    admitted_replacement_stops_before_provider_effect();
    effect_uncertainty_survives_target_removal();
    completed_effect_survives_generation_change_as_consequence_only_recovery();
    shutdown_disposes_uncertain_effect_without_claiming_rollback();
}

fn admitted_replacement_stops_before_provider_effect() {
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
    let cancelled = only_transition(advance(&mut world, 31));
    assert!(matches!(
        cancelled.posture(),
        UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. }
    ));
    assert_eq!(
        observation.cancellations()[0].reason(),
        UiIntentExecutionCancellationReason::ApplicationRebound
    );
    assert_eq!(execution_census(&world), [0, 0, 0, 0]);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_execution().active_after(), 0);
}

fn effect_uncertainty_survives_target_removal() {
    let (provider, _) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingEffectMayHaveBegun,
        AttemptStep::Completed,
    ])
    .with_cancellations([AttemptStep::CancelledBeforeEffect])
    .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 40);
    let started = only_transition(advance(&mut world, 1));
    let uncertain_basis = only_transition(advance(&mut world, 2));
    world.unmount(0).expect("the effecting target unmounts");

    let indeterminate = only_transition(advance(&mut world, 3));
    assert_eq!(indeterminate.attempt(), started.attempt());
    assert_eq!(indeterminate.idempotency(), uncertain_basis.idempotency());
    assert!(matches!(
        indeterminate.posture(),
        UiIntentExecutionTransitionPosture::Indeterminate { .. }
    ));
    let recovery = indeterminate
        .into_recovery()
        .expect("escaped-effect uncertainty retains affine recovery");
    let UiIntentRecoveryProgressOutcome::Progressed(recovered) = world
        .session
        .retry_intent_recovery(recovery, crate::intent::execution::execution_reading(4))
    else {
        panic!("the exact predecessor provider must remain recoverable")
    };
    assert!(matches!(
        recovered.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_execution().active_after(), 0);
}

fn completed_effect_survives_generation_change_as_consequence_only_recovery() {
    let mut world = ConsequenceWorld::launch(
        UiIntentConsequenceContract::mounted_posture_and_query(query_identity()),
    );
    let handle = world.complete_with_query();
    world.rebind_application();
    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(stop.reason(), UiIntentConsequenceStopReason::ApplicationGenerationChanged)
    ));
    drop(outcome);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

fn shutdown_disposes_uncertain_effect_without_claiming_rollback() {
    let (provider, observation) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingEffectMayHaveBegun,
    ])
    .with_cancellations([AttemptStep::PendingEffectMayHaveBegun])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    let admitted = world.admit_exact(0);
    assert!(matches!(
        world
            .session
            .dispatch_admitted_intent(admitted, crate::intent::execution::execution_deadline(40),),
        UiIntentExecutionDispatchOutcome::AttemptPrepared(_)
    ));
    let _ = only_transition(advance(&mut world, 1));
    let _ = only_transition(advance(&mut world, 2));
    let shutdown = world.session.shutdown();
    assert_eq!(
        shutdown.intent_execution().indeterminate_effect_disposals(),
        1
    );
    assert_eq!(shutdown.intent_execution().active_after(), 0);
    assert_eq!(observation.counts()[2], 1);
}

fn query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}
