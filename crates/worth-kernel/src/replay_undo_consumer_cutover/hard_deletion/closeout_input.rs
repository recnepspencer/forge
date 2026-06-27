use crate::replay_undo_inventory::ReplayUndoInventoryReport;

use super::source_firewall::ReplayUndoHardDeletionSourceFirewall;
use crate::replay_undo_consumer_cutover::ReplayUndoConsumerCutoverCloseout;

pub struct ReplayUndoHardDeletionCloseoutInput<'a> {
    consumer_cutover: &'a ReplayUndoConsumerCutoverCloseout,
    inventory: &'a ReplayUndoInventoryReport,
    source_firewall: ReplayUndoHardDeletionSourceFirewall,
}

impl<'a> ReplayUndoHardDeletionCloseoutInput<'a> {
    pub fn from_cutover(
        consumer_cutover: &'a ReplayUndoConsumerCutoverCloseout,
        inventory: &'a ReplayUndoInventoryReport,
        source_firewall: ReplayUndoHardDeletionSourceFirewall,
    ) -> Self {
        Self {
            consumer_cutover,
            inventory,
            source_firewall,
        }
    }

    pub const fn consumer_cutover(&self) -> &'a ReplayUndoConsumerCutoverCloseout {
        self.consumer_cutover
    }

    pub const fn inventory(&self) -> &'a ReplayUndoInventoryReport {
        self.inventory
    }

    pub const fn source_firewall(&self) -> &ReplayUndoHardDeletionSourceFirewall {
        &self.source_firewall
    }
}
