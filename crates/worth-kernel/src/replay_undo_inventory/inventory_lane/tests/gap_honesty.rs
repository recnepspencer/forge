use super::super::closeout::current_replay_undo_inventory_report;
use super::super::declaration::ReplayUndoDeclaredSourceIdentity;
use super::super::declaration::ReplayUndoDeclaredSourceKind;
use super::super::lowering::{ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition};

#[test]
fn replay_undo_boundary_admission_row_is_migrated() {
    let closeout = current_replay_undo_inventory_report().expect("closeout");
    let row = closeout
        .require_source(
            ReplayUndoDeclaredSourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
        )
        .expect("replay/undo boundary row");
    assert_eq!(row.category(), ReplayUndoInventoryCategory::UndoScope);
    assert_eq!(row.disposition(), ReplayUndoInventoryDisposition::Migrate);
    assert_eq!(
        row.source_kind(),
        ReplayUndoDeclaredSourceKind::PublicFunction
    );
    assert!(row
        .source_path()
        .ends_with("replay_undo_boundary/boolean_split_boundary_admission.rs"));
    assert!(row.removal_trigger().is_none());
}

#[test]
fn full_declared_coverage_closes_after_ordinary_undo_lane_cutover() {
    let closeout = current_replay_undo_inventory_report().expect("closeout");
    closeout
        .require_full_declared_coverage()
        .expect("ordinary replay/undo inventory should no longer carry a query-gap lane");
    assert!(closeout.gap_rows().is_empty());
}
