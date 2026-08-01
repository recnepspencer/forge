use std::time::Duration;

use super::fixture::{
    installed_authorization_world, installed_authorization_world_on_branch, live_scope,
    AccountStatus, TouchAccountOperation,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationHistoricalRead,
    WorthQueryApplicationIdempotencyBinding, WorthQueryPrincipalResolutionMode,
};
#[path = "application_attempt/authority_lifecycle.rs"]
mod authority_lifecycle;
#[path = "application_attempt/authorization_causality.rs"]
mod authorization_causality;
#[path = "application_attempt/decision_adjacency.rs"]
mod decision_adjacency;
#[path = "application_attempt/decision_manifest_ceiling.rs"]
mod decision_manifest_ceiling;
#[path = "application_attempt/effect_authority.rs"]
mod effect_authority;
#[path = "application_attempt/emitted_effects.rs"]
mod emitted_effects;
#[path = "application_attempt/program_fixture.rs"]
mod program_fixture;
#[path = "application_attempt/terminal_failures.rs"]
mod terminal_failures;
#[path = "application_attempt/touched_graph_closure.rs"]
mod touched_graph_closure;

use program_fixture::{
    admitted_program, admitted_program_with_emit, admitted_program_with_expected_status,
};

#[test]
fn committed_receipt_reopens_historical_truth_on_its_exact_non_main_branch() {
    let world = installed_authorization_world_on_branch(true, "tenant-blue");
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let program = admitted_program(&world, &principal, &account, &request, "committed");
    let WorthQueryApplicationCommitOutcome::Committed(receipt) = world
        .application
        .compare_and_commit_application(program, idempotency(41, 41))
    else {
        panic!("the non-main application attempt must commit");
    };
    assert_eq!(receipt.branch_id().0, "tenant-blue");

    let basis = world
        .application
        .admit_application_historical_basis(
            WorthQueryApplicationHistoricalRead::at_application_commit(&receipt),
            &request,
        )
        .expect("the commit receipt must reopen its own branch-qualified truth");
    assert_eq!(basis.identity().branch_id(), receipt.branch_id());
    assert!(basis.release().released());
}

