use bank_domain::{
    estate::{EstateAction, EstateDisbursement, EstatePosting},
    model::{Money, SignedMoney},
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationIdempotencyBinding,
};

use crate::estate_capability_admission::fixture::{
    disbursement_world, request_scope, ACCOUNT, APPROVER, DECEASED, ESTATE, NOTICE, OTHER_ACCOUNT,
};

#[test]
fn journal_and_revision_drift_after_materialization_stales_provider_commit() {
    let fixture = disbursement_world("estate-disbursement-provider-currentness");
    let specialist = fixture.authenticate();
    let action = action(250);
    let admission = fixture
        .runtime
        .admit_estate_disbursement(&specialist, action, &request_scope())
        .expect("the exact disbursement should admit");
    let program = fixture
        .runtime
        .materialize_estate_disbursement(admission, idempotency(211))
        .expect("the exact disbursement should materialize while balances are current");

    let competing = fixture
        .runtime
        .disburse_estate(&specialist, action, idempotency(213), &request_scope())
        .expect("a competing lawful disbursement should change both account revisions");
    assert!(matches!(
        competing,
        crate::BankMutationCommitOutcome::Committed(_)
    ));

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_application(program, idempotency(211));
    assert!(matches!(
        outcome,
        WorthQueryApplicationCommitOutcome::Stale(_)
    ));
}

#[test]
fn unrelated_death_notice_drift_does_not_stale_the_disbursement() {
    let fixture = disbursement_world("estate-disbursement-unrelated-currentness");
    let specialist = fixture.authenticate();
    let action = action(250);
    let admission = fixture
        .runtime
        .admit_estate_disbursement(&specialist, action, &request_scope())
        .expect("the exact disbursement should admit");
    let program = fixture
        .runtime
        .materialize_estate_disbursement(admission, idempotency(221))
        .expect("the exact disbursement should materialize");

    let unrelated = fixture
        .runtime
        .notify_estate_death(
            &specialist,
            EstateAction::NotifyDeath {
                estate: ESTATE,
                notice: NOTICE,
                subject: DECEASED,
            },
            idempotency(223),
            &request_scope(),
        )
        .expect("the unrelated notice transition should commit");
    assert!(matches!(
        unrelated,
        crate::BankMutationCommitOutcome::Committed(_)
    ));

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_application(program, idempotency(221));
    assert!(matches!(
        outcome,
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

fn action(amount: i64) -> EstateAction {
    EstateAction::DisburseEstate(EstateDisbursement {
        estate: ESTATE,
        source_account: ACCOUNT,
        destination_account: OTHER_ACCOUNT,
        beneficiary: APPROVER,
        amount: Money::from_minor(amount).unwrap(),
        postings: [
            EstatePosting {
                account: ACCOUNT,
                amount: SignedMoney::from_minor(-amount),
            },
            EstatePosting {
                account: OTHER_ACCOUNT,
                amount: SignedMoney::from_minor(amount),
            },
        ],
    })
}

fn idempotency(seed: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([seed; 32], [seed + 1; 32])
}
