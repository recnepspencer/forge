use std::collections::BTreeSet;

use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredSource, ReplayUndoDeclaredSourceCatalog,
};
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryDisposition;
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryReportRow;

use super::error::{ReplayUndoInventoryError, ReplayUndoInventoryErrorKind};
use super::gap_row::ReplayUndoInventoryGapRow;

pub(crate) fn reconcile(
    declared: &ReplayUndoDeclaredSourceCatalog,
    rows: &[ReplayUndoInventoryReportRow],
) -> Result<Vec<ReplayUndoInventoryGapRow>, ReplayUndoInventoryError> {
    let mut declared_identities = BTreeSet::new();
    for source in declared.sources() {
        if !declared_identities.insert(source.identity()) {
            return Err(ReplayUndoInventoryError::new(
                ReplayUndoInventoryErrorKind::DuplicateDeclaredSource,
                format!("duplicate declared source `{}`", source.identity().as_str()),
            ));
        }
    }

    let mut lowered_identities = BTreeSet::new();
    let mut gap_rows = Vec::new();
    for row in rows {
        if !lowered_identities.insert(row.source_identity()) {
            return Err(ReplayUndoInventoryError::new(
                ReplayUndoInventoryErrorKind::DuplicateLoweredSource,
                format!(
                    "duplicate lowered source `{}`",
                    row.source_identity().as_str()
                ),
            ));
        }
        let declared_source = declared
            .require_source(row.source_identity())
            .ok_or_else(|| {
                ReplayUndoInventoryError::new(
                    ReplayUndoInventoryErrorKind::LoweredSourceNotDeclared,
                    format!(
                        "lowered replay/undo source `{}` is not declared",
                        row.source_identity().as_str()
                    ),
                )
            })?;
        require_matching_roles(declared_source, row)?;
        if row.disposition() == ReplayUndoInventoryDisposition::QueryGap {
            let trigger = row.removal_trigger().ok_or_else(|| {
                ReplayUndoInventoryError::new(
                    ReplayUndoInventoryErrorKind::MissingGapTrigger,
                    format!(
                        "query-gap source `{}` is missing removal trigger",
                        row.source_identity().as_str()
                    ),
                )
            })?;
            gap_rows.push(ReplayUndoInventoryGapRow::new(
                row.source_identity(),
                trigger,
            ));
        }
    }

    for source in declared.sources() {
        if !lowered_identities.contains(&source.identity()) {
            return Err(ReplayUndoInventoryError::new(
                ReplayUndoInventoryErrorKind::DeclaredSourceNotLowered,
                format!(
                    "declared replay/undo source `{}` was not lowered into inventory",
                    source.identity().as_str()
                ),
            ));
        }
    }

    Ok(gap_rows)
}

fn require_matching_roles(
    declared: &ReplayUndoDeclaredSource,
    row: &ReplayUndoInventoryReportRow,
) -> Result<(), ReplayUndoInventoryError> {
    if declared.authority_roles() != row.authority_roles() {
        return Err(ReplayUndoInventoryError::new(
            ReplayUndoInventoryErrorKind::AuthorityRoleMismatch,
            format!(
                "authority role mismatch for `{}`",
                declared.identity().as_str()
            ),
        ));
    }
    if declared.observability_roles() != row.observability_roles() {
        return Err(ReplayUndoInventoryError::new(
            ReplayUndoInventoryErrorKind::ObservabilityRoleMismatch,
            format!(
                "observability role mismatch for `{}`",
                declared.identity().as_str()
            ),
        ));
    }
    Ok(())
}
