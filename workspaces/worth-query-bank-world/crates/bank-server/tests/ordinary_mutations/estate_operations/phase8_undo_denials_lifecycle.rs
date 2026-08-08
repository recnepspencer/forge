//! R8.39 stale / already-consumed / conflicted undo denials at the Bank boundary.

use bank_domain::proposals::BankIdempotencyKey;
use bank_external_rail::FaultScript;
use bank_server::BankEstateProgressionDenial;
use worth_query_host::facade::provisional_aftermath::WorthQueryUndoDenialKind;

use super::disburse_estate::fixture::disbursement_world;
use super::phase8_cross_gate::world::{
    cross_gate_world, cross_gate_world_with_authorization_time, PATIENT,
};
use super::phase8_undo_denial_support::{
    commit_disbursement, commit_foreign_reversal, committed_journal_ids, graph_snapshot,
};
use crate::authorization_time::AuthorizationTimeController;
use crate::support::request_scope;

#[test]
fn stale_expired_handle_undo_denies_and_writes_nothing() {
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(2_000);
    let world = cross_gate_world_with_authorization_time(
        "undo-deny-stale-admit",
        Some(authorization_time.clone()),
    );
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(102);
    let handle = world.open_recovery(&receipt);
    let revision_before = world.estate_account_revision();
    authorization_time.advance_to_epoch_seconds(5_601);
    let evaluation = world
        .fixture
        .world
        .runtime
        .evaluate_commit_recovery_expiry(&handle)
        .expect("expiry");
    assert!(matches!(
        evaluation,
        worth_query_host::facade::primary_graph::WorthQueryRecoveryExpiryEvaluation::Expired(_)
    ));
    let specialist = world.fixture.authenticate_specialist();
    let denied = world
        .fixture
        .world
        .runtime
        .admit_undo_commit_recovery(handle, &specialist, &request_scope())
        .expect_err("expired handle must deny undo as Stale");
    match denied {
        BankEstateProgressionDenial::Undo(d) => {
            assert_eq!(d.kind(), WorthQueryUndoDenialKind::Stale)
        }
        other => panic!("expected Undo(Stale), got {other:?}"),
    }
    assert_eq!(world.estate_account_revision(), revision_before);
    let twin = cross_gate_world("undo-deny-stale-twin");
    twin.transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = twin.commit_notification(103);
    let handle = twin.open_recovery(&receipt);
    let specialist = twin.fixture.authenticate_specialist();
    let _ = twin
        .fixture
        .world
        .runtime
        .admit_undo_commit_recovery(handle, &specialist, &request_scope())
        .expect("unexpired positive twin admits");
}

#[test]
fn conflicted_reverse_journal_undo_denies_and_writes_nothing() {
    let fixture = disbursement_world("undo-deny-conflicted", 800);
    let (specialist, receipt, original) = commit_disbursement(&fixture, 51);
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("admit");
    commit_foreign_reversal(
        &fixture,
        &specialist,
        original,
        "undo-conflict-prior-reversal",
    );
    let before = graph_snapshot(&fixture);
    let denied = fixture
        .world
        .runtime
        .progress_undo_commit_recovery(
            admission,
            &specialist,
            &BankIdempotencyKey::new("undo-conflicted-once").unwrap(),
            &request_scope(),
        )
        .expect_err("foreign journal must conflict");
    match denied {
        BankEstateProgressionDenial::Undo(d) => {
            assert_eq!(d.kind(), WorthQueryUndoDenialKind::Conflicted)
        }
        other => panic!("expected Undo(Conflicted), got {other:?}"),
    }
    assert_eq!(
        graph_snapshot(&fixture),
        before,
        "conflicted undo must add no second reversal"
    );
    assert_eq!(
        committed_journal_ids(&fixture).len(),
        2,
        "the denied continuation cannot append a second reversal"
    );
}

/// An undo denied *after* it left Query must not spend the commit's one recovery.
///
/// `progress_undo_commit_recovery` runs entirely in the application host, and
/// every `?` on that path drops the in-flight handoff. A bare handle records
/// `Disposed` on drop — right for a bare handle, wrong for a preparation that
/// performed nothing — so before Q8.22-C5 one Bank-side conflict destroyed the
/// commit's recoverability permanently, for an undo that never ran.
///
/// The host cannot be asked to cooperate here: the relinquish policy is
/// `pub(crate)` in Query and the host has never heard of it. The guarantee has
/// to hold through `Drop`, which is why this test denies through the *production*
/// Bank entry point rather than a Query-internal path.
///
/// Re-minting alone would only show a freed slot, so this also re-admits — a
/// usable capability is the claim (Q8.22-C5).
#[test]
fn bank_side_undo_denial_leaves_the_commit_recoverable() {
    let fixture = disbursement_world("undo-denial-keeps-recovery", 800);
    let (specialist, receipt, original) = commit_disbursement(&fixture, 71);
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("admit");
    commit_foreign_reversal(&fixture, &specialist, original, "undo-keep-prior-reversal");
    let denied = fixture
        .world
        .runtime
        .progress_undo_commit_recovery(
            admission,
            &specialist,
            &BankIdempotencyKey::new("undo-keep-once").unwrap(),
            &request_scope(),
        )
        .expect_err("foreign journal must conflict");
    match denied {
        BankEstateProgressionDenial::Undo(d) => {
            assert_eq!(d.kind(), WorthQueryUndoDenialKind::Conflicted)
        }
        other => panic!("expected Undo(Conflicted), got {other:?}"),
    }
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("a Bank-side denial must leave the commit recoverable");
    fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("the returned capability re-admits, not merely re-mints");
}
