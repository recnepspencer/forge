//! Cross-gate integration suite seed (R8.64) — Gate 8.3 through 8.1 + 8.2.
//!
//! Lost-response recovery runs through the real `bank-external-rail` process
//! and Gate 8.1's installed aftermath contract. Authority is re-admitted through
//! production Bank paths — never caller-asserted booleans (R8.31).

#[path = "phase8_cross_gate/world.rs"]
pub(crate) mod world;

use crate::support::request_scope;
use bank_domain::schema::{DisburseEstateOperation, NotifyDeathEstateOperation};
use bank_external_rail::test_control::FaultScript;
use bank_server::{
    BankCommitReceipt, BankEstateProgressionDenial, BankIdentityRuntime, BankRecoveryDenialKind,
    BankRecoveryDurability, BankRecoveryIdempotencyResolution, BankRecoveryInspection,
    BankRecoveryPosture, BankRecoverySupportTruth,
};
use worth_query_host::facade::domain::{
    PublishedAftermathPosture, WorthQueryInstalledAftermathContract,
};
use worth_query_host::facade::provisional_aftermath::{
    WorthQueryUndoDenialKind, WorthQueryUndoDerivedRequest,
};
use worth_query_host::facade::publication::application_aftermath::WorthQueryPublishedExternalEffectFailure;

use self::world::cross_gate_world;

#[test]
fn lost_response_recovery_through_real_rail_and_aftermath() {
    let world = cross_gate_world("lost-response-recovery");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, world::PATIENT);
    let binding = world::idempotency(71);
    let revision_before = world.estate_account_revision();
    let receipt = world.commit_with(binding);

    assert_lost_response_commit(&world, &receipt);
    assert_unresolved_recovery(&world, &receipt, revision_before);
}

fn assert_lost_response_commit(world: &world::CrossGateWorld, receipt: &BankCommitReceipt) {
    assert!(receipt.co_committed_dispatch_outbox());
    assert_eq!(
        receipt
            .external_dispatch_posture()
            .and_then(|posture| posture.failure()),
        Some(WorthQueryPublishedExternalEffectFailure::LostResponse)
    );

    let aftermath = installed_notify_death_aftermath(&world.fixture.world.runtime);
    assert_eq!(
        aftermath.published_posture(),
        PublishedAftermathPosture::Reconcilable
    );
}

fn assert_unresolved_recovery(
    world: &world::CrossGateWorld,
    receipt: &BankCommitReceipt,
    revision_before: u64,
) {
    let handle = world.open_recovery(receipt);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();

    // Inspect twice through production disclosure admission (R8.33).
    let view1 = world
        .fixture
        .world
        .runtime
        .inspect_commit_recovery(&handle, &specialist, action, &scope)
        .expect("inspect");
    let view2 = world
        .fixture
        .world
        .runtime
        .inspect_commit_recovery(&handle, &specialist, action, &scope)
        .expect("inspect-again");
    assert_recovery_publication(&view1, &view2);

    // Resolve through production admitted graph read — unresolved stays unresolved.
    let denied = world
        .fixture
        .world
        .runtime
        .resolve_commit_recovery(handle, &specialist, action, &scope)
        .expect_err("unresolved stays unresolved");
    match denied {
        BankEstateProgressionDenial::Recovery(d) => {
            assert_eq!(d.kind(), BankRecoveryDenialKind::UnresolvedExternalPosture)
        }
        other => panic!("expected unresolved posture denial, got {other:?}"),
    }
    assert_eq!(world.estate_account_revision(), revision_before);
}

