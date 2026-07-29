use super::{
    admitted_program, authenticated_principal, idempotency, installed_authorization_world,
    live_scope, resolved_account,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitDenialStage, WorthQueryApplicationCommitOutcome,
};

#[test]
fn preparation_rejection_is_denied_without_effect_or_idempotency_residue() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let rejected = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "prepared-replacement",
    );

    world.application.reject_next_session_prepare();
    let WorthQueryApplicationCommitOutcome::Denied(denial) = world
        .application
        .compare_and_commit_application(rejected, idempotency(19, 19))
    else {
        panic!("provider preparation rejection must be a typed denial");
    };
    assert_eq!(
        denial.stage(),
        WorthQueryApplicationCommitDenialStage::ProviderPlan
    );
    let _still_open = resolved_account(&world, "open", &live_scope());

    let retry = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "prepared-replacement",
    );
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(retry, idempotency(19, 19)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

#[test]
fn pretransaction_commit_failure_is_proved_aborted_and_applies_nothing() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let rejected = admitted_program(&world, &principal, &account, &request, "atomic-replacement");

    world.application.reject_next_commit_before_transaction();
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(rejected, idempotency(20, 20)),
        WorthQueryApplicationCommitOutcome::Aborted
    ));
    let _still_open = resolved_account(&world, "open", &live_scope());

    let retry = admitted_program(&world, &principal, &account, &request, "atomic-replacement");
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(retry, idempotency(20, 20)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

#[test]
fn index_publication_failure_recovers_the_committed_transaction_before_returning() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let first = admitted_program(&world, &principal, &account, &request, "index-replacement");
    let retry = admitted_program(&world, &principal, &account, &request, "index-replacement");

    world.application.fail_next_index_publication();
    let WorthQueryApplicationCommitOutcome::Committed(first_receipt) = world
        .application
        .compare_and_commit_application(first, idempotency(21, 21))
    else {
        panic!("index reconstruction must prove the committed transaction");
    };
    let WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt) = world
        .application
        .compare_and_commit_application(retry, idempotency(21, 21))
    else {
        panic!("index reconstruction must recover the committed idempotency record");
    };
    assert_eq!(receipt, first_receipt);
    assert!(receipt.changed_record_count() >= 2);
    let _committed = resolved_account(&world, "index-replacement", &live_scope());
}
