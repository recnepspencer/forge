use crate::replay_undo_inventory::{ReplayUndoInventoryReport, ReplayUndoSeedSurfaceAudit};
use crate::workload_composition::{
    BooleanChainIntegrationHandoff, CompletedBooleanLoopReconstructionHandoff,
};

use super::forbidden_surface_denial::ReplayUndoForbiddenConsumerSurfaceDenialLedger;

pub struct ReplayUndoConsumerCutoverCloseoutInput<'a> {
    loop_handoff: &'a CompletedBooleanLoopReconstructionHandoff,
    chain_handoff: &'a BooleanChainIntegrationHandoff,
    inventory: &'a ReplayUndoInventoryReport,
    source_firewall: &'a ReplayUndoSeedSurfaceAudit,
    forbidden_surface_denials: &'a ReplayUndoForbiddenConsumerSurfaceDenialLedger,
}

impl<'a> ReplayUndoConsumerCutoverCloseoutInput<'a> {
    pub fn new(
        loop_handoff: &'a CompletedBooleanLoopReconstructionHandoff,
        chain_handoff: &'a BooleanChainIntegrationHandoff,
        inventory: &'a ReplayUndoInventoryReport,
        source_firewall: &'a ReplayUndoSeedSurfaceAudit,
        forbidden_surface_denials: &'a ReplayUndoForbiddenConsumerSurfaceDenialLedger,
    ) -> Self {
        Self {
            loop_handoff,
            chain_handoff,
            inventory,
            source_firewall,
            forbidden_surface_denials,
        }
    }

    pub const fn loop_handoff(&self) -> &'a CompletedBooleanLoopReconstructionHandoff {
        self.loop_handoff
    }

    pub const fn chain_handoff(&self) -> &'a BooleanChainIntegrationHandoff {
        self.chain_handoff
    }

    pub const fn inventory(&self) -> &'a ReplayUndoInventoryReport {
        self.inventory
    }

    pub const fn source_firewall(&self) -> &'a ReplayUndoSeedSurfaceAudit {
        self.source_firewall
    }

    pub const fn forbidden_surface_denials(
        &self,
    ) -> &'a ReplayUndoForbiddenConsumerSurfaceDenialLedger {
        self.forbidden_surface_denials
    }
}
