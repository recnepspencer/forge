mod lookup_handoff_lowering;
mod public_closeout_lowering;
mod replay_undo_boundary_lowering;
mod topo_invalidation_lowering;
mod workload_seed_lowering;

use crate::replay_undo_inventory::inventory_lane::declaration::ReplayUndoDeclaredSourceCatalog;

use super::inventory_row::ReplayUndoInventoryReportRow;

pub fn lower_declared_sources(
    catalog: &ReplayUndoDeclaredSourceCatalog,
) -> Vec<ReplayUndoInventoryReportRow> {
    let mut rows = Vec::new();
    rows.extend(workload_seed_lowering::lower(catalog));
    rows.extend(lookup_handoff_lowering::lower(catalog));
    rows.extend(replay_undo_boundary_lowering::lower(catalog));
    rows.extend(public_closeout_lowering::lower(catalog));
    rows.extend(topo_invalidation_lowering::lower(catalog));
    rows.sort_by_key(|row| row.source_identity());
    rows
}
