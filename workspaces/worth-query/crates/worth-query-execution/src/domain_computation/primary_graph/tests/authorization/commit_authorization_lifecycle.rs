use std::cell::Cell;
use std::time::{Duration, Instant};

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::application_attempt::authenticated_principal;
use super::super::fixture::{installed_capability_authorization_world, live_scope};
use super::capability_progression::{admitted_capability_operation, time};
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[test]
fn commit_authorization_rechecks_cancellation_when_governed() {
    let mut world = installed_capability_authorization_world();
    world
        .application
        .script_authorization_time([time(100), time(100), time(100)]);
    let cancellation = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let principal = authenticated_principal(&world, &request);
    let mut admission = admitted_capability_operation(&world, &principal, &request);
    let (_, commit_basis) = admission
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();
    let proof = world
        .application
        .authorize_application_commit(&admission, &commit_basis, &serialization)
        .unwrap();

    cancellation.cancel();

    let governed_action_ran = Cell::new(false);
    assert!(proof
        .govern((), |()| governed_action_ran.set(true))
        .is_err());
    assert!(!governed_action_ran.get());
}

#[test]
fn commit_basis_cannot_be_paired_with_a_different_admitted_operation() {
    let mut world = installed_capability_authorization_world();
    world
        .application
        .script_authorization_time([time(100), time(100), time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let mut source = admitted_capability_operation(&world, &principal, &request);
    let target = admitted_capability_operation(&world, &principal, &request);
    let (_, source_basis) = source
        .take_authorization_dependencies(world.application.authorization.bridge())
        .unwrap();
    let serialization = world
        .application
        .primary_provider
        .serialize_application_commit();

    let Err(denial) =
        world
            .application
            .authorize_application_commit(&target, &source_basis, &serialization)
    else {
        panic!("a commit basis must remain bound to its originating admission");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::InconsistentDecision
    );
}
