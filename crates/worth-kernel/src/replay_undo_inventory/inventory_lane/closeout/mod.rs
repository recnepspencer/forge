mod counters;
mod error;
mod gap_row;
mod inventory_closeout;
mod reconciliation;

use std::sync::OnceLock;

use crate::replay_undo_inventory::inventory_lane::declaration::current_replay_undo_declared_source_catalog;
use crate::replay_undo_inventory::inventory_lane::declaration::ReplayUndoDeclaredSourceCatalog;
use crate::replay_undo_inventory::inventory_lane::lowering::lower_current_replay_undo_inventory;
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryReportRow;

pub use counters::ReplayUndoInventoryCloseoutCounters;
pub use error::{ReplayUndoInventoryError, ReplayUndoInventoryErrorKind};
pub use gap_row::ReplayUndoInventoryGapRow;
pub use inventory_closeout::ReplayUndoInventoryCloseout;

pub fn current_replay_undo_inventory_report(
) -> Result<ReplayUndoInventoryCloseout, ReplayUndoInventoryError> {
    static CACHE: OnceLock<ReplayUndoInventoryCloseout> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let declared = current_replay_undo_declared_source_catalog();
    let lowered = lower_current_replay_undo_inventory(&declared);
    let closeout = close_current_replay_undo_inventory(declared, lowered)?;
    let _ = CACHE.set(closeout.clone());
    Ok(closeout)
}

pub fn close_current_replay_undo_inventory(
    declared: ReplayUndoDeclaredSourceCatalog,
    lowered: Vec<ReplayUndoInventoryReportRow>,
) -> Result<ReplayUndoInventoryCloseout, ReplayUndoInventoryError> {
    ReplayUndoInventoryCloseout::new(declared, lowered)
}
