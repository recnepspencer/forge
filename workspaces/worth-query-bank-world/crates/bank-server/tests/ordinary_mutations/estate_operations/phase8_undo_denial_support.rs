//! Shared R8.39 undo-denial courtroom helpers.

use bank_domain::{
    model::JournalEntryId,
    reads::AccountActivityItem,
    schema::{PostingPurpose, ReleaseEstateOperation},
};
use bank_server::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome};
use worth_query_host::facade::domain::WorthQueryInstalledAftermathContract;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationQueryControls,
};

use super::disburse_estate::fixture::DisbursementFixture;
use crate::support::request_scope;

pub(super) fn commit_disbursement(
    fixture: &DisbursementFixture,
    key: u8,
) -> (
    BankAuthenticatedPrincipal,
    bank_server::BankCommitReceipt,
    JournalEntryId,
) {
    let specialist = fixture.authenticate_actor();
    let outcome = fixture
        .world
        .runtime
        .disburse_estate(
            &specialist,
            fixture.action(100),
            WorthQueryApplicationIdempotencyBinding::new([key; 32], [key.wrapping_add(1); 32]),
            &request_scope(),
        )
        .expect("disburse");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("disburse must commit: {outcome:?}");
    };
    let original = committed_journal_ids(fixture)
        .into_iter()
        .next()
        .expect("disbursement journal");
    (specialist, receipt, original)
}

pub(super) fn install_irreversible(
    runtime: &BankIdentityRuntime,
) -> WorthQueryInstalledAftermathContract {
    runtime.installed_operation_aftermath(ReleaseEstateOperation::reference())
}

pub(super) fn graph_snapshot(
    fixture: &DisbursementFixture,
) -> (Vec<JournalEntryId>, Vec<(i64, PostingPurpose)>) {
    let owner = fixture.authenticate_source_owner();
    let entries = account_activity(fixture, &owner, fixture.source);
    let journals = committed_journal_ids(fixture);
    let activity: Vec<_> = entries
        .iter()
        .map(|e| (e.amount().minor_units(), e.purpose()))
        .collect();
    (journals, activity)
}

pub(super) fn committed_journal_ids(fixture: &DisbursementFixture) -> Vec<JournalEntryId> {
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

/// Commit a reversal of `original` outside the undo lane, so an undo that
/// follows finds the journal already reversed and conflicts.
pub(super) fn commit_foreign_reversal(
    fixture: &DisbursementFixture,
    specialist: &BankAuthenticatedPrincipal,
    original: JournalEntryId,
    key: &str,
) {
    let already_reversed = bank_server::compensating_reverse_journal(
        bank_domain::model::InstitutionId::new(1).unwrap(),
        original,
    );
    let authorized = fixture
        .world
        .runtime
        .authorize_reverse_journal(
            specialist,
            already_reversed.institution,
            Default::default(),
            &request_scope(),
        )
        .expect("authorize prior reversal");
    let proposal = bank_server::BankOperationProposals::prepare_reverse_journal(
        &fixture.world.runtime,
        authorized,
        &bank_domain::proposals::BankIdempotencyKey::new(key).unwrap(),
        &already_reversed,
    )
    .expect("prepare prior reversal");
    assert!(matches!(
        fixture.world.runtime.commit_reverse_journal(proposal),
        Ok(BankMutationCommitOutcome::Committed(_))
    ));
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
