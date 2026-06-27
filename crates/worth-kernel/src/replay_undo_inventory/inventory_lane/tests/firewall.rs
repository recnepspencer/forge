use super::super::declaration::{ReplayUndoDeclaredInputRole, ReplayUndoDeclaredSourceIdentity};
use super::super::firewall::current_replay_undo_source_firewall_report;

#[test]
fn typed_declared_role_is_admitted() {
    let firewall = current_replay_undo_source_firewall_report();
    firewall
        .require_declared_receipt_role(
            ReplayUndoDeclaredSourceIdentity::KernelLookupConsumedWorkloadComposition,
            ReplayUndoDeclaredInputRole::LookupConsumedWorkloadHandoff,
        )
        .expect("declared role");
}

#[test]
fn undeclared_role_is_rejected() {
    let firewall = current_replay_undo_source_firewall_report();
    let violation = firewall
        .require_declared_receipt_role(
            ReplayUndoDeclaredSourceIdentity::KernelLookupConsumedWorkloadComposition,
            ReplayUndoDeclaredInputRole::RetainedReplayWorkloadReceipt,
        )
        .expect_err("undeclared role must fail");
    assert_eq!(
        violation.source_identity(),
        ReplayUndoDeclaredSourceIdentity::KernelLookupConsumedWorkloadComposition
    );
}
