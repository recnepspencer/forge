use std::cell::Cell;
use std::time::{Duration, Instant};

use super::fixture::{
    installed_authorization_world, live_scope, AccountLabel, AccountStatus, TouchAccountOperation,
};
use crate::domain_computation::primary_graph::{
    WorthQueryOperationAuthorizationDenialKind, WorthQueryOperationProjectionDenialKind,
    WorthQueryPrincipalResolutionMode,
};
use worth_query_admission::facade::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_schema::{
    OperationExpectsFact, TypedMutationPreconditions,
};

mod canonical_work_budgets;

impl OperationExpectsFact<TouchAccountOperation> for AccountLabel {}

#[test]
fn current_installed_membership_mints_exact_operation_admission() {
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
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();

    let admitted = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .unwrap();
    let retried = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &live_scope(),
        )
        .unwrap();

    assert_eq!(admitted.operation(), "TouchAccountOperation");
    assert_eq!(admitted.authorization_requirement_count(), 1);
    assert_eq!(admitted.allowed_graph_contract(), operation.contracts());
    assert_eq!(admitted.relational_counters().reconstructive_graph_scans, 0);
    assert!(admitted.signal_dependency_count() >= 2);
    assert_eq!(
        admitted.operation_scope_fingerprint(),
        retried.operation_scope_fingerprint()
    );
}

#[test]
fn caller_marker_cannot_widen_the_installed_precondition_contract() {
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
    let caller_only = TypedMutationPreconditions::new().expect_fact(
        AccountLabel::reference(),
        "forged-contract-widening".to_owned(),
    );

    let denial = world
        .application
        .authorize_operation(&principal, &account, &operation, caller_only, &request)
        .err()
        .expect("caller marker authority must not widen the installed contract");
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected
    );
}

#[test]
fn missing_membership_and_crossed_runtime_scope_open_no_operation_authority() {
    let denied_world = installed_authorization_world(false);
    let request = live_scope();
    let external = denied_world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = denied_world
        .application
        .resolve_authenticated_principal(
            &denied_world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = denied_world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = denied_world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let missing_membership = denied_world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .err()
        .expect("missing relationship must deny");
    assert_eq!(
        missing_membership.kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );

    let foreign = installed_authorization_world(true);
    let foreign_account = foreign
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let crossed_scope = denied_world
        .application
        .authorize_operation(
            &principal,
            &foreign_account,
            &operation,
            Default::default(),
            &request,
        )
        .err()
        .expect("foreign scope must deny");
    assert_eq!(
        crossed_scope.kind(),
        WorthQueryOperationAuthorizationDenialKind::ForeignRuntime
    );
}

#[test]
fn cancelled_request_cannot_reuse_otherwise_current_authority() {
    let world = installed_authorization_world(true);
    let live_request = live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &live_request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &live_request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_string(),
            &live_request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let cancellation = WorthQueryCancellationSource::new();
    let cancelled_request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    cancellation.cancel();

    let denial = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &cancelled_request,
        )
        .err()
        .expect("cancelled request must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::Cancelled
    );
}

#[test]
fn admitted_operation_retains_expiry_and_cancellation_authority() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let external = world.authenticate("alice", Duration::from_millis(20), &request);
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
            "open".to_string(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(TouchAccountOperation::reference())
        .unwrap();
    let expiring = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .unwrap();
    assert!(expiring.validate_current_authority().is_ok());
    std::thread::sleep(Duration::from_millis(30));
    assert_eq!(
        expiring.validate_current_authority().unwrap_err().kind(),
        WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication
    );
    let expired_projection_ran = Cell::new(false);
    let expired_projection = world
        .invariant
        .project_admitted_operation(&expiring, |_, _| expired_projection_ran.set(true))
        .err()
        .expect("expired admission must deny before projection");
    assert_eq!(
        expired_projection.kind(),
        WorthQueryOperationProjectionDenialKind::Authorization(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication
        )
    );
    assert!(!expired_projection_ran.get());

    let cancellation = WorthQueryCancellationSource::new();
    let cancellable_request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let external = world.authenticate("alice", Duration::from_secs(60), &cancellable_request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &cancellable_request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let cancellable = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &cancellable_request,
        )
        .unwrap();
    cancellation.cancel();
    assert_eq!(
        cancellable.validate_current_authority().unwrap_err().kind(),
        WorthQueryOperationAuthorizationDenialKind::Cancelled
    );
    let cancelled_projection_ran = Cell::new(false);
    let cancelled_projection = world
        .invariant
        .project_admitted_operation(&cancellable, |_, _| cancelled_projection_ran.set(true))
        .err()
        .expect("cancelled admission must deny before projection");
    assert_eq!(
        cancelled_projection.kind(),
        WorthQueryOperationProjectionDenialKind::Authorization(
            WorthQueryOperationAuthorizationDenialKind::Cancelled
        )
    );
    assert!(!cancelled_projection_ran.get());
}
