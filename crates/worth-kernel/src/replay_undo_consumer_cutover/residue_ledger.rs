use crate::replay_undo_inventory::{
    ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition, ReplayUndoInventoryOwner,
    ReplayUndoInventoryReport, ReplayUndoInventorySourceIdentity,
};

use super::error::{missing_residue_trigger, ReplayUndoConsumerCutoverError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoConsumerCutoverResidueRow {
    source_identity: ReplayUndoInventorySourceIdentity,
    owner: ReplayUndoInventoryOwner,
    category: ReplayUndoInventoryCategory,
    disposition: ReplayUndoInventoryDisposition,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoConsumerCutoverResidueLedger {
    rows: Vec<ReplayUndoConsumerCutoverResidueRow>,
}

impl ReplayUndoConsumerCutoverResidueLedger {
    pub(crate) fn from_inventory(
        inventory: &ReplayUndoInventoryReport,
    ) -> Result<Self, ReplayUndoConsumerCutoverError> {
        let mut rows = Vec::new();
        for row in inventory.rows() {
            if matches!(
                row.disposition(),
                ReplayUndoInventoryDisposition::Cap | ReplayUndoInventoryDisposition::QueryGap
            ) {
                let removal_trigger = row
                    .removal_trigger()
                    .ok_or_else(|| missing_residue_trigger(row.source_identity()))?;
                rows.push(ReplayUndoConsumerCutoverResidueRow {
                    source_identity: row.source_identity(),
                    owner: row.owner(),
                    category: row.category(),
                    disposition: row.disposition(),
                    removal_trigger: removal_trigger.to_string(),
                });
            }
        }
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[ReplayUndoConsumerCutoverResidueRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

impl ReplayUndoConsumerCutoverResidueRow {
    pub const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub const fn owner(&self) -> ReplayUndoInventoryOwner {
        self.owner
    }

    pub const fn category(&self) -> ReplayUndoInventoryCategory {
        self.category
    }

    pub const fn disposition(&self) -> ReplayUndoInventoryDisposition {
        self.disposition
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}
