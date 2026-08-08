#[path = "freeze_account/fixture.rs"]
pub(crate) mod fixture;

use bank_domain::{estate::EstateWorkflowStage, schema::AccountStatus};
use bank_server::{
    queries, BankEstateFreezeProjectionDenial, BankEstateProgressionDenial,
    BankMutationCommitOutcome, BankReadControls,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationIdempotencyBinding,
};

use self::fixture::{exact_freeze_world, foreign_account_freeze_world, FreezeFixture};
use crate::support::request_scope;

#[test]
fn public_query_progression_freezes_the_exact_estate_account() {
    let fixture = exact_freeze_world("estate-freeze-commit", AccountStatus::Open);
    let specialist = fixture.authenticate_specialist();
    let binding = idempotency(11);
    let outcome = fixture
        .world
        .runtime
        .freeze_estate_account(
            &specialist,
            fixture.action(fixture.estate_account),
            binding,
            &request_scope(),
        )
        .expect("the exact estate account should reach Query commit");

    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("the first exact freeze must commit: {outcome:?}");
    };
    assert_eq!(receipt.changed_record_count(), 2);
    assert_eq!(receipt.emitted_effect_count(), 0);
    assert_eq!(receipt.expected_fact_count(), 0);
    assert_eq!(receipt.decision_fact_count(), Some(5));
    assert_freeze_canonical_work(receipt.canonical_work());
    assert_eq!(estate_account_status(&fixture), AccountStatus::Frozen);

    let retry = fixture
        .world
        .runtime
        .freeze_estate_account(
            &specialist,
            fixture.action(fixture.estate_account),
            binding,
            &request_scope(),
        )
        .expect("an equivalent authorized retry should inspect Query idempotency");
    let BankMutationCommitOutcome::AlreadyCommitted(recovered) = retry else {
        panic!("the equivalent retry must recover the exact commit: {retry:?}");
    };
    assert!(receipt.is_same_authoritative_commit(&recovered));
    assert_freeze_canonical_work(recovered.canonical_work());
    assert_eq!(estate_account_status(&fixture), AccountStatus::Frozen);

    let drift = fixture
        .world
        .runtime
        .freeze_estate_account(
            &specialist,
            fixture.action(fixture.estate_account),
            WorthQueryApplicationIdempotencyBinding::new([11; 32], [99; 32]),
            &request_scope(),
        )
        .expect("intent drift is a typed commit outcome rather than a projection error");
    assert!(matches!(
        drift,
        BankMutationCommitOutcome::Denied {
            kind: WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift,
            stage: WorthQueryApplicationCommitDenialStage::Idempotency,
        }
    ));
}

#[test]
fn foreign_grant_account_reaches_projection_but_cannot_receive_effect_authority() {
    let fixture = foreign_account_freeze_world("estate-freeze-foreign-account");
    let specialist = fixture.authenticate_specialist();
    let denial = fixture
        .world
        .runtime
        .freeze_estate_account(
            &specialist,
            fixture.action(fixture.foreign_account),
            idempotency(21),
            &request_scope(),
        )
        .expect_err("a grant-bound foreign account must fail after graph observation");

    assert!(matches!(
        denial,
        BankEstateProgressionDenial::FreezeProjection(
            BankEstateFreezeProjectionDenial::RelatedAccountMismatch {
                expected,
                observed,
            }
        ) if expected == fixture.foreign_account && observed == fixture.estate_account
    ));
    assert_eq!(estate_account_status(&fixture), AccountStatus::Open);
    assert_eq!(foreign_account_status(&fixture), AccountStatus::Open);
}

#[test]
fn non_open_accounts_cannot_begin_a_second_freeze_transition() {
    for (ordinal, status) in [AccountStatus::Frozen, AccountStatus::Closed]
        .into_iter()
        .enumerate()
    {
        let fixture = exact_freeze_world(&format!("estate-freeze-{status:?}"), status);
        let specialist = fixture.authenticate_specialist();
        let denial = fixture
            .world
            .runtime
            .freeze_estate_account(
                &specialist,
                fixture.action(fixture.estate_account),
                idempotency(31 + ordinal as u8),
                &request_scope(),
            )
            .expect_err("only an open account may enter the freeze transition");

        assert!(matches!(
            denial,
            BankEstateProgressionDenial::FreezeProjection(
                BankEstateFreezeProjectionDenial::AccountNotOpen(observed)
            ) if observed == status
        ));
        assert_eq!(estate_account_status(&fixture), status);
    }
}

fn estate_account_status(fixture: &FreezeFixture) -> AccountStatus {
    let specialist = fixture.authenticate_specialist();
    let result = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&specialist)
        .controls(read_controls())
        .execute()
        .expect("the assigned specialist should observe the estate account");
    let overview = &result.rows()[0];
    assert_eq!(overview.stage(), EstateWorkflowStage::Administration);
    assert_eq!(overview.account().id(), fixture.estate_account);
    overview.account().status()
}

fn foreign_account_status(fixture: &FreezeFixture) -> AccountStatus {
    let owner = fixture.authenticate_foreign_owner();
    fixture
        .world
        .runtime
        .query(queries::account_summary(fixture.foreign_account))
        .as_principal(&owner)
        .controls(read_controls())
        .execute()
        .expect("the foreign account owner should observe their account")
        .rows()[0]
        .status()
}

fn read_controls() -> BankReadControls {
    BankReadControls::current(request_scope(), 16, 20_000).unwrap()
}

fn idempotency(identity: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([identity; 32], [identity + 1; 32])
}

fn assert_freeze_canonical_work(
    phases: worth_query_host::facade::domain::WorthQueryCanonicalWorkPhases,
) {
    let input_identity = phases.admission();
    assert_eq!(input_identity.basis_preparations(), 1);
    assert_eq!(input_identity.digest_derivations(), 1);
    assert_eq!(input_identity.canonical_encoded_bytes(), 821);
    assert_eq!(input_identity.digest_text_materializations(), 0);
    for work in [
        phases.installation(),
        phases.execution(),
        phases.provider_commit(),
        phases.projection(),
        phases.live_delivery(),
        phases.retry_resolution(),
        phases.recovery_inspection(),
        phases.publication(),
    ] {
        assert_eq!(work.basis_preparations(), 0);
        assert_eq!(work.digest_derivations(), 0);
        assert_eq!(work.canonical_encoded_bytes(), 0);
        assert_eq!(work.digest_text_materializations(), 0);
    }
}
