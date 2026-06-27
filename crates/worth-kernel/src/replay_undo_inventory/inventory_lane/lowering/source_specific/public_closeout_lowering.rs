use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredSourceCatalog, ReplayUndoDeclaredSourceIdentity,
};

use super::super::inventory_category::ReplayUndoInventoryCategory;
use super::super::inventory_disposition::ReplayUndoInventoryDisposition;
use super::super::inventory_owner::ReplayUndoInventoryOwner;
use super::super::inventory_row::ReplayUndoInventoryReportRow;

pub fn lower(catalog: &ReplayUndoDeclaredSourceCatalog) -> Vec<ReplayUndoInventoryReportRow> {
    [
        ReplayUndoDeclaredSourceIdentity::SpatialEvidenceLookupPublicCloseout,
        ReplayUndoDeclaredSourceIdentity::SpatialEvidenceLookupPublicCloseoutAssemblyInput,
    ]
    .into_iter()
    .map(|identity| {
        let source = catalog.require_source(identity).expect("declared source");
        ReplayUndoInventoryReportRow::new(
            source.identity(),
            source.source_path(),
            source.source_kind(),
            ReplayUndoInventoryOwner::WorthSpatial,
            ReplayUndoInventoryCategory::TransactionBoundary,
            ReplayUndoInventoryDisposition::Migrate,
            source.authority_roles().clone(),
            source.observability_roles().clone(),
            None,
        )
    })
    .collect()
}
