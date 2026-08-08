use super::program_fixture::admitted_program;
use super::{
    authenticated_principal, idempotency, installed_authorization_world, live_scope,
    resolved_account,
};
use crate::domain_computation::primary_graph::tests::fixture::MultiTouchOperation;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitTerminalKind,
    WorthQueryApplicationIdempotencyResolution,
};

#[test]
fn equivalent_retry_recovers_original_receipt_while_intent_drift_is_denied() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let first = admitted_program(&world, &principal, &account, &request, "committed");
    let retry = admitted_program(&world, &principal, &account, &request, "committed");
    let drift = admitted_program(&world, &principal, &account, &request, "different");

    let WorthQueryApplicationCommitOutcome::Committed(original) = world
        .application
        .compare_and_commit_application(first, idempotency(9, 7))
    else {
        panic!("the first idempotent application attempt must commit");
    };
    let WorthQueryApplicationCommitOutcome::AlreadyCommitted(recovered) = world
        .application
        .compare_and_commit_application(retry, idempotency(9, 7))
    else {
        panic!("an equivalent retry must recover the original commit");
    };
    assert!(recovered.is_same_authoritative_commit(&original));
    assert_eq!(
        original.terminal().kind(),
        WorthQueryApplicationCommitTerminalKind::Executed
    );
    assert_eq!(
        recovered.terminal().kind(),
        WorthQueryApplicationCommitTerminalKind::Recovered
    );
    assert!(recovered.terminal().retry_inspection().is_none());

    let WorthQueryApplicationCommitOutcome::Denied(denial) = world
        .application
        .compare_and_commit_application(drift, idempotency(9, 8))
    else {
        panic!("reusing a key for another intent must be denied");
    };
    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift
    );
}

#[test]
fn same_caller_intent_cannot_cross_installed_operation_identity() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let first = admitted_program(&world, &principal, &account, &request, "committed");
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(first, idempotency(10, 10)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));

    let account = resolved_account(&world, "committed", &request);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(MultiTouchOperation::reference())
        .unwrap();
    let admission = world
        .application
        .authorize_operation(
            &principal,
            &account,
            &operation,
            Default::default(),
            &request,
        )
        .unwrap();
    let resolution = world
        .application
        .resolve_admitted_application_idempotency(&admission, idempotency(10, 10))
        .unwrap()
        .into_resolution();
    assert_eq!(
        resolution,
        WorthQueryApplicationIdempotencyResolution::IntentDrift
    );
}

#[test]
fn same_operation_intent_cannot_cross_an_admitted_scope() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let first_account = resolved_account(&world, "open", &request);
    let second_account = resolved_account(&world, "unrelated", &request);
    let first = admitted_program(&world, &principal, &first_account, &request, "first-scope");
    let second = admitted_program(
        &world,
        &principal,
        &second_account,
        &request,
        "second-scope",
    );

    assert!(matches!(
        world
            .application
            .compare_and_commit_application(first, idempotency(11, 11)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
    let WorthQueryApplicationCommitOutcome::Denied(denial) = world
        .application
        .compare_and_commit_application(second, idempotency(11, 11))
    else {
        panic!("reusing one operation intent across scopes must deny");
    };
    assert_eq!(
        denial.kind(),
        crate::domain_computation::primary_graph::WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift
    );
}
