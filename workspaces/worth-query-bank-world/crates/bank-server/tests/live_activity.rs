#[allow(
    dead_code,
    reason = "the shared read fixture includes scenarios used by sibling consumer courtrooms"
)]
#[path = "ordinary_reads/fixture.rs"]
mod fixture;
mod support;

use std::time::{Duration, Instant};

use bank_domain::model::{CustomerRole, Money, ReadOutcome};
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::{Deposit, GrantAccountAuthorization, RevokeAccountAuthorization};
use bank_server::{
    mutations, queries, BankActivityLiveOutcome, BankLiveControls, BankMutationControls,
    BankMutationStatus, BankReadControls,
};

use fixture::{ordinary_read_world, OWNER, TELLER, VIEWER};
use support::request_scope;
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

#[test]
fn live_activity_delivers_only_matching_commits_as_fresh_reads() {
    let fixture = ordinary_read_world("live-activity", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let mut live = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(live_controls())
        .expect("authorized activity lease should open");
    assert!(matches!(live.poll(), BankActivityLiveOutcome::Pending));

    commit_deposit(
        &fixture,
        &teller,
        fixture.recipient_account,
        "unrelated-live-deposit",
    );
    assert!(matches!(live.poll(), BankActivityLiveOutcome::Pending));

    let before = activity_count(&fixture, &owner);
    commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "matching-live-deposit",
    );
    let outcome = live.poll();
    let BankActivityLiveOutcome::Delivered(update) = outcome else {
        panic!("matching commit must deliver a fresh account projection: {outcome:?}");
    };
    assert_eq!(
        update.activity().output().account_sequence().get(),
        u64::try_from(before).unwrap() + 1
    );
    assert_eq!(update.activity().metadata().result_count(), 1);
    assert!(!update.activity().metadata().truncated());
    assert!(update.commit_id() > 0);
    live.close();
}

#[test]
fn permission_revocation_closes_before_another_payload_is_delivered() {
    let fixture = ordinary_read_world("live-revocation", 0);
    let owner = fixture.authenticate(OWNER);
    let viewer = fixture.authenticate(VIEWER);
    let authorization = authorized_user_id(&fixture, &owner, VIEWER);
    let mut live = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&viewer)
        .subscribe(live_controls())
        .expect("viewer should open activity lease");

    let revoked = fixture
        .world
        .runtime
        .mutate(mutations::revoke_account_access(
            RevokeAccountAuthorization {
                account: fixture.personal_account,
                authorization,
            },
        ))
        .as_principal(&owner)
        .controls(BankMutationControls::new(
            request_scope(),
            BankIdempotencyKey::new("revoke-live-viewer").unwrap(),
        ))
        .execute();
    assert!(matches!(revoked.status(), BankMutationStatus::Committed(_)));
    assert!(matches!(
        live.poll(),
        BankActivityLiveOutcome::AuthorizationRevoked(_)
    ));
}

#[test]
fn cancellation_and_deadline_are_distinct_live_terminals() {
    let fixture = ordinary_read_world("live-interruption", 0);
    let owner = fixture.authenticate(OWNER);
    let cancellation = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    );
    let mut cancelled = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(BankLiveControls::current(request, 16, 2).unwrap())
        .expect("live lease should open before cancellation");
    cancellation.cancel();
    let cancelled_outcome = cancelled.poll();
    assert!(
        matches!(cancelled_outcome, BankActivityLiveOutcome::Cancelled),
        "unexpected cancellation outcome: {cancelled_outcome:?}"
    );

    let deadline = Instant::now() + Duration::from_millis(25);
    let deadline_source = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(deadline, deadline_source.token());
    let mut expired = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(BankLiveControls::current(request, 16, 2).unwrap())
        .expect("live lease should open before its deadline");
    while Instant::now() < deadline {
        std::thread::yield_now();
    }
    let expired_outcome = expired.poll();
    assert!(
        matches!(expired_outcome, BankActivityLiveOutcome::DeadlineExceeded),
        "unexpected deadline outcome: {expired_outcome:?}"
    );
}

#[test]
fn caller_cannot_enlarge_the_installed_live_buffer_ceiling() {
    let fixture = ordinary_read_world("live-buffer-ceiling", 0);
    let owner = fixture.authenticate(OWNER);
    let denial = match fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(BankLiveControls::current(request_scope(), 16, 3).unwrap())
    {
        Err(denial) => denial,
        Ok(_) => panic!("caller buffer wider than installed queue authority must fail"),
    };
    assert!(matches!(
        denial,
        bank_server::BankLiveOpenDenial::Delivery(
            worth_query_host::facade::primary_graph::WorthQueryLiveDeliveryOpenDenialKind::BufferCapacityExceedsInstalled
        )
    ));
}

