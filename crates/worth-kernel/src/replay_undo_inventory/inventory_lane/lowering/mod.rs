mod inventory_category;
mod inventory_disposition;
mod inventory_owner;
mod inventory_row;
mod source_specific;

use crate::replay_undo_inventory::inventory_lane::declaration::ReplayUndoDeclaredSourceCatalog;

pub use inventory_category::ReplayUndoInventoryCategory;
pub use inventory_disposition::ReplayUndoInventoryDisposition;
pub use inventory_owner::ReplayUndoInventoryOwner;
pub use inventory_row::ReplayUndoInventoryReportRow;

pub fn lower_current_replay_undo_inventory(
    catalog: &ReplayUndoDeclaredSourceCatalog,
) -> Vec<ReplayUndoInventoryReportRow> {
    source_specific::lower_declared_sources(catalog)
}
