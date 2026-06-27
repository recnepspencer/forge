use super::inventory_classification::{
    ReplayUndoPublicCloseoutClassification, ReplayUndoPublicCloseoutInventoryRow,
};
use crate::replay_undo_consumer_cutover::{
    ReplayUndoConsumerCutoverCloseout, ReplayUndoHardDeletionCloseout,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayUndoMilestoneTwelvePublicCloseoutCounters {
    migrated_source_count: usize,
    deleted_source_count: usize,
    capped_source_count: usize,
    query_gap_source_count: usize,
    inventory_row_count: usize,
    residue_row_count: usize,
    deletion_row_count: usize,
    residue_cap_row_count: usize,
    forbidden_surface_denial_count: usize,
    hard_deletion_firewall_row_count: usize,
}

impl ReplayUndoMilestoneTwelvePublicCloseoutCounters {
    pub(crate) fn from_proof_products(
        inventory_rows: &[ReplayUndoPublicCloseoutInventoryRow],
        consumer_cutover: &ReplayUndoConsumerCutoverCloseout,
        hard_deletion: &ReplayUndoHardDeletionCloseout,
    ) -> Self {
        let classification_count = |classification| {
            inventory_rows
                .iter()
                .filter(|row| row.classification() == classification)
                .count()
        };
        Self {
            migrated_source_count: classification_count(
                ReplayUndoPublicCloseoutClassification::Migrated,
            ),
            deleted_source_count: classification_count(
                ReplayUndoPublicCloseoutClassification::Deleted,
            ),
            capped_source_count: classification_count(
                ReplayUndoPublicCloseoutClassification::Capped,
            ),
            query_gap_source_count: classification_count(
                ReplayUndoPublicCloseoutClassification::QueryGap,
            ),
            inventory_row_count: inventory_rows.len(),
            residue_row_count: consumer_cutover.residue_ledger().row_count(),
            deletion_row_count: hard_deletion.deletion_ledger().row_count(),
            residue_cap_row_count: hard_deletion.residue_cap_audit().row_count(),
            forbidden_surface_denial_count: consumer_cutover
                .forbidden_surface_denials()
                .row_count(),
            hard_deletion_firewall_row_count: hard_deletion.source_firewall().violation_count(),
        }
    }

    pub const fn migrated_source_count(self) -> usize {
        self.migrated_source_count
    }

    pub const fn deleted_source_count(self) -> usize {
        self.deleted_source_count
    }

    pub const fn capped_source_count(self) -> usize {
        self.capped_source_count
    }

    pub const fn query_gap_source_count(self) -> usize {
        self.query_gap_source_count
    }

    pub const fn inventory_row_count(self) -> usize {
        self.inventory_row_count
    }

    pub const fn residue_row_count(self) -> usize {
        self.residue_row_count
    }

    pub const fn deletion_row_count(self) -> usize {
        self.deletion_row_count
    }

    pub const fn residue_cap_row_count(self) -> usize {
        self.residue_cap_row_count
    }

    pub const fn forbidden_surface_denial_count(self) -> usize {
        self.forbidden_surface_denial_count
    }

    pub const fn hard_deletion_firewall_row_count(self) -> usize {
        self.hard_deletion_firewall_row_count
    }
}
