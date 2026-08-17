use worth_ui::facade::{
    intent::{
        UiIntentConsequenceContract, UiIntentConsequencePublicationOutcome,
        UiIntentConsequenceStopReason,
    },
    rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest},
};
use worth_ui_host_headless::UiHeadlessRecorderCapacity;

use super::world::ConsequenceWorld;

#[test]
fn host_rejection_withdraws_query_admission_and_retries_only_consequences() {
    let mut world = ConsequenceWorld::launch_with_capacity(
        UiIntentConsequenceContract::mounted_posture_and_query(query_identity()),
        UiHeadlessRecorderCapacity::new(8, 1, 4_096),
    );
    assert_eq!(world.transcripts().len(), 1);
    let handle = world.complete_with_query();

    let recovery = match world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    ) {
        UiIntentConsequencePublicationOutcome::Stopped(stop) => {
            assert!(matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::HostRejectedBeforeEffects { rejection_count: 1 }
            ));
            stop.into_recovery()
        }
        _ => panic!("full recorder must stop before mounted effects"),
    };
    let stopped = world.query_change_state();
    assert_eq!(stopped.staged_change_count(), 1);
    assert_eq!(stopped.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);

    assert_eq!(world.drain_transcripts().len(), 1);
    match world.interaction.session.retry_intent_consequences(
        recovery,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(41),
    ) {
        UiIntentConsequencePublicationOutcome::Published(_) => {}
        UiIntentConsequencePublicationOutcome::Stopped(stop) => {
            panic!("consequence-only retry stopped: {:?}", stop.reason())
        }
        _ => panic!("consequence-only retry must publish synchronously"),
    }

    let published = world.query_change_state();
    assert_eq!(published.staged_change_count(), 0);
    assert_eq!(published.admitted_change_count(), 1);
    assert_eq!(published.next_change_order(), stopped.next_change_order());
    assert_eq!(world.provider_calls(), [1, 1]);
    let transcript = world
        .transcripts()
        .into_vec()
        .pop()
        .expect("retry publishes one mounted frame");
    assert!(transcript
        .semantic_text()
        .iter()
        .any(|text| text.text() == "COMPLETED"));
    world.shutdown();
}

fn query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}
