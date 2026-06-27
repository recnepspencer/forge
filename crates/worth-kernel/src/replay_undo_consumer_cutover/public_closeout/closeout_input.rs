use super::error::{
    ReplayUndoMilestoneTwelvePublicCloseoutError, ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
};
use crate::replay_undo_consumer_cutover::{
    ReplayUndoConsumerCutoverCloseout, ReplayUndoHardDeletionCloseout,
};
use crate::replay_undo_inventory::ReplayUndoInventoryReport;
use crate::workload_composition::BooleanChainReplayUndoBoundaryHandoff;

pub struct ReplayUndoMilestoneTwelvePublicCloseoutInput<'a> {
    consumer_cutover: &'a ReplayUndoConsumerCutoverCloseout,
    hard_deletion: &'a ReplayUndoHardDeletionCloseout,
    inventory: &'a ReplayUndoInventoryReport,
}

impl<'a> ReplayUndoMilestoneTwelvePublicCloseoutInput<'a> {
    pub fn from_parts(
        consumer_cutover: &'a ReplayUndoConsumerCutoverCloseout,
        hard_deletion: &'a ReplayUndoHardDeletionCloseout,
        inventory: &'a ReplayUndoInventoryReport,
    ) -> Result<Self, ReplayUndoMilestoneTwelvePublicCloseoutError> {
        require_post_hard_deletion_seed(hard_deletion)?;
        require_matching_replay_undo_proof_products(consumer_cutover, hard_deletion)?;
        Ok(Self {
            consumer_cutover,
            hard_deletion,
            inventory,
        })
    }

    pub fn from_replay_undo_boundary(
        boundary: &'a BooleanChainReplayUndoBoundaryHandoff,
        inventory: &'a ReplayUndoInventoryReport,
    ) -> Result<Self, ReplayUndoMilestoneTwelvePublicCloseoutError> {
        Self::from_parts(
            boundary.consumer_cutover_closeout(),
            boundary.hard_deletion_closeout(),
            inventory,
        )
    }

    pub const fn consumer_cutover(&self) -> &'a ReplayUndoConsumerCutoverCloseout {
        self.consumer_cutover
    }

    pub const fn hard_deletion(&self) -> &'a ReplayUndoHardDeletionCloseout {
        self.hard_deletion
    }

    pub const fn inventory(&self) -> &'a ReplayUndoInventoryReport {
        self.inventory
    }
}

fn require_matching_replay_undo_proof_products(
    consumer_cutover: &ReplayUndoConsumerCutoverCloseout,
    hard_deletion: &ReplayUndoHardDeletionCloseout,
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    let seed = hard_deletion.milestone_thirteen_seed();
    if seed.transaction_packet_identity() == consumer_cutover.transaction_packet_identity()
        && seed.replay_scope_identity() == consumer_cutover.replay_scope_identity()
        && seed.undo_scope_identity() == consumer_cutover.undo_scope_identity()
    {
        Ok(())
    } else {
        Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::MismatchedProofProducts,
            "public replay/undo closeout requires hard-deletion proof derived from the same consumer cutover proof",
        ))
    }
}

fn require_post_hard_deletion_seed(
    hard_deletion: &ReplayUndoHardDeletionCloseout,
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    let seed = hard_deletion.milestone_thirteen_seed();
    if seed.hard_deletion_ledger_digest().is_some()
        && seed.residue_cap_audit_digest().is_some()
        && seed.hard_deletion_source_firewall_digest().is_some()
    {
        Ok(())
    } else {
        Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::UnpublishedHardDeletionProof,
            "public replay/undo closeout requires the post-hard-deletion Milestone 13 seed",
        ))
    }
}
