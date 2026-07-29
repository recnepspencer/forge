use std::time::{Duration, Instant};

use bank_domain::model::ReadOutcome;
use bank_server::{queries, BankReadControls};
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::fixture::{ordinary_read_world, APPROVER, AUDITOR, OWNER, STRANGER, TELLER, VIEWER};
use crate::support::request_scope;

#[test]
fn account_visibility_does_not_imply_account_access_management() {
    let fixture = ordinary_read_world("read-account-authority", 0);
    let viewer = fixture.authenticate(VIEWER);
    let owner = fixture.authenticate(OWNER);
    let stranger = fixture.authenticate(STRANGER);

    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::account_detail(fixture.personal_account))
            .as_principal(&viewer)
            .controls(controls(8))
            .execute(),
        ReadOutcome::Delivered(_)
    ));
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::account_authorized_users(fixture.personal_account))
            .as_principal(&viewer)
            .controls(controls(8))
            .execute(),
        ReadOutcome::Denied(_)
    ));
    let users = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_authorized_users(fixture.personal_account))
            .as_principal(&owner)
            .controls(controls(8))
            .execute(),
    );
    assert!(users
        .output()
        .iter()
        .any(|user| user.principal().get() == u64::try_from(VIEWER).unwrap() + 1));
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::account_detail(fixture.personal_account))
            .as_principal(&stranger)
            .controls(controls(8))
            .execute(),
        ReadOutcome::Denied(_)
    ));
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::account_detail(fixture.recipient_account))
            .as_principal(&owner)
            .controls(controls(8))
            .execute(),
        ReadOutcome::Denied(_)
    ));
}

#[test]
fn payment_and_audit_reads_preserve_distinct_authority_paths() {
    let fixture = ordinary_read_world("read-payment-authority", 0);
    let approver = fixture.authenticate(APPROVER);
    let owner = fixture.authenticate(OWNER);
    let auditor = fixture.authenticate(AUDITOR);
    let teller = fixture.authenticate(TELLER);

    let pending = delivered(
        fixture
            .world
            .runtime
            .query(queries::pending_payments())
            .as_principal(&approver)
            .controls(controls(8))
            .execute(),
    );
    assert_eq!(pending.output().as_slice(), [payment_summary(&fixture)]);
    let owner_pending = delivered(
        fixture
            .world
            .runtime
            .query(queries::pending_payments())
            .as_principal(&owner)
            .controls(controls(8))
            .execute(),
    );
    assert!(owner_pending.output().is_empty());
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::payment(fixture.payment))
            .as_principal(&owner)
            .controls(controls(1))
            .execute(),
        ReadOutcome::Delivered(_)
    ));

    let audit = delivered(
        fixture
            .world
            .runtime
            .query(queries::institution_audit(fixture.institution))
            .as_principal(&auditor)
            .controls(controls(16))
            .execute(),
    );
    assert!(!audit.output().is_empty());
    assert_eq!(audit.metadata().work().reconstructive_scans(), 0);
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::institution_audit(fixture.institution))
            .as_principal(&teller)
            .controls(controls(16))
            .execute(),
        ReadOutcome::Denied(_)
    ));
}

#[test]
fn cancellation_and_deadline_are_typed_before_projection() {
    let fixture = ordinary_read_world("read-interruption", 0);
    let owner = fixture.authenticate(OWNER);
    let cancellation = WorthQueryCancellationSource::new();
    cancellation.cancel();
    let cancelled = BankReadControls::current(
        WorthQueryRequestScope::new(
            Instant::now() + Duration::from_secs(60),
            cancellation.token(),
        ),
        8,
    )
    .unwrap();
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::accounts())
            .as_principal(&owner)
            .controls(cancelled)
            .execute(),
        ReadOutcome::Cancelled
    ));

    let deadline_source = WorthQueryCancellationSource::new();
    let expired = BankReadControls::current(
        WorthQueryRequestScope::new(Instant::now(), deadline_source.token()),
        8,
    )
    .unwrap();
    assert!(matches!(
        fixture
            .world
            .runtime
            .query(queries::accounts())
            .as_principal(&owner)
            .controls(expired)
            .execute(),
        ReadOutcome::DeadlineExceeded
    ));
}

fn payment_summary(
    fixture: &super::fixture::OrdinaryReadFixture,
) -> bank_domain::reads::PaymentSummary {
    let approver = fixture.authenticate(APPROVER);
    delivered(
        fixture
            .world
            .runtime
            .query(queries::payment(fixture.payment))
            .as_principal(&approver)
            .controls(controls(1))
            .execute(),
    )
    .into_output()
}

fn controls(maximum_results: usize) -> BankReadControls {
    BankReadControls::current(request_scope(), maximum_results).unwrap()
}

fn delivered<T, D>(outcome: ReadOutcome<T, D>) -> T {
    match outcome {
        ReadOutcome::Delivered(result) => result,
        _ => panic!("expected a delivered read"),
    }
}