#[test]
fn same_fact_race_stales_loser_while_unrelated_drift_does_not_conflict() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let unrelated = resolved_account(&world, "unrelated", &request);

    let first = admitted_program(&world, &principal, &account, &request, "first");
    let losing = admitted_program(&world, &principal, &account, &request, "losing");
    let unrelated_program =
        admitted_program(&world, &principal, &unrelated, &request, "unrelated-after");

    let unrelated_outcome = world
        .application
        .compare_and_commit_application(unrelated_program, idempotency(1, 1));
    assert!(
        matches!(
            unrelated_outcome,
            WorthQueryApplicationCommitOutcome::Committed(_)
        ),
        "unexpected unrelated outcome: {unrelated_outcome:?}"
    );
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(first, idempotency(2, 2)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
    let WorthQueryApplicationCommitOutcome::Stale(stale) = world
        .application
        .compare_and_commit_application(losing, idempotency(3, 3))
    else {
        panic!("the second same-fact attempt must be stale");
    };
    assert_eq!(stale.stale_fact_count(), 1);
}

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
    assert_eq!(recovered, original);
    let original_graph = original
        .graph_work()
        .expect("the original commit retains its graph-work transcript");
    let recovered_graph = recovered
        .graph_work()
        .expect("the equivalent retry retains its own graph-work transcript");
    assert_ne!(
        original_graph.session_identity(),
        recovered_graph.session_identity(),
        "equivalent commit meaning does not collapse distinct attempt sessions"
    );
    assert_eq!(
        original_graph.provider_session_identity(),
        original_graph.session_identity().render_hex(),
    );
    assert_eq!(
        recovered_graph.provider_session_identity(),
        recovered_graph.session_identity().render_hex(),
    );

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
fn concurrent_equivalent_attempts_publish_one_transaction() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let first = admitted_program(&world, &principal, &account, &request, "once");
    let second = admitted_program(&world, &principal, &account, &request, "once");

    let (left, right) = std::thread::scope(|scope| {
        let left = scope.spawn(|| {
            world
                .application
                .compare_and_commit_application(first, idempotency(12, 12))
        });
        let right = scope.spawn(|| {
            world
                .application
                .compare_and_commit_application(second, idempotency(12, 12))
        });
        (left.join().unwrap(), right.join().unwrap())
    });
    let receipts = [left, right]
        .into_iter()
        .map(|outcome| match outcome {
            WorthQueryApplicationCommitOutcome::Committed(receipt)
            | WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt) => receipt,
            unexpected => panic!("unexpected concurrent outcome: {unexpected:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(receipts[0], receipts[1]);
}

#[test]
fn concurrent_independent_attempts_both_commit() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let first_account = resolved_account(&world, "open", &request);
    let second_account = resolved_account(&world, "unrelated", &request);
    let first = admitted_program(
        &world,
        &principal,
        &first_account,
        &request,
        "first-independent",
    );
    let second = admitted_program(
        &world,
        &principal,
        &second_account,
        &request,
        "second-independent",
    );

    let (left, right) = std::thread::scope(|scope| {
        let left = scope.spawn(|| {
            world
                .application
                .compare_and_commit_application(first, idempotency(13, 13))
        });
        let right = scope.spawn(|| {
            world
                .application
                .compare_and_commit_application(second, idempotency(14, 14))
        });
        (left.join().unwrap(), right.join().unwrap())
    });
    assert!(matches!(
        left,
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
    assert!(matches!(
        right,
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

#[test]
fn response_loss_resolves_the_published_commit_before_returning() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let first = admitted_program_with_expected_status(
        &world,
        &principal,
        &account,
        &request,
        ("open", "published"),
    );
    let retry = admitted_program_with_expected_status(
        &world,
        &principal,
        &account,
        &request,
        ("open", "published"),
    );

    world.application.lose_next_commit_response();
    let WorthQueryApplicationCommitOutcome::Committed(first_receipt) = world
        .application
        .compare_and_commit_application(first, idempotency(15, 15))
    else {
        panic!("authoritative idempotency must prove the response-lost commit");
    };
    let WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt) = world
        .application
        .compare_and_commit_application(retry, idempotency(15, 15))
    else {
        panic!("retry must recover the transaction published before response loss");
    };
    assert_eq!(receipt, first_receipt);
    assert_eq!(
        receipt.precondition_comparison().expected_version_count(),
        0
    );
    assert_eq!(receipt.precondition_comparison().expected_fact_count(), 1);
    assert!(receipt.changed_record_count() >= 2);
}

#[test]
fn preparation_commit_recovery_and_retry_perform_no_execution_digest_derivation() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let before = crate::execution_digest::test_hash_parts_call_count();
    let first = admitted_program(&world, &principal, &account, &request, "digest-free");
    let retry = admitted_program(&world, &principal, &account, &request, "digest-free");

    world.application.lose_next_commit_response();
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(first, idempotency(31, 31)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
    let after_commit = crate::execution_digest::test_hash_parts_call_count();
    assert_eq!(
        after_commit,
        before,
        "preparation, provider execution, commit, or response-loss recovery derived legacy execution digests: {:?}",
        crate::execution_digest::test_hash_parts_domains_since(before)
    );

    assert!(matches!(
        world
            .application
            .compare_and_commit_application(retry, idempotency(31, 31)),
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(_)
    ));
    assert_eq!(
        crate::execution_digest::test_hash_parts_call_count(),
        after_commit,
        "idempotent retry or recovery derived a legacy execution digest"
    );
}

pub(super) fn idempotency(key: u8, intent: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([key; 32], [intent; 32])
}

pub(super) fn authenticated_principal(
    world: &super::fixture::AuthorizationWorld,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
) -> crate::domain_computation::primary_graph::WorthQueryAuthenticatedPrincipal<
    super::fixture::IdentityExecutionSchema,
    super::fixture::Principal,
    u64,
> {
    let external = world.authenticate("alice", Duration::from_secs(60), request);
    world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
}

pub(super) fn resolved_account(
    world: &super::fixture::AuthorizationWorld,
    status: &str,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
) -> crate::domain_computation::primary_graph::WorthQueryApplicationEntityIdentity<
    super::fixture::IdentityExecutionSchema,
    super::fixture::Account,
> {
    world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            status.to_string(),
            request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap()
}
