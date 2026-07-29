use std::time::Duration;

use super::application_attempt::{
    admitted_program_with_emissions, authenticated_principal, idempotency, resolved_account,
};
use super::fixture::{
    installed_authorization_world, live_scope, AccountActivityEffect, AccountStatus,
    TouchAccountOperation,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryLiveDeliveryControls,
    WorthQueryLiveDeliveryOpenDenialKind, WorthQueryLiveDeliveryOutcome,
    WorthQueryPrincipalResolutionMode,
};

worth_query_declaration::worth_query_effect!(
    pub UninstalledLiveEffect(String) in super::fixture::IdentityExecutionSchema
);

#[test]
fn same_typed_foreign_scope_cannot_drive_an_existing_live_lease() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let first = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let second = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "unrelated".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let first = world
        .application
        .authorize_operation(&principal, &first, &operation, &request)
        .unwrap();
    let second = world
        .application
        .authorize_operation(&principal, &second, &operation, &request)
        .unwrap();
    let controls = WorthQueryLiveDeliveryControls::bounded(request, 1).unwrap();
    let mut lease = world
        .application
        .open_live_effect_lease(&first, AccountActivityEffect::reference(), controls)
        .unwrap();

    assert!(matches!(
        lease.poll(&second),
        WorthQueryLiveDeliveryOutcome::ScopeMismatch
    ));
}

#[test]
fn uninstalled_typed_effect_cannot_open_a_live_lease() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(&principal, &account, &operation, &request)
        .unwrap();
    let controls = WorthQueryLiveDeliveryControls::bounded(request, 1).unwrap();
    let denial = match world.application.open_live_effect_lease(
        &admission,
        UninstalledLiveEffect::reference(),
        controls,
    ) {
        Err(denial) => denial,
        Ok(_) => panic!("uninstalled typed effect must not open a live lease"),
    };
    assert_eq!(
        denial.kind(),
        WorthQueryLiveDeliveryOpenDenialKind::UninstalledEffect
    );
}

#[test]
fn runtime_delivery_closure_is_visible_to_an_existing_lease() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(&principal, &account, &operation, &request)
        .unwrap();
    let controls = WorthQueryLiveDeliveryControls::bounded(request, 1).unwrap();
    let mut lease = world
        .application
        .open_live_effect_lease(&admission, AccountActivityEffect::reference(), controls)
        .unwrap();

    world.application.close_live_delivery();
    assert!(matches!(
        lease.poll(&admission),
        WorthQueryLiveDeliveryOutcome::Closed
    ));
}

#[test]
fn transient_queue_release_failure_preserves_every_matching_emission_in_order() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(&principal, &account, &operation, &request)
        .unwrap();
    let controls = WorthQueryLiveDeliveryControls::bounded(request.clone(), 1).unwrap();
    let mut lease = world
        .application
        .open_live_effect_lease(&admission, AccountActivityEffect::reference(), controls)
        .unwrap();
    let program = admitted_program_with_emissions(
        &world,
        &principal,
        &account,
        &request,
        "two-emissions",
        ["first", "second"],
    );
    let WorthQueryApplicationCommitOutcome::Committed(receipt) = world
        .application
        .compare_and_commit_application(program, idempotency(44, 44))
    else {
        panic!("multi-emission program must commit");
    };

    lease.fail_next_queue_release();
    assert!(matches!(
        lease.poll(&admission),
        WorthQueryLiveDeliveryOutcome::Unavailable
    ));
    assert_eq!(lease.buffered_cause_count(), 1);
    for expected in ["first", "second"] {
        let WorthQueryLiveDeliveryOutcome::Delivered(cause) = lease.poll(&admission) else {
            panic!("each matching emission must be delivered");
        };
        assert_eq!(cause.commit_id(), receipt.commit_id());
        assert_eq!(cause.payload(), expected);
    }
    assert!(matches!(
        lease.poll(&admission),
        WorthQueryLiveDeliveryOutcome::Pending
    ));
}
