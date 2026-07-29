#[allow(
    dead_code,
    reason = "the shared read fixture includes scenarios used by sibling consumer courtrooms"
)]
#[path = "ordinary_reads/fixture.rs"]
mod fixture;
mod support;

use bank_domain::model::{Money, ReadOutcome};
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::Deposit;
use bank_server::{
    mutations, queries, BankActivityCursorDenial, BankMutationControls, BankMutationStatus,
    BankReadControlDenial, BankReadControls, BankReadDenial,
};

use fixture::{ordinary_read_world, OWNER, TELLER};
use support::request_scope;

#[test]
fn activity_pages_continue_only_at_the_exact_provider_version() {
    let fixture = ordinary_read_world("activity-pages", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let first = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_activity_page(fixture.personal_account))
            .as_principal(&owner)
            .controls(read_controls(1))
            .execute(),
    );
    assert_eq!(first.output().entries().len(), 1);
    let cursor = first.output().next().expect("first page must continue");

    let second = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_activity_page(fixture.personal_account).after(cursor))
            .as_principal(&owner)
            .controls(read_controls(1))
            .execute(),
    );
    assert_eq!(second.output().entries().len(), 1);
    assert_ne!(
        first.output().entries()[0].journal(),
        second.output().entries()[0].journal()
    );

    let mutation = fixture
        .world
        .runtime
        .mutate(mutations::deposit(Deposit {
            institution: fixture.institution,
            account: fixture.personal_account,
            amount: Money::from_minor(1).unwrap(),
        }))
        .as_principal(&teller)
        .controls(BankMutationControls::new(
            request_scope(),
            BankIdempotencyKey::new("activity-page-version-change").unwrap(),
        ))
        .execute();
    assert!(matches!(
        mutation.status(),
        BankMutationStatus::Committed(_)
    ));

    let stale = fixture
        .world
        .runtime
        .query(queries::account_activity_page(fixture.personal_account).after(cursor))
        .as_principal(&owner)
        .controls(read_controls(1))
        .execute();
    assert!(matches!(
        stale,
        ReadOutcome::Denied(BankReadDenial::ActivityCursor(
            BankActivityCursorDenial::StaleVersion { .. }
        ))
    ));
}

#[test]
fn activity_cursor_cannot_cross_account_scope() {
    let fixture = ordinary_read_world("activity-cursor-scope", 0);
    let owner = fixture.authenticate(OWNER);
    let first = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_activity_page(fixture.personal_account))
            .as_principal(&owner)
            .controls(read_controls(1))
            .execute(),
    );
    let cursor = first.output().next().expect("first page must continue");
    let crossed = fixture
        .world
        .runtime
        .query(queries::account_activity_page(fixture.business_account).after(cursor))
        .as_principal(&owner)
        .controls(read_controls(1))
        .execute();
    assert!(matches!(
        crossed,
        ReadOutcome::Denied(BankReadDenial::ActivityCursor(
            BankActivityCursorDenial::ForeignAccount
        ))
    ));
}

#[test]
fn activity_order_follows_committed_account_sequence_not_idempotency_derived_identity() {
    let fixture = ordinary_read_world("activity-chronology", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    const COMMIT_COUNT: usize = 12;
    for ordinal in 0..COMMIT_COUNT {
        let key = format!("activity-chronology-{ordinal}");
        let amount = 100 + ordinal as i64;
        let outcome = fixture
            .world
            .runtime
            .mutate(mutations::deposit(Deposit {
                institution: fixture.institution,
                account: fixture.personal_account,
                amount: Money::from_minor(amount).unwrap(),
            }))
            .as_principal(&teller)
            .controls(BankMutationControls::new(
                request_scope(),
                BankIdempotencyKey::new(key).unwrap(),
            ))
            .execute();
        assert!(matches!(outcome.status(), BankMutationStatus::Committed(_)));
    }

    let history = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_activity_page(fixture.personal_account))
            .as_principal(&owner)
            .controls(read_controls(32))
            .execute(),
    );
    let entries = history.output().entries();
    let recent = &entries[entries.len() - COMMIT_COUNT..];
    assert!(recent
        .windows(2)
        .all(|pair| pair[0].account_sequence() < pair[1].account_sequence()));
    for (ordinal, entry) in recent.iter().enumerate() {
        assert_eq!(entry.amount().minor_units(), 100 + ordinal as i64);
    }
    assert!(
        recent
            .windows(2)
            .any(|pair| pair[0].journal() > pair[1].journal()),
        "fixed commits must include an identity inversion so this proves sequence ordering"
    );
}

fn read_controls(maximum_results: usize) -> BankReadControls {
    BankReadControls::current(request_scope(), maximum_results)
        .map_err(|denial| match denial {
            BankReadControlDenial::ZeroResultLimit => "zero",
            BankReadControlDenial::ResultLimitTooLarge { .. } => "large",
        })
        .unwrap()
}

fn delivered<T, D>(outcome: ReadOutcome<T, D>) -> T {
    match outcome {
        ReadOutcome::Delivered(result) => result,
        _ => panic!("expected a delivered read"),
    }
}
