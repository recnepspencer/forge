use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredSourceCatalog, ReplayUndoDeclaredSourceIdentity,
};
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryDisposition;
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryReportRow;

use super::counters::ReplayUndoInventoryCloseoutCounters;
use super::error::{ReplayUndoInventoryError, ReplayUndoInventoryErrorKind};
use super::gap_row::ReplayUndoInventoryGapRow;
use super::reconciliation::reconcile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoInventoryCloseout {
    declared: ReplayUndoDeclaredSourceCatalog,
    rows: Vec<ReplayUndoInventoryReportRow>,
    counters: ReplayUndoInventoryCloseoutCounters,
    gap_rows: Vec<ReplayUndoInventoryGapRow>,
}

impl ReplayUndoInventoryCloseout {
    pub(crate) fn new(
        declared: ReplayUndoDeclaredSourceCatalog,
        mut rows: Vec<ReplayUndoInventoryReportRow>,
    ) -> Result<Self, ReplayUndoInventoryError> {
        rows.sort_by_key(|row| row.source_identity());
        let gap_rows = reconcile(&declared, &rows)?;
        let counters = ReplayUndoInventoryCloseoutCounters::new(declared.sources().len(), &rows);
        Ok(Self {
            declared,
            rows,
            counters,
            gap_rows,
        })
    }

    pub fn declared_sources(&self) -> &ReplayUndoDeclaredSourceCatalog {
        &self.declared
    }

    pub fn rows(&self) -> &[ReplayUndoInventoryReportRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &ReplayUndoInventoryCloseoutCounters {
        &self.counters
    }

    pub fn gap_rows(&self) -> &[ReplayUndoInventoryGapRow] {
        &self.gap_rows
    }

    pub fn require_source(
        &self,
        identity: ReplayUndoDeclaredSourceIdentity,
    ) -> Result<&ReplayUndoInventoryReportRow, ReplayUndoInventoryError> {
        self.rows
            .iter()
            .find(|row| row.source_identity() == identity)
            .ok_or_else(|| {
                ReplayUndoInventoryError::new(
                    ReplayUndoInventoryErrorKind::DeclaredSourceNotLowered,
                    format!(
                        "missing replay/undo inventory row for `{}`",
                        identity.as_str()
                    ),
                )
            })
    }

    pub fn require_full_declared_coverage(&self) -> Result<(), ReplayUndoInventoryError> {
        if let Some(gap_row) = self.gap_rows.first() {
            return Err(ReplayUndoInventoryError::new(
                ReplayUndoInventoryErrorKind::UnclassifiedSource,
                format!(
                    "replay/undo inventory still carries query-gap source `{}`",
                    gap_row.source_identity().as_str()
                ),
            ));
        }
        if let Some(row) = self
            .rows
            .iter()
            .find(|row| row.disposition() == ReplayUndoInventoryDisposition::QueryGap)
        {
            return Err(ReplayUndoInventoryError::new(
                ReplayUndoInventoryErrorKind::UnclassifiedSource,
                format!(
                    "replay/undo inventory still carries query-gap source `{}`",
                    row.source_identity().as_str()
                ),
            ));
        }
        Ok(())
    }
}
