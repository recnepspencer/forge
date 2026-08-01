use worth_ui::facade::intent::{
    UiIntentExecutionTransitionPosture, UiIntentRecoveryProgressOutcome,
    UiIntentRecoveryProgressPosture, UiIntentResourceCensus,
};

use super::{
    assert_empty, assert_evidence_count, assert_retirement, latest_evidence_reference,
    lookup_evidence,
};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::execution::execution_reading;
use crate::intent::execution::lifecycle::{
    advance, dispatch, only_transition, AttemptStep, ExecutionScript, RecoveryStep,
    ScriptedProvider,
};

#[test]
fn running_recovery_consequence_and_replacement_each_retire_once() {
    recovery_to_consequence_shutdown();
    replacement_cancellation();
}

fn recovery_to_consequence_shutdown() {
    let (provider, _) = ScriptedProvider::new([ExecutionScript::running([
        AttemptStep::PendingBeforeEffect,
        AttemptStep::Indeterminate,
    ])
    .with_recovery([RecoveryStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    assert_eq!(
        only_transition(advance(&mut world, 1)).posture(),
        UiIntentExecutionTransitionPosture::Started
    );
    assert_running_resources(census(&world));

    assert_eq!(
        only_transition(advance(&mut world, 2)).posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    );
    let indeterminate = only_transition(advance(&mut world, 3));
    assert_eq!(
        indeterminate.posture(),
        UiIntentExecutionTransitionPosture::Indeterminate { detail: None }
    );
    let recovery = indeterminate
        .into_recovery()
        .expect("indeterminate execution retains exact recovery authority");
    assert_recovering_resources(census(&world));

    let UiIntentRecoveryProgressOutcome::Progressed(completed) = world
        .session
        .retry_intent_recovery(recovery, execution_reading(4))
    else {
        panic!("recovery completes through its exact owner")
    };
    assert!(matches!(
        completed.posture(),
        UiIntentRecoveryProgressPosture::Completed { .. }
    ));
    let _consequence = completed
        .into_consequence()
        .expect("completed recovery retains one consequence receipt");
    assert_consequence_resources(census(&world));

    let shutdown = world.session.shutdown();
    assert_eq!(
        shutdown
            .intent_execution()
            .consequence_pending_outcomes_disposed(),
        1
    );
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}

fn assert_running_resources(census: UiIntentResourceCensus) {
    assert_eq!(census.execution_entries(), 1);
    assert_eq!(census.active_reservations(), 1);
    assert_eq!(census.running_executor_handles(), 1);
    assert_evidence_count(census, 1);
}

fn assert_recovering_resources(census: UiIntentResourceCensus) {
    assert_eq!(census.recovery_authorities(), 1);
    assert_eq!(census.active_reservations(), 1);
}

fn assert_consequence_resources(census: UiIntentResourceCensus) {
    assert_eq!(census.execution_entries(), 1);
    assert_eq!(census.active_reservations(), 0);
    assert_eq!(census.recovery_authorities(), 0);
    assert_eq!(census.consequence_receipts(), 1);
    assert_evidence_count(census, 1);
}

fn replacement_cancellation() {
    let (provider, _) =
        ScriptedProvider::new(
            [ExecutionScript::running([AttemptStep::PendingBeforeEffect])
                .with_cancellations([AttemptStep::CancelledBeforeEffect])],
        );
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 20);
    let _ = only_transition(advance(&mut world, 1));
    let _ = only_transition(advance(&mut world, 2));
    assert_eq!(census(&world).running_executor_handles(), 1);
    assert_evidence_count(census(&world), 1);
    let prior_evidence = latest_evidence_reference(&world.session);
    let observation_turn =
        world.begin_replacement_observation_turn_for_provider("ia-11-held-observation");
    let observation_bytes = observation_turn.resource_snapshot().retained_bytes();
    let held_observations = observation_turn
        .seal()
        .expect("replacement test retains one sealed observation set");
    let (observation_retirement, retirement) = world.rebind_application();
    super::assert_observation_retirement(
        observation_retirement,
        worth_ui::facade::observation::UiObservationResourceRetirementCause::ApplicationReplacement,
        1,
        1,
        observation_bytes,
    );
    assert_retirement(
        retirement,
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationReplacement,
        1,
    );
    assert!(lookup_evidence(&world.session, prior_evidence).is_none());
    drop(held_observations);
    assert!(matches!(
        only_transition(advance(&mut world, 3)).posture(),
        UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. }
    ));
    assert_eq!(census(&world), UiIntentResourceCensus::EMPTY);
    let shutdown = world.session.shutdown();
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        0,
    );
}

fn census(world: &AdmissionWorld) -> UiIntentResourceCensus {
    super::census(&world.session)
}
