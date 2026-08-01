use worth_ui::facade::{
    intent::{
        UiIntentConsequenceContract, UiIntentConsequencePublicationOutcome,
        UiIntentConsequenceStopReason,
    },
    observation::UiObservationProfile,
    rebind::{UiChangeProfile, UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindProfile},
};

use super::world::ConsequenceWorld;

#[test]
fn undeclared_query_consequence_stops_before_owner_admission_or_publication() {
    let mut world = ConsequenceWorld::launch(UiIntentConsequenceContract::none());
    let handle = world.complete_with_query();

    let recovery = match world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    ) {
        UiIntentConsequencePublicationOutcome::Stopped(stop) => {
            assert!(matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::UndeclaredQueryConsequence { observed }
                    if observed.as_str() == "certification.live.measurements"
            ));
            stop.into_recovery()
        }
        _ => panic!("an undeclared Query consequence must stop before publication"),
    };
    let stopped = world.query_change_state();
    assert_eq!(stopped.staged_change_count(), 1);
    assert_eq!(stopped.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);

    let repeated = world.interaction.session.retry_intent_consequences(
        recovery,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(41),
    );
    assert!(matches!(
        repeated,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(stop.reason(), UiIntentConsequenceStopReason::UndeclaredQueryConsequence { .. })
    ));
    drop(repeated);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

#[test]
fn complete_batch_capacity_denial_publishes_neither_query_nor_posture() {
    let mut budget = UiRebindProfile::platform_pulse().budget();
    budget.changed_facts = 1;
    let rebind = UiRebindProfile::bounded(budget, UiRebindProfile::platform_pulse().concurrency())
        .expect("one changed fact remains a valid bounded profile");
    let profile = UiChangeProfile::new(UiObservationProfile::platform_pulse(), rebind);
    let mut world = ConsequenceWorld::launch_with_change_profile(
        UiIntentConsequenceContract::mounted_posture_and_query(query_identity()),
        profile,
    );
    let frames_before = world.transcripts().len();
    let handle = world.complete_with_query();

    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::ConsequenceFactCapacityExceeded {
                    limit: 1,
                    observed: 2
                }
            )
    ));
    drop(outcome);
    let stopped = world.query_change_state();
    assert_eq!(stopped.staged_change_count(), 1);
    assert_eq!(stopped.admitted_change_count(), 0);
    assert_eq!(world.transcripts().len(), frames_before);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

#[test]
fn equal_query_identity_from_foreign_owner_stops_before_admission() {
    let mut world = ConsequenceWorld::launch(UiIntentConsequenceContract::query_collection_change(
        query_identity(),
    ));
    let (mut foreign_owner, foreign_consequence, foreign_binding) = foreign_consequence();
    let handle = world.complete_with_consequence(foreign_consequence);

    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::QueryAdmission(
                    worth_ui_query_binding::WorthUiCollectionChangeAdmissionDenial::ForeignInstalledReference
                )
            )
    ));
    drop(outcome);
    let local = world.query_change_state();
    assert_eq!(local.staged_change_count(), 0);
    assert_eq!(local.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();

    assert!(matches!(
        foreign_owner.close_retirement(foreign_binding.into_operation_live_retirement()),
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}

#[test]
fn declared_query_consequence_cannot_be_omitted_by_a_completed_outcome() {
    let mut world = ConsequenceWorld::launch(UiIntentConsequenceContract::query_collection_change(
        query_identity(),
    ));
    let handle = world.complete_without_consequences();

    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::MissingDeclaredQueryConsequence { expected }
                    if expected == &query_identity()
            )
    ));
    drop(outcome);
    let query = world.query_change_state();
    assert_eq!(query.staged_change_count(), 0);
    assert_eq!(query.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
}

fn foreign_consequence() -> (
    worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture,
    worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    worth_ui_query_binding::WorthUiRuntimeQueryBinding,
) {
    let mut owner = worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::new(
        "phase4-foreign-consequence-owner",
    );
    let resource = owner.open_resource();
    let mut binding = owner.binding_plan().prepare_downstream_state();
    binding
        .admit_operation_live(resource)
        .expect("foreign owner retains its exact live resource");
    owner.update_measurement();
    let consequence = match binding
        .refresh_operation_live(owner.refresh_request())
        .expect("foreign Query owner refresh succeeds")
    {
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::Applied(consequence) => {
            consequence
        }
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("foreign Query change must mint owner evidence")
        }
    };
    (owner, consequence, binding)
}

fn query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}
