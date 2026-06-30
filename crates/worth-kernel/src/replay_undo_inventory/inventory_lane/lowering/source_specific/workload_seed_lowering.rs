use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredSourceCatalog, ReplayUndoDeclaredSourceIdentity,
};

use super::super::inventory_category::ReplayUndoInventoryCategory;
use super::super::inventory_disposition::ReplayUndoInventoryDisposition;
use super::super::inventory_owner::ReplayUndoInventoryOwner;
use super::super::inventory_row::ReplayUndoInventoryReportRow;

pub fn lower(catalog: &ReplayUndoDeclaredSourceCatalog) -> Vec<ReplayUndoInventoryReportRow> {
    [
        (
            ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadRetainedReplay,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::SpatialReplayScope,
            ReplayUndoInventoryDisposition::Migrate,
            None,
        ),
        (
            ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadDiagnostics,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::Residue,
            ReplayUndoInventoryDisposition::Cap,
            Some("milestone12.public_closeout_cutover"),
        ),
        (
            ReplayUndoDeclaredSourceIdentity::KernelLookupConsumedWorkloadComposition,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::SpatialReplayScope,
            ReplayUndoInventoryDisposition::Migrate,
            None,
        ),
    ]
    .into_iter()
    .map(|(identity, owner, category, disposition, trigger)| {
        let source = catalog.require_source(identity).expect("declared source");
        let residue_cap = (disposition == ReplayUndoInventoryDisposition::Cap).then_some(1);
        let observed_residue_count = usize::from(residue_cap.is_some());
        ReplayUndoInventoryReportRow::new_with_residue_count(
            source.identity(),
            source.source_path(),
            source.source_kind(),
            owner,
            category,
            disposition,
            source.authority_roles().clone(),
            source.observability_roles().clone(),
            trigger,
            residue_cap,
            observed_residue_count,
        )
    })
    .collect()
}
