use std::num::NonZeroUsize;

use bank_domain::model::Money;
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::{BankSchema, Deposit};
use bank_server::{
    mutations, BankApplicationQueryDenial, BankMutationControls, BankMutationStatus,
};
use worth_query_host::facade::{
    admission::application_query::WorthQueryApplicationQueryLane,
    primary_graph::{
        WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryBasisPosture,
        WorthQueryApplicationQueryConsistency, WorthQueryApplicationQueryControls,
        WorthQueryApplicationQueryFreshness,
    },
};

use super::fixture::{ordinary_read_world, OrdinaryReadFixture, OWNER, TELLER};
use super::support::request_scope;

#[test]
fn account_activity_reads_one_real_prior_bank_commit() {
    let fixture = ordinary_read_world("historical-account-activity", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let historical_commit = commit_deposit(&fixture, &teller, 1, "historical-first");
    commit_deposit(&fixture, &teller, 2, "historical-second");

    let historical_request = request_scope();
    let historical = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .historical(
            &historical_commit,
            NonZeroUsize::new(16).unwrap(),
            NonZeroUsize::new(2_048).unwrap(),
            &historical_request,
        )
        .expect("the retained bank commit must remain queryable");
    let current_request = request_scope();
    let current = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .execute(
            WorthQueryApplicationQueryControls::<BankSchema>::current_one_shot(
                NonZeroUsize::new(16).unwrap(),
                NonZeroUsize::new(2_048).unwrap(),
                &current_request,
            ),
        )
        .expect("the current account activity must execute");

    assert_eq!(historical.rows()[0].entries().len(), 3);
    assert_eq!(current.rows()[0].entries().len(), 4);
    assert_eq!(historical.rows()[0].entries()[2].amount().minor_units(), 1);
    let receipt = historical.receipt();
    assert_eq!(receipt.lane(), WorthQueryApplicationQueryLane::Historical);
    assert_eq!(
        receipt.basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Historical
    );
    assert_eq!(
        receipt.consistency(),
        WorthQueryApplicationQueryConsistency::HistoricalSnapshot
    );
    assert_eq!(
        receipt.freshness(),
        WorthQueryApplicationQueryFreshness::Historical
    );
    assert_eq!(receipt.fallback_count(), 0);
    assert_eq!(receipt.per_result_neighbor_lookup_count(), 0);
    assert!(receipt.basis_released());
}

#[test]
fn foreign_bank_commit_receipt_cannot_select_local_history() {
    let source = ordinary_read_world("historical-receipt-source", 0);
    let target = ordinary_read_world("historical-receipt-target", 0);
    let source_teller = source.authenticate(TELLER);
    let target_owner = target.authenticate(OWNER);
    let target_teller = target.authenticate(TELLER);
    let foreign_commit = commit_deposit(&source, &source_teller, 1, "foreign-history");
    let local_commit = commit_deposit(&target, &target_teller, 1, "local-history");
    assert_eq!(
        foreign_commit.commit_id(),
        local_commit.commit_id(),
        "the hostile case requires colliding commit ordinals"
    );
    let request = request_scope();

    let denial = target
        .world
        .runtime
        .account_activity(target.personal_account)
        .as_principal(&target_owner)
        .historical(
            &foreign_commit,
            NonZeroUsize::new(16).unwrap(),
            NonZeroUsize::new(2_048).unwrap(),
            &request,
        )
        .err()
        .expect("a foreign bank commit receipt must not select local history");

    let BankApplicationQueryDenial::Admission(denial) = denial else {
        panic!("foreign receipt must deny during historical basis admission");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::ForeignHistoricalReceipt
    );
}

fn commit_deposit(
    fixture: &OrdinaryReadFixture,
    teller: &bank_server::BankAuthenticatedPrincipal,
    minor_units: i64,
    idempotency_key: &str,
) -> bank_server::BankCommitReceipt {
    let outcome = fixture
        .world
        .runtime
        .mutate(mutations::deposit(Deposit {
            institution: fixture.institution,
            account: fixture.personal_account,
            amount: Money::from_minor(minor_units).unwrap(),
        }))
        .as_principal(teller)
        .controls(BankMutationControls::new(
            request_scope(),
            BankIdempotencyKey::new(idempotency_key).unwrap(),
        ))
        .execute();
    match outcome.into_status() {
        BankMutationStatus::Committed(receipt) => receipt,
        status => panic!("deposit must commit, got {status:?}"),
    }
}
