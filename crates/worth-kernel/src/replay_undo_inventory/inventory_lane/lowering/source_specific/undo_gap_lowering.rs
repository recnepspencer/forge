use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredSourceCatalog, ReplayUndoDeclaredSourceIdentity,
};

use super::super::inventory_category::ReplayUndoInventoryCategory;
use super::super::inventory_disposition::ReplayUndoInventoryDisposition;
use super::super::inventory_owner::ReplayUndoInventoryOwner;
use super::super::inventory_row::ReplayUndoInventoryReportRow;

pub fn lower(catalog: &ReplayUndoDeclaredSourceCatalog) -> Vec<ReplayUndoInventoryReportRow> {
    let source = catalog
        .require_source(ReplayUndoDeclaredSourceIdentity::KernelUndoOrdinaryLaneGap)
        .expect("declared source");
    vec![ReplayUndoInventoryReportRow::new(
        source.identity(),
        source.source_path(),
        source.source_kind(),
        ReplayUndoInventoryOwner::WorthKernel,
        ReplayUndoInventoryCategory::UndoScope,
        ReplayUndoInventoryDisposition::QueryGap,
        source.authority_roles().clone(),
        source.observability_roles().clone(),
        Some("milestone12.undo_family_lane"),
    )]
}
