//! Same-runtime recovery authority remains affine to one exact handle (Q8.20).

use bank_external_rail::test_control::FaultScript;
use worth_query_host::facade::primary_graph::{
    expire_recovery_handle, inspect_recovery_handle, reconcile_recovery_handle,
    WorthQueryRecoveryExpiryEvaluation, WorthQueryRecoveryHandleDenialKind,
};

use super::phase8_cross_gate::world::{
    cross_gate_world, cross_gate_world_with_authorization_time, PATIENT,
};
use crate::authorization_time::AuthorizationTimeController;
use crate::support::request_scope;

#[test]
fn cloned_receipt_cannot_mint_a_second_recovery_handle() {
    let world = cross_gate_world("same-runtime-receipt-mint-claim");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(120);
    let copied_receipt = receipt.clone();
    let handle = world.open_recovery(&receipt);

    let denied = world
        .fixture
        .world
        .runtime
        .open_commit_recovery(&copied_receipt)
        .expect_err("one authoritative commit can open only one handle");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::RecoveryAlreadyMinted
    );
    drop(handle);
}

// R8.28 — a commit receipt is public, and a process may publish more than one
// application runtime. Mint used to derive everything from the receipt and check
// nothing about *who* was minting, so a second runtime could open a handle for a
// commit it never admitted. `provider_runtime_instance_id` cannot catch this: it
// names the relational instance, which every Query runtime over the same source
// shares. The receipt carries the admitting Query runtime's authority instead,
// derived at commit from the admitted operation.
#[test]
fn a_receipt_committed_by_another_runtime_cannot_mint_a_handle_here() {
    let committing = cross_gate_world("cross-runtime-receipt-mint-committing");
    let bystander = cross_gate_world("cross-runtime-receipt-mint-bystander");
    committing
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = committing.commit_notification(127);

    let denied = bystander
        .fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect_err("a runtime cannot open recovery for a commit it never admitted");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::ForeignRuntime
    );

    // The refusal is about provenance, not about the receipt being spent: the
    // runtime that actually committed still mints from the very same receipt.
    let handle = committing.open_recovery(&receipt);
    drop(handle);
}

#[test]
fn effect_authority_for_handle_a_cannot_transition_handle_b_in_the_same_runtime() {
    let world = cross_gate_world("same-runtime-effect-handle-affinity");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt_a = world.commit_notification(121);
    let receipt_b = world.commit_second_notification(122);
    let handle_a = world.open_recovery(&receipt_a);
    let handle_b = world.open_recovery(&receipt_b);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let authority_a = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(&handle_a, &specialist, action, &request_scope())
        .expect("handle A authority");

    let denied = reconcile_recovery_handle(handle_b, &authority_a)
        .expect_err("authority A cannot transition handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );
    reconcile_recovery_handle(handle_a, &authority_a).expect("exact handle A admits");
}

// Q8.21-L11 — the companion the affinity test above could never catch. That
// test asserts only that handle B was *refused*, then checks the registry has
// no live handles — which passed just as well when the refusal silently
// destroyed B. Transitions take the handle by value, so every `?` on a denial
// path dropped it; `Drop` recorded `Disposed`; and the commit's authoritative
// mint claim was never released. One attempt with the wrong authority
// permanently spent a recovery that had not been exercised at all.
//
// This performs the retry rather than asserting a terminal label: after the
// denial, B's receipt must still mint, and that fresh handle must still
// complete the transition the denied attempt never made.
#[test]
fn a_denied_transition_does_not_spend_the_commits_one_recovery() {
    let world = cross_gate_world("denied-transition-relinquishes");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt_a = world.commit_notification(128);
    let receipt_b = world.commit_second_notification(129);
    let handle_a = world.open_recovery(&receipt_a);
    let handle_b = world.open_recovery(&receipt_b);
    let specialist = world.fixture.authenticate_specialist();
    let authority_a = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(
            &handle_a,
            &specialist,
            world.specialist_action(),
            &request_scope(),
        )
        .expect("handle A authority");

    let denied = reconcile_recovery_handle(handle_b, &authority_a)
        .expect_err("authority A cannot transition handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    // Nothing about B was consumed, so B's commit is exactly as recoverable as
    // it was a moment earlier. Minting is the removal check: with the claim
    // retained this returns `RecoveryAlreadyMinted` and the test fails here.
    let handle_b = world
        .fixture
        .world
        .runtime
        .open_commit_recovery(&receipt_b)
        .expect("a denied attempt must not spend the commit's one recovery");
    let authority_b = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(
            &handle_b,
            &specialist,
            world.second_specialist_action(),
            &request_scope(),
        )
        .expect("handle B authority");
    reconcile_recovery_handle(handle_b, &authority_b)
        .expect("the corrected attempt still has a recovery to transition");

    reconcile_recovery_handle(handle_a, &authority_a).expect("exact handle A admits");
}

// The other half of Q8.21-L11, and the reason relinquishment is a distinct
// terminal rather than a blanket claim release: a recovery that really was
// exercised stays spent forever. Without this, `relinquish` could be widened
// into `mark_terminal` and the one-shot guarantee would silently become
// "one-shot per attempt".
#[test]
fn a_completed_transition_spends_the_commits_one_recovery_permanently() {
    let world = cross_gate_world("completed-transition-spends-recovery");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(130);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let authority = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(
            &handle,
            &specialist,
            world.specialist_action(),
            &request_scope(),
        )
        .expect("handle authority");
    reconcile_recovery_handle(handle, &authority).expect("exact handle admits");

    let denied = world
        .fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect_err("an exercised recovery cannot be reopened");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::RecoveryAlreadyMinted
    );
}

#[test]
fn inspect_authority_for_handle_a_cannot_inspect_handle_b_in_the_same_runtime() {
    let world = cross_gate_world("same-runtime-inspect-handle-affinity");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt_a = world.commit_notification(123);
    let receipt_b = world.commit_second_notification(124);
    let handle_a = world.open_recovery(&receipt_a);
    let handle_b = world.open_recovery(&receipt_b);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let authority_a = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_inspect(&handle_a, &specialist, action, &request_scope())
        .expect("handle A inspect authority");

    let denied = inspect_recovery_handle(&handle_b, &authority_a)
        .expect_err("authority A cannot inspect handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );
    inspect_recovery_handle(&handle_a, &authority_a).expect("exact handle A admits");
    drop(handle_a);
    drop(handle_b);
}

#[test]
fn expired_evidence_for_handle_a_cannot_expire_handle_b_in_the_same_runtime() {
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(2_000);
    let world = cross_gate_world_with_authorization_time(
        "same-runtime-expiry-handle-affinity",
        Some(authorization_time.clone()),
    );
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt_a = world.commit_notification(125);
    let receipt_b = world.commit_second_notification(126);
    let handle_a = world.open_recovery(&receipt_a);
    let handle_b = world.open_recovery(&receipt_b);
    authorization_time.advance_to_epoch_seconds(5_601);
    let evaluation = world
        .fixture
        .world
        .runtime
        .evaluate_commit_recovery_expiry(&handle_a)
        .expect("expiry evaluation");
    let WorthQueryRecoveryExpiryEvaluation::Expired(expired_a) = evaluation else {
        panic!("advanced clock must expire handle A");
    };

    let denied = expire_recovery_handle(handle_b, &expired_a)
        .expect_err("handle A expiry evidence cannot terminate handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );
    expire_recovery_handle(handle_a, &expired_a).expect("exact handle A expires");
}
