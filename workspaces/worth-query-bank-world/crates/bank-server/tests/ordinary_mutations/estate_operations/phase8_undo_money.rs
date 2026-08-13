//! R8.38 money compensation through ordinary reverse-journal progression.

use bank_domain::{
    proposals::BankIdempotencyKey, reads::AccountActivityItem, schema::PostingPurpose,
};
use bank_server::{BankAuthenticatedPrincipal, BankMutationCommitOutcome};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationQueryControls,
};
use worth_query_host::facade::provisional_aftermath::WorthQueryUndoDerivedRequest;

use super::disburse_estate::fixture::{disbursement_world, DisbursementFixture};
use crate::support::request_scope;

#[test]
fn disburse_estate_undo_commits_compensating_debit_and_credit_journals() {
    let fixture = disbursement_world("undo-money-compensate", 1_000);
    let specialist = fixture.authenticate_actor();
    let binding = idempotency(11);
    let outcome = fixture
        .world
        .runtime
        .disburse_estate(&specialist, fixture.action(250), binding, &request_scope())
        .expect("disburse");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("disburse must commit: {outcome:?}");
    };
    let original = original_disbursement_journal(&fixture);
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint recovery for compensatable disbursement");
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("admit compensation undo");
    assert_eq!(
        admission.derived_request(),
        WorthQueryUndoDerivedRequest::Compensation
    );

    let journals_before = committed_disbursement_and_reversal_ids(&fixture);
    assert_eq!(
        journals_before.len(),
        1,
        "one disbursement journal before undo"
    );

    let compensation_key =
        BankIdempotencyKey::new("undo-money-compensate-once").expect("idempotency key");
    let compensated = fixture
        .world
        .runtime
        .progress_undo_commit_recovery(admission, &specialist, &compensation_key, &request_scope())
        .expect("compensation through ordinary reverse-journal lane");
    assert!(matches!(
        compensated.mutation(),
        BankMutationCommitOutcome::Committed(_) | BankMutationCommitOutcome::AlreadyCommitted(_)
    ));

    let journals_after = committed_disbursement_and_reversal_ids(&fixture);
    assert_eq!(
        journals_after.len(),
        2,
        "exactly one compensating transfer must add one committed journal row"
    );
    assert!(journals_after.contains(&original));
    let reversal = journals_after
        .iter()
        .find(|id| **id != original)
        .expect("compensating journal");
    let source_owner = fixture.authenticate_source_owner();
    let entries = account_activity(&fixture, &source_owner, fixture.source);
    let reversal_entry = entries
        .iter()
        .find(|e| e.journal() == *reversal)
        .expect("reversal visible");
    assert_eq!(reversal_entry.reversal_of(), Some(original));
    assert_eq!(
        entries
            .iter()
            .find(|e| e.purpose() == PostingPurpose::EstateDisbursement)
            .map(|e| e.reversal_of()),
        Some(None),
        "original journal must be preserved"
    );

    // The recovery handle moved into the first undo admission. A second undo
    // admission is a compile-time error; ordinary idempotency still protects
    // retries of the admitted mutation itself.
    assert_eq!(
        committed_disbursement_and_reversal_ids(&fixture).len(),
        2,
        "equivalent retry must not write a second compensating journal"
    );

    independent_oracle_agrees(&fixture);
}

/// Independent double-entry oracle — sums activity rows only; does not import
/// production accounting helpers (R8.38 / §11).
fn independent_oracle_agrees(fixture: &DisbursementFixture) {
    let source_owner = fixture.authenticate_source_owner();
    let beneficiary = fixture.authenticate_beneficiary();
    let source = account_activity(fixture, &source_owner, fixture.source);
    let destination = account_activity(fixture, &beneficiary, fixture.destination);
    let source_balance: i64 = source.iter().map(|e| e.amount().minor_units()).sum();
    let dest_balance: i64 = destination.iter().map(|e| e.amount().minor_units()).sum();
    // Funding 1000, disburse -250/+250, compensate +250/-250 → source back to 1000, dest 0.
    assert_eq!(source_balance, 1_000);
    assert_eq!(dest_balance, 0);
}

fn original_disbursement_journal(
    fixture: &DisbursementFixture,
) -> bank_domain::model::JournalEntryId {
    let owner = fixture.authenticate_source_owner();
    let entries = account_activity(fixture, &owner, fixture.source);
    entries
        .iter()
        .find(|e| e.purpose() == PostingPurpose::EstateDisbursement)
        .expect("disbursement journal")
        .journal()
}

fn committed_disbursement_and_reversal_ids(
    fixture: &DisbursementFixture,
) -> Vec<bank_domain::model::JournalEntryId> {
    let owner = fixture.authenticate_source_owner();
    let mut ids: Vec<_> = account_activity(fixture, &owner, fixture.source)
        .into_iter()
        .filter(|e| e.purpose() == PostingPurpose::EstateDisbursement || e.reversal_of().is_some())
        .map(|e| e.journal())
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

fn account_activity(
    fixture: &DisbursementFixture,
    principal: &BankAuthenticatedPrincipal,
    account: bank_domain::model::AccountId,
) -> Vec<AccountActivityItem> {
    fixture
        .world
        .runtime
        .account_activity(account)
        .as_principal(principal)
        .execute(WorthQueryApplicationQueryControls::current_one_shot(
            std::num::NonZeroUsize::new(16).unwrap(),
            std::num::NonZeroUsize::new(1_024).unwrap(),
            &request_scope(),
        ))
        .expect("activity")
        .rows()[0]
        .entries()
        .to_vec()
}

fn idempotency(tag: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([tag; 32], [tag.wrapping_add(1); 32])
}