#[test]
fn admitted_buffer_capacity_retains_multiple_matching_commit_causes() {
    let fixture = ordinary_read_world("live-buffer-retention", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let mut warm_work = None;
    for ordinal in 0..16 {
        let outcome = commit_deposit(
            &fixture,
            &teller,
            fixture.personal_account,
            &format!("preexisting-live-history-{ordinal}"),
        );
        let work = outcome.metadata().projection_work().unwrap();
        if ordinal == 0 {
            assert_eq!(work.aggregate_cache_hits(), 0);
        } else {
            assert_eq!(work.aggregate_cache_hits(), 2);
            assert_eq!(work.aggregate_rebuild_input_rows(), 0);
            assert_eq!(warm_work.get_or_insert(work), &work);
        }
    }
    let mut live = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(BankLiveControls::current(request_scope(), 1, 2).unwrap())
        .expect("installed two-cause buffer should open");
    let first_commit = commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "buffered-live-deposit-one",
    );
    let second_commit = commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "buffered-live-deposit-two",
    );
    assert_eq!(
        first_commit.metadata().projection_work(),
        second_commit.metadata().projection_work()
    );

    let first_outcome = live.poll();
    let BankActivityLiveOutcome::Delivered(first) = first_outcome else {
        panic!("first exact activity cause must deliver: {first_outcome:?}")
    };
    assert_eq!(live.buffered_update_count(), 1);
    let second_outcome = live.poll();
    let BankActivityLiveOutcome::Delivered(second) = second_outcome else {
        panic!("second exact activity cause must deliver: {second_outcome:?}")
    };
    assert_eq!(live.buffered_update_count(), 0);
    assert!(first.commit_id() < second.commit_id());
    assert_eq!(
        first.activity().output().account_sequence().get() + 1,
        second.activity().output().account_sequence().get()
    );
    assert_eq!(first.activity().metadata().result_count(), 1);
    assert_eq!(second.activity().metadata().result_count(), 1);
    let first_work = first.activity().metadata().work();
    assert_eq!(first_work, second.activity().metadata().work());
    assert_eq!(first_work.equality_lookups(), 1);
    assert_eq!(first_work.reconstructive_scans(), 0);
}

#[test]
fn retained_commit_source_reports_exact_consumer_overflow() {
    let fixture = ordinary_read_world("live-source-overflow", 0);
    let owner = fixture.authenticate(OWNER);
    let mut live = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(&owner)
        .subscribe(live_controls())
        .expect("authorized activity lease should open");

    for ordinal in 0..65 {
        commit_authorization_toggle(&fixture, &owner, ordinal);
    }

    let BankActivityLiveOutcome::Overflow(overflow) = live.poll() else {
        panic!("a consumer older than the retained source must receive typed overflow");
    };
    assert_eq!(overflow.missed_commit_batches(), 1);
}

fn commit_authorization_toggle(
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
    assert!(
        matches!(outcome.status(), BankMutationStatus::Committed(_)),
        "deposit did not commit: {:?}",
        outcome.status()
    );
}

fn authorized_user_id(
    fixture: &fixture::OrdinaryReadFixture,
    owner: &bank_server::BankAuthenticatedPrincipal,
    principal: usize,
) -> bank_domain::model::AccountAuthorizationId {
    let ReadOutcome::Delivered(users) = fixture
        .world
        .runtime
        .query(queries::account_authorized_users(fixture.personal_account))
        .as_principal(owner)
        .controls(BankReadControls::current(request_scope(), 16).unwrap())
        .execute()
    else {
        panic!("owner should read authorized users")
    };
    users
        .output()
        .iter()
        .find(|user| user.principal() == fixture::principal_id(principal))
        .expect("fixture authorization should exist")
        .authorization()
}

fn activity_count(
    fixture: &fixture::OrdinaryReadFixture,
    owner: &bank_server::BankAuthenticatedPrincipal,
) -> usize {
    let ReadOutcome::Delivered(activity) = fixture
        .world
        .runtime
        .query(queries::account_activity(fixture.personal_account))
        .as_principal(owner)
        .controls(BankReadControls::current(request_scope(), 16).unwrap())
        .execute()
    else {
        panic!("owner should read activity")
    };
    activity.output().len()
}

fn commit_deposit(
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
    assert!(
        matches!(outcome.status(), BankMutationStatus::Committed(_)),
        "deposit did not commit: {:?}",
        outcome.status()
    );
    outcome
}

fn live_controls() -> BankLiveControls {
    BankLiveControls::current(request_scope(), 16, 2).unwrap()
}

fn mutation_controls(key: &str) -> BankMutationControls {
    BankMutationControls::new(
        request_scope(),
        BankIdempotencyKey::new(key).expect("test idempotency key should admit"),
    )
}
