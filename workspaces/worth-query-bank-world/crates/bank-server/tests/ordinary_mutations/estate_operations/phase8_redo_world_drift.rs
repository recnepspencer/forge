//! Gate 8.5 A9 / X2 / X3 — world-drift redo denials through the Bank path.
//!
//! Drift the world after a proved undo; leave the intent completely honest;
//! prove the exact typed cause. Positive twins admit without the drift.

use bank_server::BankEstateProgressionDenial;
use worth_query_host::facade::provisional_aftermath::WorthQueryRedoDenialKind;

use super::disburse_estate::fixture::disbursement_world_with_clock_and_grant_validity;
use super::phase8_redo_support::{commit_and_prove_undo, graph_snapshot};
use crate::authorization_time::AuthorizationTimeController;
use crate::support::request_scope;

#[test]
fn newly_unauthorized_after_grant_expiry_with_honest_intent() {
    // R8.43 / A9 — grant expires after proved undo; intent untouched; redo
    // denies NewlyUnauthorized because the world changed, not the intent.
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(500);
    let fixture = disbursement_world_with_clock_and_grant_validity(
        "redo-world-drift-auth",
        1_000,
        Some(authorization_time.clone()),
        Some(600),
    );
    let proved = commit_and_prove_undo(&fixture, 61);
    let intent_before = proved.intent.clone();
    let before = graph_snapshot(&fixture);
    authorization_time.advance_to_epoch_seconds(601);
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request_scope(),
            &proved.intent,
        )
        .expect_err("grant expiry must deny redo as NewlyUnauthorized");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert_eq!(d.kind(), WorthQueryRedoDenialKind::NewlyUnauthorized);
        }
        other => panic!("expected Redo(NewlyUnauthorized), got {other:?}"),
    }
    assert_eq!(proved.intent, intent_before);
    assert_eq!(graph_snapshot(&fixture), before);

    let twin_time = AuthorizationTimeController::at_epoch_seconds(500);
    let twin = disbursement_world_with_clock_and_grant_validity(
        "redo-world-drift-auth-twin",
        1_000,
        Some(twin_time),
        Some(600),
    );
    let twin_proved = commit_and_prove_undo(&twin, 62);
    twin.world
        .runtime
        .admit_redo_disbursement_recovery(
            twin_proved.recovery,
            &twin_proved.specialist,
            &request_scope(),
            &twin_proved.intent,
        )
        .expect("unexpired grant positive twin admits");
}

#[test]
fn stale_after_handle_expiry_with_honest_intent() {
    // X2 — recovery handle clock-expires after proved undo; intent untouched;
    // redo denies Stale at the Bank boundary.
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(2_000);
    let fixture = disbursement_world_with_clock_and_grant_validity(
        "redo-world-drift-stale",
        1_000,
        Some(authorization_time.clone()),
        None,
    );
    let proved = commit_and_prove_undo(&fixture, 63);
    let intent_before = proved.intent.clone();
    let before = graph_snapshot(&fixture);
    authorization_time.advance_to_epoch_seconds(5_601);
    let evaluation = fixture
        .world
        .runtime
        .evaluate_commit_recovery_expiry(proved.recovery.handle())
        .expect("expiry evaluation");
    assert!(matches!(
        evaluation,
        worth_query_host::facade::primary_graph::WorthQueryRecoveryExpiryEvaluation::Expired(_)
    ));
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request_scope(),
            &proved.intent,
        )
        .expect_err("expired handle must deny redo as Stale");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert_eq!(d.kind(), WorthQueryRedoDenialKind::Stale);
        }
        other => panic!("expected Redo(Stale), got {other:?}"),
    }
    assert_eq!(proved.intent, intent_before);
    assert_eq!(graph_snapshot(&fixture), before);

    let twin_time = AuthorizationTimeController::at_epoch_seconds(2_000);
    let twin = disbursement_world_with_clock_and_grant_validity(
        "redo-world-drift-stale-twin",
        1_000,
        Some(twin_time),
        None,
    );
    let twin_proved = commit_and_prove_undo(&twin, 64);
    twin.world
        .runtime
        .admit_redo_disbursement_recovery(
            twin_proved.recovery,
            &twin_proved.specialist,
            &request_scope(),
            &twin_proved.intent,
        )
        .expect("unexpired handle positive twin admits");
}
