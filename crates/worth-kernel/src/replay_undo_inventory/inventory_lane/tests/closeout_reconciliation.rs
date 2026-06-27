use super::super::closeout::close_current_replay_undo_inventory;
use super::super::declaration::{
    current_replay_undo_declared_source_catalog, ReplayUndoDeclaredSourceIdentity,
    ReplayUndoDeclaredSourceKind,
};
use super::super::lowering::{
    lower_current_replay_undo_inventory, ReplayUndoInventoryCategory,
    ReplayUndoInventoryDisposition, ReplayUndoInventoryOwner, ReplayUndoInventoryReportRow,
};

#[test]
fn declared_source_not_lowered_fails_closeout() {
    let declared = current_replay_undo_declared_source_catalog();
    let mut lowered = lower_current_replay_undo_inventory(&declared);
    lowered.retain(|row| {
        row.source_identity() != ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadDiagnostics
    });

    let error = close_current_replay_undo_inventory(declared, lowered)
        .expect_err("declared-but-not-lowered must fail");
    assert_eq!(
        error.kind(),
        super::super::closeout::ReplayUndoInventoryErrorKind::DeclaredSourceNotLowered
    );
}

#[test]
fn duplicate_lowered_source_fails_closeout() {
    let declared = current_replay_undo_declared_source_catalog();
    let mut lowered = lower_current_replay_undo_inventory(&declared);
    lowered.push(ReplayUndoInventoryReportRow::new(
        ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadRetainedReplay,
        "fake",
        ReplayUndoDeclaredSourceKind::PublicType,
        ReplayUndoInventoryOwner::WorthKernel,
        ReplayUndoInventoryCategory::TopologyReplayScope,
        ReplayUndoInventoryDisposition::Migrate,
        declared
            .require_source(ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadRetainedReplay)
            .expect("declared")
            .authority_roles()
            .clone(),
        declared
            .require_source(ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadRetainedReplay)
            .expect("declared")
            .observability_roles()
            .clone(),
        None,
    ));

    let error = close_current_replay_undo_inventory(declared, lowered)
        .expect_err("duplicate lowered source must fail");
    assert_eq!(
        error.kind(),
        super::super::closeout::ReplayUndoInventoryErrorKind::DuplicateLoweredSource
    );
}
