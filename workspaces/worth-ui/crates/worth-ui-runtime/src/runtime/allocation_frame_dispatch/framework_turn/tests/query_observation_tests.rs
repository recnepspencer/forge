use super::{empty_artifact, framework_from_artifact};

#[test]
fn admission_is_effect_free_duplicate_and_leaves_handoff_retryable() {
    let mut fixture = worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::new(
        "observation-query-admission",
    );
    let reference = fixture.reference().clone();
    let mut binding = fixture.binding_plan().prepare_downstream_state();
    binding
        .admit_operation_live(fixture.open_resource())
        .expect("live resource belongs to the installed binding");
    fixture.update_measurement();
    let consequence = match binding
        .refresh_operation_live(fixture.refresh_request())
        .expect("exact Query change stages")
    {
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::Applied(consequence) => {
            consequence
        }
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("changed Query value must issue one UI consequence")
        }
    };
    let mut runtime = framework_from_artifact(empty_artifact());
    runtime.install_query_binding_state_for_test(binding);
    let before = runtime.operation_live_change_observation_for_test(&reference);
    let session =
        crate::facade::WorthUiActiveApplicationSessionIdentity::from_host_session_value(93);

    let mut turn = runtime.begin_observation_turn(session, 79).unwrap();
    turn.admit_query(consequence).unwrap();
    let admitted = turn.seal().unwrap();
    assert_eq!(
        admitted.summary().families(),
        &[crate::runtime::observation::UiObservationFamily::Query]
    );
    assert_eq!(
        runtime.operation_live_change_observation_for_test(&reference),
        before
    );

    drop(admitted);
    let replay = runtime
        .query_binding
        .retry_operation_live_change_handoff(&reference)
        .expect("effect-free observation leaves the Query handoff retryable");
    let mut repeated = runtime.begin_observation_turn(session, 79).unwrap();
    assert!(matches!(
        repeated.admit_query(replay),
        Err(
            crate::runtime::observation::UiQueryObservationAdmissionStop::Observation(
                crate::runtime::observation::UiObservationAdmissionDenial::DuplicateOwnerOrder
            )
        )
    ));
    drop(repeated);

    let retry = runtime
        .query_binding
        .retry_operation_live_change_handoff(&reference)
        .expect("duplicate rejection leaves the Query handoff retryable");
    runtime
        .query_binding
        .admit_operation_live_change(retry)
        .unwrap();
    assert_eq!(
        runtime
            .query_binding
            .publish_staged_operation_live_changes()
            .published_change_count(),
        1
    );
    let retirement = runtime.shutdown().into_operation_live_retirement();
    assert!(matches!(
        fixture.close_retirement(retirement),
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}
