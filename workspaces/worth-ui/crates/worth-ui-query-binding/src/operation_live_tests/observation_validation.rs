use super::{applied, LiveBindingFixture};

#[test]
fn collection_change_observation_validation_is_effect_free() {
    let mut fixture = LiveBindingFixture::new("worth-ui-observation-validation");
    fixture.owner.update_measurement();
    let consequence = applied(fixture.refresh().unwrap());
    let before = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();

    let validated = fixture
        .binding
        .validate_operation_live_change_observation(consequence)
        .expect("the exact owner-issued consequence validates");
    let after = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();

    assert_eq!(validated.change_order(), 1);
    assert_eq!(after, before);
    assert_eq!(
        fixture
            .binding
            .publish_staged_operation_live_changes()
            .published_change_count(),
        0
    );
    drop(validated);

    let retry = fixture
        .binding
        .retry_operation_live_change_handoff(&fixture.reference)
        .expect("effect-free validation leaves the Query handoff retryable");
    fixture.admit_and_publish(retry);
    fixture.close();
}