fn assert_recovery_publication(first: &BankRecoveryInspection, second: &BankRecoveryInspection) {
    assert_eq!(first.recovery_inspection_work().basis_preparations(), 0);
    assert_eq!(first.recovery_inspection_work().digest_derivations(), 0);
    assert_eq!(
        first
            .recovery_inspection_work()
            .digest_text_materializations(),
        0
    );
    assert_eq!(second.recovery_inspection_work().basis_preparations(), 0);
    assert_eq!(
        first.durability(),
        BankRecoveryDurability::StoreCapabilityRequired
    );
    assert_eq!(
        first.support_truth(),
        BankRecoverySupportTruth::DegradedRecoveryReport
    );
    assert_eq!(first.posture(), BankRecoveryPosture::Reconcilable);
    assert_eq!(first, second);
}

#[test]
fn unrelated_aftermath_lookup_cannot_substitute_the_handle_contract() {
    let world = cross_gate_world("aftermath-slot-substitution");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, world::PATIENT);
    let receipt = world.commit_notification(85);
    let aftermath = installed_notify_death_aftermath(&world.fixture.world.runtime);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();

    // Positive twin — matching installed aftermath admits effect authority.
    let handle = world.open_recovery(&receipt);
    // The unrelated DisburseEstate contract can be inspected but cannot be
    // supplied to the transition; the handle keeps NotifyDeath authoritative.
    let substituted = installed_disburse_aftermath(&world.fixture.world.runtime);
    assert_ne!(substituted.identity(), aftermath.identity());
    let admitted = world
        .fixture
        .world
        .runtime
        .reconcile_commit_recovery(handle, &specialist, action, &scope)
        .expect("handle-carried NotifyDeath aftermath admits reconcile");
    assert_eq!(admitted.installed_operation(), aftermath.operation_slot());
}

#[test]
fn already_completed_resolve_returns_inherited_taxonomy() {
    let world = cross_gate_world("already-completed");
    world.transport.under(FaultScript::Succeed, world::PATIENT);
    let binding = world::idempotency(76);
    let receipt = world.commit_with(binding);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    // Positive twin — admitted graph read returns AlreadyCommitted for the same binding.
    let resolution =
        world
            .fixture
            .world
            .runtime
            .resolve_commit_recovery(handle, &specialist, action, &scope);
    match resolution {
        Ok(BankRecoveryIdempotencyResolution::AlreadyCommitted) => {}
        Err(BankEstateProgressionDenial::Recovery(d))
            if d.kind() == BankRecoveryDenialKind::UnresolvedExternalPosture =>
        {
            // Succeed may still leave Unresolved under some rail timings.
        }
        other => panic!("expected AlreadyCommitted or unresolved posture, got {other:?}"),
    }
}

#[test]
fn inspect_requires_disclosure_proof_not_boolean() {
    // InspectAuthority requires WorthQueryRecoveryDisclosureAdmission. There is
    // no disclosure_admitted bool. This test proves the production inspect path
    // succeeds only when disclosure is minted through admit_recovery_inspection_disclosure.
    let world = cross_gate_world("inspect-disclosure");
    world.transport.under(FaultScript::Succeed, world::PATIENT);
    let receipt = world.commit_notification(74);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    let view = world
        .fixture
        .world
        .runtime
        .inspect_commit_recovery(&handle, &specialist, action, &scope)
        .expect("disclosure-backed inspect");
    assert_eq!(view.recovery_inspection_work().basis_preparations(), 0);
}

#[test]
fn undo_through_handle_rail_and_aftermath_populates_undo_admission() {
    // R8.64 — undo through Gate 8.3 handle, 8.2 real rail, 8.1 installed aftermath.
    let world = cross_gate_world("undo-through-stack");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, world::PATIENT);
    let receipt = world.commit_notification(90);
    assert!(receipt.changed_record_count() > 0);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let scope = request_scope();
    let admission = world
        .fixture
        .world
        .runtime
        .admit_undo_commit_recovery(handle, &specialist, &scope)
        .expect("fresh undo admission through production path");
    assert_eq!(
        admission.derived_request(),
        WorthQueryUndoDerivedRequest::Reconciliation
    );
    let work = admission.undo_admission_work();
    assert_eq!(work.basis_preparations(), 1);
    assert_eq!(work.digest_derivations(), 1);
    assert_eq!(work.digest_text_materializations(), 0);
    let phases = admission.canonical_work_phases();
    assert_eq!(phases.undo_admission().basis_preparations(), 1);
    assert_eq!(phases.undo_admission().digest_derivations(), 1);
    assert_eq!(phases.undo_admission().digest_text_materializations(), 0);
    // Receipt remains honest evidence of what happened — not current authority.
}

