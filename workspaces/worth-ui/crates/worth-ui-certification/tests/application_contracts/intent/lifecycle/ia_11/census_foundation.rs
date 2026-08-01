use worth_ui::facade::observation_report::UiHostPointerButtonTransition;

use super::{
    assert_empty, assert_evidence_count, assert_observation_retirement, assert_only_evidence,
    assert_retirement, census, latest_evidence,
};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::confirmation::ConfirmationWorld;
use crate::intent::execution::execution_deadline;
use crate::intent::interaction_world::InteractionWorld;

#[test]
fn gesture_capture_challenge_admission_and_prepared_execution_reach_exact_zero() {
    observation_turn_drop_and_retry();
    gesture_and_capture_shutdown();
    confirmation_shutdown();
    admission_release_and_repeated_cleanup();
    prepared_execution_shutdown();
}

fn observation_turn_drop_and_retry() {
    let mut world = AdmissionWorld::launch(1);
    let turn = world.begin_replacement_observation_turn();
    let active = turn.resource_snapshot();
    assert_eq!(active.active_turns(), 1);
    assert_eq!(active.retained_sets(), 0);
    assert_eq!(active.retained_observations(), 1);
    assert!(active.retained_bytes() > 0);
    let retained_bytes = active.retained_bytes();
    let set = turn.seal().expect("the admitted observation set seals");
    let retained = census(&world.session);
    assert_eq!(retained.active_observation_turns(), 0);
    assert_eq!(retained.retained_observation_sets(), 1);
    assert_eq!(retained.retained_observations(), 1);
    assert_eq!(retained.retained_observation_bytes(), retained_bytes);
    drop(set);
    assert_empty(census(&world.session));

    let retry = world.session.begin_observation_turn().unwrap();
    assert_eq!(retry.resource_snapshot().active_turns(), 1);
    drop(retry);
    assert_empty(census(&world.session));

    let mut shutdown_world = AdmissionWorld::launch(1);
    let turn = shutdown_world.begin_replacement_observation_turn();
    let retained_bytes = turn.resource_snapshot().retained_bytes();
    let held = turn
        .seal()
        .expect("shutdown world seals one observation set");
    let shutdown = shutdown_world.session.shutdown();
    assert_empty(shutdown.intent_resource_census());
    assert_observation_retirement(
        shutdown.observation_resources(),
        worth_ui::facade::observation::UiObservationResourceRetirementCause::ApplicationShutdown,
        1,
        1,
        retained_bytes,
    );
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        0,
    );
    drop(held);
}

fn gesture_and_capture_shutdown() {
    let mut world = InteractionWorld::canonical();
    assert_empty(census(&world.session));
    let _ = world.button(1, 1, UiHostPointerButtonTransition::Pressed, [10, 20]);
    let active = census(&world.session);
    assert_eq!(active.active_pointer_gestures(), 1);
    assert_eq!(active.active_pointer_captures(), 1);
    let shutdown = world.session.shutdown();
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        0,
    );
    assert_eq!(shutdown.interaction().cancelled_gestures(), 1);
    assert_eq!(
        shutdown
            .interaction()
            .final_state()
            .expect("interaction shutdown owns one final state")
            .active_gestures(),
        0
    );
}

fn confirmation_shutdown() {
    let mut world = ConfirmationWorld::launch();
    assert_empty(census(&world.interaction.session));
    let _challenge = world.issue();
    let active = census(&world.interaction.session);
    assert_eq!(active.pending_challenges(), 1);
    assert_eq!(active.retained_confirmation_candidates(), 1);
    assert_eq!(active.retained_confirmation_payloads(), 1);
    assert_evidence_count(active, 1);
    assert_eq!(
        latest_evidence(&world.interaction.session).input().family(),
        worth_ui_inspection::UiIntentInteractionEvidenceFamily::Activate
    );
    let shutdown = world.interaction.session.shutdown();
    assert_eq!(shutdown.intent_confirmation().settled_challenges(), 1);
    assert_eq!(shutdown.intent_confirmation().pending_after(), 0);
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}

fn admission_release_and_repeated_cleanup() {
    let mut world = AdmissionWorld::launch(1);
    assert_empty(census(&world.session));
    let admitted = world.admit_exact(0);
    let active = census(&world.session);
    assert_eq!(active.execution_entries(), 1);
    assert_eq!(active.active_reservations(), 1);
    assert_eq!(active.retained_admission_candidates(), 1);
    assert_eq!(active.retained_payloads(), 1);
    assert_eq!(active.retained_owner_references(), 5);
    assert_eq!(active.retained_payload_bytes(), 0);
    assert_evidence_count(active, 1);

    let settlement = world.session.cancel_admitted_intent(admitted);
    assert_eq!(settlement.active_after(), 0);
    assert_only_evidence(census(&world.session), 1);
    let retry = world.admit_exact(0);
    assert_eq!(census(&world.session).active_reservations(), 1);
    assert_evidence_count(census(&world.session), 2);
    let retry_settlement = world.session.cancel_admitted_intent(retry);
    assert_eq!(retry_settlement.active_after(), 0);
    assert_only_evidence(census(&world.session), 2);
    let _ = world.unmount(0);
    assert_only_evidence(census(&world.session), 2);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_admission().settled_attempts(), 0);
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        2,
    );
}

fn prepared_execution_shutdown() {
    let mut world = AdmissionWorld::launch(1);
    let admitted = world.admit_exact(0);
    let outcome = world
        .session
        .dispatch_admitted_intent(admitted, execution_deadline(20));
    assert!(matches!(
        outcome,
        worth_ui::facade::intent::UiIntentExecutionDispatchOutcome::AttemptPrepared(_)
    ));
    let active = census(&world.session);
    assert_eq!(active.execution_entries(), 1);
    assert_eq!(active.active_reservations(), 1);
    assert_eq!(active.prepared_executor_handles(), 1);
    assert_evidence_count(active, 1);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_execution().execution_entries_disposed(), 1);
    assert_eq!(shutdown.intent_execution().before_effect_disposals(), 1);
    assert_empty(shutdown.intent_resource_census());
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}
