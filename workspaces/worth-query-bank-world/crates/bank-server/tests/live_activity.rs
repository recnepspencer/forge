#[allow(
    dead_code,
    reason = "the shared read fixture includes scenarios used by sibling consumer courtrooms"
)]
#[path = "ordinary_reads/fixture.rs"]
mod fixture;
#[path = "live_activity/support.rs"]
mod live_activity_support;
mod support;

use std::time::{Duration, Instant};

use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::RevokeAccountAuthorization;
use bank_server::{
    mutations, BankAccountActivityLiveOutcome, BankApplicationQueryDenial, BankMutationControls,
    BankMutationStatus,
};

use fixture::{ordinary_read_world, OWNER, TELLER, VIEWER};
use live_activity_support::{
    activity_count, assert_phase_posture, authorized_user_id, commit_authorization_toggle,
    commit_deposit, live_controls, live_controls_for,
};
use support::request_scope;
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationLiveControls, WorthQueryApplicationLiveOpenDenialKind,
};

#[test]
fn live_activity_delivers_only_matching_commits_as_fresh_reads() {
    let fixture = ordinary_read_world("live-activity", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let mut live = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls())
        .expect("authorized activity lease should open");
    assert!(matches!(
        live.poll(),
        BankAccountActivityLiveOutcome::Pending
    ));

    commit_deposit(
        &fixture,
        &teller,
        fixture.recipient_account,
        "unrelated-live-deposit",
    );
    assert!(matches!(
        live.poll(),
        BankAccountActivityLiveOutcome::Pending
    ));

    let before = activity_count(&fixture, &owner);
    commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "matching-live-deposit",
    );
    let outcome = live.poll();
    let BankAccountActivityLiveOutcome::Delivered(update) = outcome else {
        panic!("matching commit must deliver a fresh account projection");
    };
    let activity = update.result();
    assert_eq!(
        activity.entries()[0].account_sequence().get(),
        u64::try_from(before).unwrap() + 1
    );
    assert_eq!(activity.account(), fixture.personal_account);
    assert_eq!(activity.entries().len(), 1);
    assert_eq!(update.receipt().projected_record_count(), 3);
    assert!(update.commit_ordinal() > 0);
    assert_eq!(
        live.close(),
        worth_query_host::facade::primary_graph::WorthQueryApplicationLiveCloseOutcome::Completed
    );
}

#[test]
fn live_consumer_fanout_keeps_each_delivery_free_of_canonical_work() {
    const CONSUMER_COUNT: usize = 32;

    let fixture = ordinary_read_world("live-consumer-canonical-scale", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let mut leases = (0..CONSUMER_COUNT)
        .map(|_| {
            fixture
                .world
                .runtime
                .account_activity(fixture.personal_account)
                .as_principal(&owner)
                .subscribe(live_controls())
                .expect("each bounded authenticated live consumer should open")
        })
        .collect::<Vec<_>>();

    commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "live-consumer-canonical-scale-deposit",
    );

    let mut expected_phases = None;
    let mut delivered = 0usize;
    for lease in &mut leases {
        let BankAccountActivityLiveOutcome::Delivered(update) = lease.poll() else {
            panic!("each retained consumer must receive the matching commit")
        };
        delivered += 1;
        let phases = update.receipt().canonical_work();
        assert_phase_posture(phases);
        assert_eq!(update.receipt().authorization_work().requirement_count(), 1);
        if let Some(expected) = expected_phases {
            assert_eq!(phases, expected);
        } else {
            expected_phases = Some(phases);
        }
    }
    assert_eq!(delivered, CONSUMER_COUNT);
    for lease in leases {
        assert_eq!(
            lease.close(),
            worth_query_host::facade::primary_graph::WorthQueryApplicationLiveCloseOutcome::Completed
        );
    }
}

