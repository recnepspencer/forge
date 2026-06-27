use super::super::closeout::current_replay_undo_inventory_report;
use super::super::declaration::ReplayUndoDeclaredSourceIdentity;
use super::super::lowering::{ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition};

#[test]
fn query_gap_row_stays_visible_in_closeout() {
    let closeout = current_replay_undo_inventory_report().expect("closeout");
    let row = closeout
        .require_source(ReplayUndoDeclaredSourceIdentity::KernelUndoOrdinaryLaneGap)
        .expect("gap row");
    assert_eq!(row.category(), ReplayUndoInventoryCategory::UndoScope);
    assert_eq!(row.disposition(), ReplayUndoInventoryDisposition::QueryGap);
    assert_eq!(closeout.gap_rows().len(), 1);
    assert_eq!(
        closeout.gap_rows()[0].removal_trigger(),
        "milestone12.undo_family_lane"
    );
}

#[test]
fn full_declared_coverage_stays_open_while_gap_exists() {
    let closeout = current_replay_undo_inventory_report().expect("closeout");
    let error = closeout
        .require_full_declared_coverage()
        .expect_err("gap must keep phase open");
    assert_eq!(
        error.kind(),
        super::super::closeout::ReplayUndoInventoryErrorKind::UnclassifiedSource
    );
}
