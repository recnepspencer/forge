use bank_domain::model::{CustomerRole, Money};
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::{Deposit, GrantAccountAuthorization, RevokeAccountAuthorization};
use bank_server::{mutations, queries, BankMutationControls, BankMutationStatus, BankReadControls};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::domain::{
    WorthQueryCanonicalWorkEvidence, WorthQueryCanonicalWorkPhases,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationLiveControls, WorthQueryApplicationQueryControls,
};

use crate::fixture::{self, VIEWER};
use crate::support::request_scope;

pub(crate) fn commit_authorization_toggle(
    fixture: &fixture::OrdinaryReadFixture,
    owner: &bank_server::BankAuthenticatedPrincipal,
    ordinal: usize,
) {
    let outcome = if ordinal.is_multiple_of(2) {
        fixture
            .world
            .runtime
            .mutate(mutations::revoke_account_access(
                RevokeAccountAuthorization {
                    account: fixture.personal_account,
                    authorization: authorized_user_id(fixture, owner, VIEWER),
                },
            ))
            .as_principal(owner)
            .controls(mutation_controls(&format!("overflow-revoke-{ordinal}")))
            .execute()
    } else {
        fixture
            .world
            .runtime
            .mutate(mutations::grant_account_access(GrantAccountAuthorization {
                account: fixture.personal_account,
                principal: fixture::principal_id(VIEWER),
                role: CustomerRole::Viewer,
            }))
            .as_principal(owner)
            .controls(mutation_controls(&format!("overflow-grant-{ordinal}")))
            .execute()
    };
    assert!(matches!(outcome.status(), BankMutationStatus::Committed(_)));
}

pub(crate) fn authorized_user_id(
    fixture: &fixture::OrdinaryReadFixture,
    owner: &bank_server::BankAuthenticatedPrincipal,
    principal: usize,
) -> bank_domain::model::AccountAuthorizationId {
    let users = fixture
        .world
        .runtime
        .query(queries::account_authorized_users(fixture.personal_account))
        .as_principal(owner)
        .controls(BankReadControls::current(request_scope(), 16, 10_000).unwrap())
        .execute()
        .expect("owner should read authorized users");
    let [users] = users.rows() else {
        panic!("authorized users query must return one account row")
    };
    users
        .users()
        .iter()
        .find(|user| user.principal() == fixture::principal_id(principal))
        .expect("fixture authorization should exist")
        .authorization()
}

pub(crate) fn activity_count(
    fixture: &fixture::OrdinaryReadFixture,
    owner: &bank_server::BankAuthenticatedPrincipal,
) -> usize {
    let request = request_scope();
    fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(owner)
        .execute(WorthQueryApplicationQueryControls::current_one_shot(
            std::num::NonZeroUsize::new(64).unwrap(),
            std::num::NonZeroUsize::new(8_192).unwrap(),
            &request,
        ))
        .expect("owner should read installed account activity")
        .rows()[0]
        .entries()
        .len()
}

pub(crate) fn commit_deposit(
    fixture: &fixture::OrdinaryReadFixture,
    teller: &bank_server::BankAuthenticatedPrincipal,
    account: bank_domain::model::AccountId,
    key: &str,
) -> bank_server::BankMutationOutcome {
    let outcome = fixture
        .world
        .runtime
        .mutate(mutations::deposit(Deposit {
            institution: fixture.institution,
            account,
            amount: Money::from_minor(1).unwrap(),
        }))
        .as_principal(teller)
        .controls(BankMutationControls::new(
            request_scope(),
            BankIdempotencyKey::new(key).unwrap(),
        ))
        .execute();
    assert!(matches!(outcome.status(), BankMutationStatus::Committed(_)));
    outcome
}

pub(crate) fn live_controls() -> WorthQueryApplicationLiveControls {
    live_controls_for(request_scope(), 16)
}

pub(crate) fn live_controls_for(
    request: WorthQueryRequestScope,
    buffer_capacity: usize,
) -> WorthQueryApplicationLiveControls {
    WorthQueryApplicationLiveControls::bounded(request, buffer_capacity, 8, 2_048).unwrap()
}

pub(crate) fn assert_phase_posture(phases: WorthQueryCanonicalWorkPhases) {
    assert!(phases.installation().digest_derivations() > 0);
    assert!(phases.admission().digest_derivations() > 0);
    assert_zero(phases.execution());
    assert_zero(phases.provider_commit());
    assert_zero(phases.projection());
    assert_zero(phases.live_delivery());
    assert_zero(phases.retry_resolution());
    assert_zero(phases.recovery_inspection());
    assert_zero(phases.publication());
}

fn assert_zero(work: WorthQueryCanonicalWorkEvidence) {
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.canonical_entries(), 0);
    assert_eq!(work.canonical_encoded_bytes(), 0);
    assert_eq!(work.canonical_material_allocation_bytes(), 0);
    assert_eq!(work.sha256_input_bytes(), 0);
    assert_eq!(work.sha256_compression_blocks(), 0);
    assert_eq!(work.digest_text_materializations(), 0);
}

fn mutation_controls(key: &str) -> BankMutationControls {
    BankMutationControls::new(
        request_scope(),
        BankIdempotencyKey::new(key).expect("test idempotency key should admit"),
    )
}
