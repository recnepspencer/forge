//! Mint and inspection counter honesty (R8.33).

use bank_external_rail::FaultScript;

use super::phase8_cross_gate::installed_notify_death_aftermath;
use super::phase8_cross_gate::world::{cross_gate_world, PATIENT};
use crate::support::request_scope;

#[test]
fn mint_and_inspect_leave_recovery_inspection_at_zero() {
    let world = cross_gate_world("r8-33-counters");
    world
        .transport
        .under(FaultScript::CommitThenLoseResponse, PATIENT);
    let receipt = world.commit_notification(89);
    let aftermath = installed_notify_death_aftermath(&world.fixture.world.runtime);
    let handle = world.open_recovery(&receipt);
    assert_eq!(
        handle.binding().installed_aftermath_identity(),
        aftermath.identity(),
        "mint must retain the commit's exact installed aftermath",
    );
    assert_eq!(
        handle.binding().installed_aftermath_operation_slot(),
        aftermath.operation_slot(),
        "aftermath identity and operation slot must travel together",
    );
    // Mint leaves recovery_inspection at exactly 0/0/0 (R8.33).
    let after_mint = handle.canonical_work().recovery_inspection();
    assert_eq!(after_mint.basis_preparations(), 0);
    assert_eq!(after_mint.digest_derivations(), 0);
    assert_eq!(after_mint.digest_text_materializations(), 0);
    // Provider inquiry — retained posture read performs no canonical work.
    let _provider = handle.binding().provider_posture();
    let after_inquiry = handle.canonical_work().recovery_inspection();
    assert_eq!(after_inquiry.basis_preparations(), 0);
    assert_eq!(after_inquiry.digest_derivations(), 0);
    assert_eq!(after_inquiry.digest_text_materializations(), 0);

    let specialist = world.fixture.authenticate_specialist();
    let action = world.specialist_action();
    let scope = request_scope();
    let view1 = world
        .fixture
        .world
        .runtime
        .inspect_commit_recovery(&handle, &specialist, action, &scope)
        .expect("inspect");
    assert_eq!(view1.recovery_inspection_work().basis_preparations(), 0);
    assert_eq!(view1.recovery_inspection_work().digest_derivations(), 0);
    assert_eq!(
        view1
            .recovery_inspection_work()
            .digest_text_materializations(),
        0
    );
    let view2 = world
        .fixture
        .world
        .runtime
        .inspect_commit_recovery(&handle, &specialist, action, &scope)
        .expect("inspect again");
    assert_eq!(view2.recovery_inspection_work().basis_preparations(), 0);
    assert_eq!(view2.recovery_inspection_work().digest_derivations(), 0);
    assert_eq!(
        view2
            .recovery_inspection_work()
            .digest_text_materializations(),
        0
    );
}
