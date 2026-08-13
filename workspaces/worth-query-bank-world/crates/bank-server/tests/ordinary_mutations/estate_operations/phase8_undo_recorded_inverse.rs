//! RecordedInverse undo consumes retained pre-image through ordinary progression.

use bank_domain::{
    estate::EstateWorkflowStage,
    schema::{AccountStatus, FreezeEstateAccountOperation, Status},
};
use bank_server::{
    queries, BankCommitDenialKind, BankIdentityRuntime, BankMutationCommitOutcome,
    BankReadControls, BankUndoRetry,
};
use worth_foundational::facade::{AspectValue, InternedString};
use worth_query_host::facade::domain::{
    InstalledCorrectionMechanism, PublishedAftermathPosture, WorthQueryInstalledAftermathContract,
};
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
use worth_query_host::facade::provisional_aftermath::WorthQueryUndoDerivedRequest;

use super::freeze_account::fixture::{exact_freeze_world, FreezeFixture};
use crate::support::request_scope;

/// Freeze the fixture's account through the ordinary lane, returning the receipt
/// every undo journey below starts from.
fn commit_freeze(
    fixture: &FreezeFixture,
    specialist: &bank_server::BankAuthenticatedPrincipal,
    binding: WorthQueryApplicationIdempotencyBinding,
) -> bank_server::BankCommitReceipt {
    let outcome = fixture
        .world
        .runtime
        .freeze_estate_account(
            specialist,
            fixture.action(fixture.estate_account),
            binding,
            &request_scope(),
        )
        .expect("freeze");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("freeze must commit: {outcome:?}");
    };
    assert_eq!(estate_account_status(fixture), AccountStatus::Frozen);
    receipt
}

#[test]
fn recorded_inverse_undo_restores_prior_status_from_retained_preimage() {
    let fixture = exact_freeze_world("undo-recorded-inverse", AccountStatus::Open);
    let specialist = fixture.authenticate_specialist();
    let original_binding = WorthQueryApplicationIdempotencyBinding::new([21; 32], [22; 32]);
    let receipt = commit_freeze(&fixture, &specialist, original_binding);
    assert!(
        receipt.retained_preimage(),
        "freeze must retain Status pre-image for RecordedInverse"
    );

    let aftermath = install_freeze_aftermath(&fixture.world.runtime);
    assert_eq!(
        aftermath.published_posture(),
        PublishedAftermathPosture::Reversible
    );
    let Some(InstalledCorrectionMechanism::RecordedInverse(_)) = aftermath.mechanism() else {
        panic!("freeze aftermath must be RecordedInverse");
    };

    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_freeze_recovery(handle, &specialist, &request_scope())
        .expect("admit recorded inverse");
    assert_eq!(
        admission.derived_request(),
        WorthQueryUndoDerivedRequest::RecordedInverse
    );
    let prior = admission
        .retained_preimage()
        .expect("admission consumes retained pre-image")
        .field_for(Status::reference())
        .expect("Status slot consumed")
        .value()
        .clone();
    assert_eq!(prior, AspectValue::String(InternedString::from("open")));

    // The admitted one-shot continuation now enters its matching inverse lane.
    let restored = fixture
        .world
        .runtime
        .progress_undo_recorded_inverse(
            admission,
            &specialist,
            WorthQueryApplicationIdempotencyBinding::new([23; 32], [24; 32]),
            &request_scope(),
        )
        .expect("restore from retained pre-image");
    assert!(matches!(
        restored.mutation(),
        BankMutationCommitOutcome::Committed(_) | BankMutationCommitOutcome::AlreadyCommitted(_)
    ));
    assert_eq!(estate_account_status(&fixture), AccountStatus::Open);
}

fn install_freeze_aftermath(runtime: &BankIdentityRuntime) -> WorthQueryInstalledAftermathContract {
    runtime.installed_operation_aftermath(FreezeEstateAccountOperation::reference())
}

fn estate_account_status(fixture: &FreezeFixture) -> AccountStatus {
    let specialist = fixture.authenticate_specialist();
    let result = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&specialist)
        .controls(BankReadControls::current(request_scope(), 16, 20_000).unwrap())
        .execute()
        .expect("observe estate");
    let overview = &result.rows()[0];
    assert_eq!(overview.stage(), EstateWorkflowStage::Administration);
    overview.account().status()
}

