use worth_ui::facade::{
    intent::{
        UiIntentConsequenceContract, UiIntentConsequencePublicationOutcome,
        UiIntentConsequenceStopReason,
    },
    rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest},
};

use super::world::ConsequenceWorld;

#[test]
fn compatible_successor_frame_preserves_admitted_execution_consequence_affinity() {
    let mut world = ConsequenceWorld::launch(
        UiIntentConsequenceContract::mounted_posture_and_query(query_identity()),
    );
    let handle = world.complete_with_query();
    let (predecessor, successor) = world.publish_compatible_successor_frame();
    assert_ne!(predecessor, successor);

    let receipt = match world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    ) {
        UiIntentConsequencePublicationOutcome::Published(receipt) => receipt,
        UiIntentConsequencePublicationOutcome::InFlight(_) => {
            panic!("the headless consequence frame unexpectedly remained in flight")
        }
        UiIntentConsequencePublicationOutcome::Indeterminate(_) => {
            panic!("the headless consequence frame became indeterminate")
        }
        UiIntentConsequencePublicationOutcome::InternalDefect(_) => {
            panic!("the compatible successor produced an internal publication defect")
        }
        UiIntentConsequencePublicationOutcome::Stopped(stop) => {
            panic!(
                "compatible successor stopped consequence: {:?}",
                stop.reason()
            )
        }
        UiIntentConsequencePublicationOutcome::NoConsequences(_) => {
            panic!("declared mounted and Query consequences cannot be empty")
        }
    };
    drop(receipt);
    let published = world.query_change_state();
    assert_eq!(published.staged_change_count(), 0);
    assert_eq!(published.admitted_change_count(), 1);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

#[test]
fn target_unmount_after_effect_completion_retains_consequence_only_recovery() {
    let mut world = ConsequenceWorld::launch(
        UiIntentConsequenceContract::mounted_posture_and_query(query_identity()),
    );
    let handle = world.complete_with_query();
    world
        .interaction
        .unmount(0)
        .expect("the completed attempt target unmounts before consequence publication");

    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(stop.reason(), UiIntentConsequenceStopReason::TargetChanged(_))
    ));
    drop(outcome);
    let stopped = world.query_change_state();
    assert_eq!(stopped.staged_change_count(), 1);
    assert_eq!(stopped.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

#[test]
fn application_replacement_after_effect_completion_retains_consequence_only_recovery() {
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
    let stopped = world.query_change_state();
    assert_eq!(stopped.staged_change_count(), 1);
    assert_eq!(stopped.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

fn query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}
