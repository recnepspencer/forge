use crate::{
    UiPresentProjection, UiProjectionAvailability, UiProjectionUnavailableKind,
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