#[test]
fn permission_revocation_closes_before_another_payload_is_delivered() {
    let fixture = ordinary_read_world("live-revocation", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let viewer = fixture.authenticate(VIEWER);
    let authorization = authorized_user_id(&fixture, &owner, VIEWER);
    let mut live = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
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
    commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "revoked-viewer-live-deposit",
    );
    assert!(matches!(
        live.poll(),
        BankAccountActivityLiveOutcome::AuthorizationDenied(_)
    ));
    assert!(matches!(
        live.poll(),
        BankAccountActivityLiveOutcome::Closed
    ));
    assert_eq!(live.buffered_cause_count(), 0);
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
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls_for(request, 16))
        .expect("live lease should open before cancellation");
    cancellation.cancel();
    let cancelled_outcome = cancelled.poll();
    assert!(
        matches!(cancelled_outcome, BankAccountActivityLiveOutcome::Cancelled),
        "unexpected cancellation outcome"
    );
    assert!(matches!(
        cancelled.poll(),
        BankAccountActivityLiveOutcome::Closed
    ));

    let deadline = Instant::now() + Duration::from_secs(1);
    let deadline_source = WorthQueryCancellationSource::new();
    let request = WorthQueryRequestScope::new(deadline, deadline_source.token());
    let mut expired = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls_for(request, 16))
        .expect("live lease should open before its deadline");
    while Instant::now() < deadline {
        std::thread::yield_now();
    }
    let expired_outcome = expired.poll();
    assert!(
        matches!(
            expired_outcome,
            BankAccountActivityLiveOutcome::DeadlineExceeded
        ),
        "unexpected deadline outcome"
    );
    assert!(matches!(
        expired.poll(),
        BankAccountActivityLiveOutcome::Closed
    ));
}

#[test]
fn caller_cannot_enlarge_the_installed_live_buffer_ceiling() {
    let fixture = ordinary_read_world("live-buffer-ceiling", 0);
    let owner = fixture.authenticate(OWNER);
    let denial = match fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls_for(request_scope(), 65))
    {
        Err(denial) => denial,
        Ok(_) => panic!("caller buffer wider than installed queue authority must fail"),
    };
    assert!(matches!(
        denial,
        BankApplicationQueryDenial::LiveOpen(error)
            if error.kind()
                == WorthQueryApplicationLiveOpenDenialKind::BufferCapacityExceedsInstalled
    ));
}

#[test]
fn caller_cannot_enlarge_the_installed_live_work_ceiling() {
    let fixture = ordinary_read_world("live-work-ceiling", 0);
    let owner = fixture.authenticate(OWNER);
    let controls =
        WorthQueryApplicationLiveControls::bounded(request_scope(), 16, 8, 2_049).unwrap();
    let denial = match fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(controls)
    {
        Err(denial) => denial,
        Ok(_) => panic!("caller work wider than installed delivery authority must fail"),
    };
    assert!(matches!(
        denial,
        BankApplicationQueryDenial::LiveOpen(error)
            if error.kind() == WorthQueryApplicationLiveOpenDenialKind::WorkLimitExceedsInstalled
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
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls_for(request_scope(), 2))
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
    let BankAccountActivityLiveOutcome::Delivered(first) = first_outcome else {
        panic!("first exact activity cause must deliver")
    };
    assert_eq!(live.buffered_cause_count(), 1);
    let second_outcome = live.poll();
    let BankAccountActivityLiveOutcome::Delivered(second) = second_outcome else {
        panic!("second exact activity cause must deliver")
    };
    assert_eq!(live.buffered_cause_count(), 0);
    assert!(first.commit_id() < second.commit_id());
    assert_eq!(
        first.result().entries()[0].account_sequence().get() + 1,
        second.result().entries()[0].account_sequence().get()
    );
    assert_eq!(first.result().entries().len(), 1);
    assert_eq!(second.result().entries().len(), 1);
    assert_eq!(
        first.receipt().target_identity_index_entry_count(),
        second.receipt().target_identity_index_entry_count()
    );
    assert_eq!(first.receipt().target_identity_index_entry_count(), 1);
    assert_eq!(first.receipt().examined_candidate_count(), 2);
    assert_eq!(
        first.receipt().edge_scan_count(),
        second.receipt().edge_scan_count()
    );
    assert_eq!(first.receipt().edge_scan_count(), 2);
    assert_eq!(
        first.receipt().adjacency_list_read_count(),
        second.receipt().adjacency_list_read_count()
    );
    assert_eq!(first.receipt().per_result_neighbor_lookup_count(), 0);
    assert_eq!(first.receipt().fallback_count(), 0);
}

#[test]
fn retained_commit_source_reports_exact_consumer_overflow() {
    let fixture = ordinary_read_world("live-source-overflow", 0);
    let owner = fixture.authenticate(OWNER);
    let mut live = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .subscribe(live_controls())
        .expect("authorized activity lease should open");

    for ordinal in 0..65 {
        commit_authorization_toggle(&fixture, &owner, ordinal);
    }

    let BankAccountActivityLiveOutcome::Overflow(overflow) = live.poll() else {
        panic!("a consumer older than the retained source must receive typed overflow");
    };
    assert_eq!(overflow.missed_commit_batches(), 1);
}
