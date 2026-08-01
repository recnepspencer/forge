use crate::{
    UiPresentProjection, UiProjectionAvailability, UiProjectionUnavailableKind,
    WorthUiScalarProjectionActionOutcome, WorthUiScalarProjectionActionRequest,
    WorthUiScalarProjectionHostPlan, WorthUiScalarProjectionSourceRecord,
};

#[test]
fn product_host_installation_and_move_only_source_owner_reach_pending_and_current() {
    let plan = WorthUiScalarProjectionHostPlan::prepare().expect("product plan prepares");
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .expect("host installs the exact admitted packages");
    let installed = completion
        .complete(installation)
        .expect("binding completion opens the production Query owner");

    let (registration, initial) = installed.into_parts();
    assert_eq!(
        registration.view().identity().as_str(),
        "platform.pulse.status"
    );
    assert_pending(initial.observation());
    assert_eq!(initial.observation().owner_order(), 1);
    let (observation, completion) = initial.into_parts();
    let fact = scalar_fact(observation);
    assert_pending_fact(&fact);
    let owner = completion
        .admit_publication(fact)
        .expect("the exact returned pending fact readmits its owner");

    let current = owner
        .advance(
            WorthUiScalarProjectionSourceRecord::new("ONLINE", 1).expect("native source record"),
        )
        .expect("owner-issued refresh reaches Query");
    assert_eq!(current.observation().owner_order(), 2);
    let (observation, completion) = current.into_parts();
    let fact = scalar_fact(observation);
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => {
            assert_eq!(value.as_str(), "ONLINE")
        }
        other => panic!("expected current native Query value, got {other:?}"),
    }
    let owner = completion
        .admit_publication(fact)
        .expect("the exact current fact readmits its owner");
    let updated = owner
        .advance(
            WorthUiScalarProjectionSourceRecord::new("UPDATED-LONG", 2)
                .expect("second native source record"),
        )
        .expect("owner-issued revalidation reaches Query");
    assert_eq!(updated.observation().owner_order(), 5);
    let (observation, completion) = updated.into_parts();
    let fact = scalar_fact(observation);
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => {
            assert_eq!(value.as_str(), "UPDATED-LONG")
        }
        other => panic!("expected updated native Query value, got {other:?}"),
    }
    let owner = completion
        .admit_publication(fact)
        .expect("the exact updated fact readmits its owner");
    let closed = owner.close().expect("the exact Query owner closes");
    assert!(closed.owner_terminal());
    assert_eq!(closed.live_source_count(), 0);
    assert_eq!(closed.live_attempt_count(), 0);
    assert_eq!(closed.live_resource_count(), 0);
    assert_eq!(closed.live_consumer_lease_count(), 0);
    assert_eq!(closed.retained_projection_count(), 0);
    assert_eq!(closed.projection_receipt_count(), 0);
}

#[test]
fn product_action_enters_query_refreshes_the_exact_live_target_and_closes() {
    let plan = WorthUiScalarProjectionHostPlan::prepare().expect("product plan prepares");
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .expect("host installs the exact admitted packages");
    let installed = completion
        .complete(installation)
        .expect("binding completion opens the production Query owner")
        .into_action_installation();

    let (_, initial) = installed.into_parts();
    let owner = publish_action_advance(initial);
    let current = owner
        .advance_source(
            WorthUiScalarProjectionSourceRecord::new("ONLINE", 1).expect("native source record"),
        )
        .expect("source truth reaches Query");
    let owner = publish_action_advance(current);

    let stale = WorthUiScalarProjectionActionRequest::new(0, "STALE").unwrap();
    let WorthUiScalarProjectionActionOutcome::Denied(denied) = owner.execute_action(stale) else {
        panic!("stale source revision must deny before Query execution");
    };
    assert_eq!(denied.active_revision(), 1);
    assert_eq!(denied.submitted_revision(), 0);
    let owner = denied.into_owner();

    let owner = execute_and_publish_action(owner, 1, "ACTION 1");
    let owner = execute_and_publish_action(owner, 1, "ACTION 2");
    assert_zero_close(owner.close().expect("action Query owner closes"));
}

fn execute_and_publish_action(
    owner: crate::WorthUiScalarProjectionActionLiveOwner,
    source_revision: u64,
    status: &str,
) -> crate::WorthUiScalarProjectionActionLiveOwner {
    let action = WorthUiScalarProjectionActionRequest::new(source_revision, status).unwrap();
    let executed = match owner.execute_action(action) {
        WorthUiScalarProjectionActionOutcome::Executed(executed) => executed,
        WorthUiScalarProjectionActionOutcome::Denied(denied) => panic!(
            "current product action denied: active={}, submitted={}",
            denied.active_revision(),
            denied.submitted_revision()
        ),
        WorthUiScalarProjectionActionOutcome::Indeterminate(indeterminate) => {
            let detail = indeterminate.detail().to_string();
            let _ = indeterminate.close();
            panic!("current product action became indeterminate: {detail}");
        }
    };
    assert_eq!(executed.evidence().source_revision(), source_revision);
    assert_eq!(executed.evidence().status(), status);
    assert!(!executed.evidence().query_receipt_digest().is_empty());
    assert_eq!(
        executed.evidence().affected_live_view_ids(),
        &["platform.pulse.status".to_string()]
    );
    let (_, action_advance) = executed.into_parts();
    let (observation, completion) = action_advance.into_parts();
    let fact = scalar_fact(observation);
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => {
            assert_eq!(value.as_str(), status)
        }
        other => panic!("expected Query-backed action value, got {other:?}"),
    }
    completion
        .admit_publication(fact)
        .expect("the exact action fact readmits its owner")
}

fn assert_pending(observation: &crate::UiProjectionObservation) {
    let crate::UiProjectionObservation::Scalar(observation) = observation else {
        panic!("product scalar owner must not issue collection evidence")
    };
    assert!(matches!(
        observation.fact().availability(),
        UiProjectionAvailability::Unavailable(unavailable)
            if unavailable.kind() == UiProjectionUnavailableKind::Pending
    ));
}

fn assert_pending_fact(fact: &crate::UiScalarProjectionFactReceipt) {
    assert!(matches!(
        fact.availability(),
        UiProjectionAvailability::Unavailable(unavailable)
            if unavailable.kind() == UiProjectionUnavailableKind::Pending
    ));
}

fn scalar_fact(
    observation: crate::UiProjectionObservation,
) -> crate::UiScalarProjectionFactReceipt {
    match observation {
        crate::UiProjectionObservation::Scalar(observation) => observation.into_fact(),
        crate::UiProjectionObservation::Collection(_) => {
            panic!("product scalar owner must not issue collection evidence")
        }
    }
}

fn publish_action_advance(
    advance: crate::WorthUiScalarProjectionActionAdvance,
) -> crate::WorthUiScalarProjectionActionLiveOwner {
    let (observation, completion) = advance.into_parts();
    completion
        .admit_publication(scalar_fact(observation))
        .expect("the exact action-capable fact readmits its owner")
}

fn assert_zero_close(closed: crate::WorthUiScalarProjectionSourceCloseReceipt) {
    assert!(closed.owner_terminal());
    assert_eq!(closed.live_source_count(), 0);
    assert_eq!(closed.live_attempt_count(), 0);
    assert_eq!(closed.live_resource_count(), 0);
    assert_eq!(closed.live_consumer_lease_count(), 0);
    assert_eq!(closed.retained_projection_count(), 0);
    assert_eq!(closed.projection_receipt_count(), 0);
}
