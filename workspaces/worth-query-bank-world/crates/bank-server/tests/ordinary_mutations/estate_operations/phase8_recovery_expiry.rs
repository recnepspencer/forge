//! Recovery expiry through the runtime clock (R8.7 M2/M3 / R8.29).

use bank_external_rail::FaultScript;
use bank_server::BankEstateProgressionDenial;
use worth_query_host::facade::primary_graph::{
    expire_recovery_handle, WorthQueryRecoveryExpiryEvaluation, WorthQueryRecoveryHandleDenialKind,
};

use super::phase8_cross_gate::world::{
    cross_gate_world, cross_gate_world_with_authorization_time, PATIENT,
};
use crate::authorization_time::AuthorizationTimeController;
use crate::support::request_scope;

#[test]
fn current_expiry_evidence_records_the_runtime_sample() {
    let world = cross_gate_world("expiry-lifecycle");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(77);
    let handle = world.open_recovery(&receipt);
    // M2/M3 — evaluation samples the runtime clock; callers cannot supply a sample.
    let evaluation = world
        .fixture
        .world
        .runtime
        .evaluate_commit_recovery_expiry(&handle)
        .expect("expiry evaluation");
    let WorthQueryRecoveryExpiryEvaluation::Current(current) = evaluation else {
        panic!("unexpired handle must produce current evidence");
    };
    let _ = current.sample();
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    let _ = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(&handle, &specialist, action, &scope)
        .expect("unexpired effect authority");
    drop(handle);
}

#[test]
fn clock_advanced_expiry_terminalizes_and_denies_fresh_admission() {
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(2_000);
    let world = cross_gate_world_with_authorization_time(
        "expiry-terminal",
        Some(authorization_time.clone()),
    );
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(78);
    let handle = world.open_recovery(&receipt);
    authorization_time.advance_to_epoch_seconds(5_601);
    let evaluation = world
        .fixture
        .world
        .runtime
        .evaluate_commit_recovery_expiry(&handle)
        .expect("expiry evaluation");
    let WorthQueryRecoveryExpiryEvaluation::Expired(decision) = evaluation else {
        panic!("advanced clock must produce expired evidence");
    };
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    let denied = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(&handle, &specialist, action, &scope)
        .expect_err("admit after expiry denies Expired");
    match denied {
        BankEstateProgressionDenial::Recovery(d) => {
            assert_eq!(d.kind(), WorthQueryRecoveryHandleDenialKind::Expired)
        }
        other => panic!("expected Expired, got {other:?}"),
    }
    let _binding = expire_recovery_handle(handle, &decision).expect("expire terminal");
    let denied = world
        .fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect_err("expired recovery remains terminal");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::RecoveryAlreadyMinted
    );
}