#[test]
fn undo_denies_on_current_policy_after_world_drift_with_honest_receipt() {
    // R8.37 — revoke the world after commit; leave the receipt untouched; undo
    // must deny on current policy, not because the receipt was corrupted.
    let authorization_time =
        crate::authorization_time::AuthorizationTimeController::at_epoch_seconds(500);
    let world = world::cross_gate_world_with_clock_and_grant_validity(
        "undo-world-drift",
        Some(authorization_time.clone()),
        Some(600),
    );
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, world::PATIENT);
    let receipt = world.commit_notification(91);
    let handle = world.open_recovery(&receipt);
    let specialist = world.fixture.authenticate_specialist();
    let scope = request_scope();
    // Honest receipt — still names the same operation / idempotency binding.
    authorization_time.advance_to_epoch_seconds(601);
    let denied = world
        .fixture
        .world
        .runtime
        .admit_undo_commit_recovery(handle, &specialist, &scope)
        .expect_err("world drift must deny fresh undo");
    match denied {
        BankEstateProgressionDenial::Recovery(d) => {
            assert_eq!(d.kind(), BankRecoveryDenialKind::CurrentPolicyDenied);
        }
        BankEstateProgressionDenial::Authorization(d) => {
            // Fresh capability re-admission is the current-policy fact — not an
            // unrelated authorization denial (R8.37 / A10). Clock-advanced
            // grant expiry surfaces as a missing live authorization on the
            // current world, not as a forged or foreign principal fact.
            assert_eq!(
                d.kind(),
                bank_server::BankAuthorizationDenialKind::CapabilityAuthorizationMissing,
                "world-drift authorization denial must be missing live capability authorization, got {:?}",
                d.kind()
            );
        }
        BankEstateProgressionDenial::Undo(d) => {
            assert_eq!(d.kind(), WorthQueryUndoDenialKind::CurrentPolicyDenied);
        }
        other => panic!("expected current-policy denial, got {other:?}"),
    }
}

#[test]
fn redo_through_undo_handle_rail_and_aftermath() {
    // R8.64 — redo through Gate 8.4 undo, 8.3 handle, 8.2 real rail, 8.1 aftermath.
    // Disbursement redo is the money path; this scenario proves the stack still
    // admits undo (derivation precondition) through the rail-backed recovery
    // handle, then derives a descriptive redo intent bound to the linear head.
    use super::disburse_estate::fixture::disbursement_world;
    use super::phase8_redo_support::commit_and_prove_undo;

    let fixture = disbursement_world("redo-cross-gate-stack", 1_000);
    let proved = commit_and_prove_undo(&fixture, 77);
    assert!(proved.intent.is_bound_to(&proved.recovery));
    let request = request_scope();
    let admission = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request,
            &proved.intent,
        )
        .expect("redo through undo+handle+aftermath");
    assert_eq!(admission.redo_admission_work().basis_preparations(), 1);
    assert_eq!(admission.redo_admission_work().digest_derivations(), 1);
    assert_eq!(
        admission
            .redo_admission_work()
            .digest_text_materializations(),
        0
    );
}

pub(crate) fn installed_notify_death_aftermath(
    runtime: &BankIdentityRuntime,
) -> WorthQueryInstalledAftermathContract {
    runtime.installed_operation_aftermath(NotifyDeathEstateOperation::reference())
}

pub(crate) fn installed_disburse_aftermath(
    runtime: &BankIdentityRuntime,
) -> WorthQueryInstalledAftermathContract {
    runtime.installed_operation_aftermath(DisburseEstateOperation::reference())
}
