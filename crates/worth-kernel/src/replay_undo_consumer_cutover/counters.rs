use crate::replay_undo_inventory::ReplayUndoInventoryCounters;
use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacketCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoConsumerCutoverCounters {
    migrated_sources: usize,
    deleted_sources: usize,
    capped_residue_sources: usize,
    query_gap_sources: usize,
    replay_raw_row_scan_count: usize,
    replay_broad_receipt_scan_count: usize,
    replay_caller_owned_scan_count: usize,
    undo_raw_row_scan_count: usize,
    undo_broad_receipt_scan_count: usize,
    undo_caller_owned_scan_count: usize,
}

impl ReplayUndoConsumerCutoverCounters {
    pub(crate) fn from_inventory_and_packet(
        inventory: &ReplayUndoInventoryCounters,
        packet: &ReplayUndoTransactionBoundaryPacketCounters,
    ) -> Self {
        Self {
            migrated_sources: inventory.migrate_sources(),
            deleted_sources: inventory.delete_sources(),
            capped_residue_sources: inventory.capped_sources(),
            query_gap_sources: inventory.query_gap_sources(),
            replay_raw_row_scan_count: packet.replay_raw_row_scan_count(),
            replay_broad_receipt_scan_count: packet.replay_broad_receipt_scan_count(),
            replay_caller_owned_scan_count: packet.replay_caller_owned_scan_count(),
            undo_raw_row_scan_count: packet.undo_raw_row_scan_count(),
            undo_broad_receipt_scan_count: packet.undo_broad_receipt_scan_count(),
            undo_caller_owned_scan_count: packet.undo_caller_owned_scan_count(),
        }
    }

    pub const fn migrated_sources(&self) -> usize {
        self.migrated_sources
    }

    pub const fn deleted_sources(&self) -> usize {
        self.deleted_sources
    }

    pub const fn capped_residue_sources(&self) -> usize {
        self.capped_residue_sources
    }

    pub const fn query_gap_sources(&self) -> usize {
        self.query_gap_sources
    }

    pub const fn replay_raw_row_scan_count(&self) -> usize {
        self.replay_raw_row_scan_count
    }

    pub const fn replay_broad_receipt_scan_count(&self) -> usize {
        self.replay_broad_receipt_scan_count
    }

    pub const fn replay_caller_owned_scan_count(&self) -> usize {
        self.replay_caller_owned_scan_count
    }

    pub const fn undo_raw_row_scan_count(&self) -> usize {
        self.undo_raw_row_scan_count
    }

    pub const fn undo_broad_receipt_scan_count(&self) -> usize {
        self.undo_broad_receipt_scan_count
    }

    pub const fn undo_caller_owned_scan_count(&self) -> usize {
        self.undo_caller_owned_scan_count
    }
}