/// `progress_undo_recorded_inverse` still takes a caller-authored idempotency
/// binding, so the sharpest remaining substitution on this surface is to hand it
/// the *original* commit's binding — aliasing the undo onto the very commit it
/// is undoing. If the binding decided idempotency alone, the runtime would
/// answer `AlreadyCommitted` and the caller would hold a proved undo for an undo
/// that never ran, with the account still frozen (Q8.22-C6).
///
/// It cannot: the caller's two halves are inert salt, and the other five axes are
/// bound from the admission. The alias denies as `IdempotencyIntentDrift`.
///
/// The second half is the point that makes the first half honest. A denial that
/// destroyed the commit's recovery would be a false guarantee dressed as a
/// defence, so this test then *completes* the undo through a fresh binding —
/// proving the denied attempt left a capability that still works, not merely a
/// free registry slot (Q8.22-C5).
#[test]
fn aliasing_the_original_binding_denies_and_leaves_the_undo_still_performable() {
    let fixture = exact_freeze_world("undo-alias-binding", AccountStatus::Open);
    let specialist = fixture.authenticate_specialist();
    let original_binding = WorthQueryApplicationIdempotencyBinding::new([21; 32], [22; 32]);
    let receipt = commit_freeze(&fixture, &specialist, original_binding);

    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_freeze_recovery(handle, &specialist, &request_scope())
        .expect("admit recorded inverse");
    let aliased = fixture
        .world
        .runtime
        .progress_undo_recorded_inverse(admission, &specialist, original_binding, &request_scope())
        .expect("the aliased attempt is answered, not errored");
    assert!(
        matches!(
            aliased.mutation(),
            BankMutationCommitOutcome::Denied {
                kind: BankCommitDenialKind::IdempotencyIntentDrift,
                ..
            }
        ),
        "aliasing the original binding must drift, not resolve as AlreadyCommitted: {:?}",
        aliased.mutation()
    );
    assert_eq!(
        estate_account_status(&fixture),
        AccountStatus::Frozen,
        "the denied undo must not restore the prior status"
    );
    assert!(
        !aliased.has_proved_undo(),
        "a denied undo mints no proved-undo evidence"
    );

    let (_, _, retry) = aliased.into_parts();
    let Some(BankUndoRetry::RecordedInverse(retry)) = retry else {
        panic!("safe idempotency drift must return the exact recorded-inverse authority");
    };
    complete_undo_after_denial(&fixture, &specialist, retry);
}

/// The other side of Q8.22-C5: relinquishing on drop must not hand back a
/// *usable* second undo.
///
/// A committed undo's handle moves into the redo continuation, which also
/// relinquishes when dropped. So a host that simply discards the undo outcome
/// returns the mint claim for a recovery that was genuinely exercised. That is
/// only sound because a second undo of an already-undone commit cannot land:
/// the account is no longer frozen, and beneath that Relational rechecks the
/// bound branch head (`expected_head` is `Commit(original)`, which the undo
/// commit has already advanced past).
///
/// This test performs the substitution rather than reasoning about it: it
/// re-mints after a successful undo and drives a full second undo attempt
/// through the production entry point.
#[test]
fn a_relinquished_handle_after_a_committed_undo_cannot_undo_twice() {
    let fixture = exact_freeze_world("undo-no-second-undo", AccountStatus::Open);
    let specialist = fixture.authenticate_specialist();
    let receipt = commit_freeze(
        &fixture,
        &specialist,
        WorthQueryApplicationIdempotencyBinding::new([31; 32], [32; 32]),
    );
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_freeze_recovery(handle, &specialist, &request_scope())
        .expect("admit recorded inverse");
    let restored = fixture
        .world
        .runtime
        .progress_undo_recorded_inverse(
            admission,
            &specialist,
            WorthQueryApplicationIdempotencyBinding::new([33; 32], [34; 32]),
            &request_scope(),
        )
        .expect("undo");
    assert!(matches!(
        restored.mutation(),
        BankMutationCommitOutcome::Committed(_)
    ));
    assert_eq!(estate_account_status(&fixture), AccountStatus::Open);
    // Discarding the redo continuation relinquishes the handle it holds.
    drop(restored);

    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("the discarded continuation returned the mint claim");
    let admission = fixture
        .world
        .runtime
        .admit_undo_freeze_recovery(handle, &specialist, &request_scope())
        .expect("admission is descriptive; it does not re-check the world");
    let second = fixture.world.runtime.progress_undo_recorded_inverse(
        admission,
        &specialist,
        WorthQueryApplicationIdempotencyBinding::new([35; 32], [36; 32]),
        &request_scope(),
    );
    assert!(
        second.is_err(),
        "a second undo of an already-undone commit must not progress: {:?}",
        second.map(|outcome| format!("{:?}", outcome.mutation()))
    );
    assert_eq!(
        estate_account_status(&fixture),
        AccountStatus::Open,
        "the refused second undo must change nothing"
    );
}

/// Carry the exact retry authority returned after a denied attempt all the way
/// to a committed undo.
///
/// A remint would prove only that a registry slot was freed. Consuming the
/// returned authority proves the capability itself survived (Q8.22-C5).
fn complete_undo_after_denial(
    fixture: &FreezeFixture,
    specialist: &bank_server::BankAuthenticatedPrincipal,
    admission: bank_server::BankRecordedInverseUndoAdmission,
) {
    let restored = fixture
        .world
        .runtime
        .progress_undo_recorded_inverse(
            admission,
            specialist,
            WorthQueryApplicationIdempotencyBinding::new([25; 32], [26; 32]),
            &request_scope(),
        )
        .expect("retry with a fresh binding");
    assert!(
        matches!(restored.mutation(), BankMutationCommitOutcome::Committed(_)),
        "the retry must actually undo: {:?}",
        restored.mutation()
    );
    assert_eq!(estate_account_status(fixture), AccountStatus::Open);
    assert!(
        restored.has_proved_undo(),
        "a committed undo seals proved-undo evidence"
    );
}
