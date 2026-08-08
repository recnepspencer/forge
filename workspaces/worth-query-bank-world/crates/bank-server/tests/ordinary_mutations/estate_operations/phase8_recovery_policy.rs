//! Fresh admission policy distinct from binding-axis drift (Gate 8.3 turn 3).
//!
//! `ForeignPrincipal` is proved per-axis in `worth-query-execution`
//! `binding_axis_tests` — the default notify-death fixture only authorizes one
//! specialist principal on recovery re-admission.

use bank_external_rail::FaultScript;
use bank_server::BankEstateProgressionDenial;
use worth_query_host::facade::primary_graph::WorthQueryRecoveryHandleDenialKind;

use super::phase8_cross_gate::world::{cross_gate_world_with_clock_and_grant_validity, PATIENT};
use crate::authorization_time::AuthorizationTimeController;
use crate::support::request_scope;

#[test]
fn expired_grant_after_mint_denies_fresh_admission_not_foreign_principal() {
    let authorization_time = AuthorizationTimeController::at_epoch_seconds(300);
    let world = cross_gate_world_with_clock_and_grant_validity(
        "grant-expired-after-mint",
        Some(authorization_time.clone()),
        Some(400),
    );
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(84);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    authorization_time.advance_to_epoch_seconds(401);
    let denied = world
        .fixture
        .world
        .runtime
        .admit_commit_recovery_effect(&handle, &specialist, action, &scope)
        .expect_err("expired grant must fail fresh admission");
    match denied {
        BankEstateProgressionDenial::Recovery(d) => {
            assert_ne!(
                d.kind(),
                WorthQueryRecoveryHandleDenialKind::ForeignPrincipal
            );
            assert_eq!(
                d.kind(),
                WorthQueryRecoveryHandleDenialKind::CurrentPolicyDenied
            );
        }
        BankEstateProgressionDenial::Authorization(_) => {}
        other => panic!("expected current-policy or authorization denial, got {other:?}"),
    }
}
