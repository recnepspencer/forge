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
        ReplayUndoInventoryReportRow::new(
            source.identity(),
            source.source_path(),
            source.source_kind(),
            owner,
            category,
            disposition,
            source.authority_roles().clone(),
            source.observability_roles().clone(),
            trigger,
        )
    })
    .collect()
}
