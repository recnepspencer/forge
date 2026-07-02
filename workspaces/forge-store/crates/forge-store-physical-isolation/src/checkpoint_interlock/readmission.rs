use super::CheckpointReadInterlockDenial;
use crate::{CheckpointPublicationRoot, CurrentPhysicalRoot};
use forge_store_recovery_physics::{
    CheckpointCutoverReceipt, CheckpointPageLsnFrontier, CheckpointValidation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPublicationReadmission {
    checkpoint_root: CheckpointPublicationRoot,
    published_current_root: CurrentPhysicalRoot,
    cutover_receipt: CheckpointCutoverReceipt,
    page_lsn_frontier: CheckpointPageLsnFrontier,
    frontier_bound_to_cutover: bool,
}

impl CheckpointPublicationReadmission {
    pub fn admit(
        checkpoint_root: CheckpointPublicationRoot,
        published_current_root: CurrentPhysicalRoot,
        validation: &CheckpointValidation,
        cutover_receipt: CheckpointCutoverReceipt,
    ) -> Result<Self, CheckpointReadInterlockDenial> {
        if checkpoint_root.epoch() != published_current_root.epoch() {
            return Err(
                CheckpointReadInterlockDenial::CheckpointPublicationRootNotReadmitted {
                    checkpoint_root: checkpoint_root.epoch(),
                    admitted_root: published_current_root.epoch(),
                },
            );
        }
        if !checkpoint_root
            .checkpoint_identity()
            .matches_checkpoint_id(validation.checkpoint_id())
        {
            return Err(CheckpointReadInterlockDenial::CheckpointPublicationRootCheckpointMismatch);
        }
        if cutover_receipt.checkpoint_id() != validation.checkpoint_id() {
            return Err(CheckpointReadInterlockDenial::CheckpointCutoverReceiptMismatch);
        }
        let validation_range = validation.manifest().covered_lsn_range();
        let receipt_range = cutover_receipt.covered_lsn_range();
        if receipt_range != validation_range {
            return Err(
                CheckpointReadInterlockDenial::CheckpointCutoverRangeMismatch {
                    validation_range,
                    receipt_range,
                },
            );
        }
        let page_lsn_frontier = validation.manifest().page_lsn_frontier().clone();
        for (_, page_lsn) in page_lsn_frontier.pages() {
            if !receipt_range.contains(page_lsn.lsn()) {
                return Err(
                    CheckpointReadInterlockDenial::PageLsnFrontierOutsideCutoverRange {
                        page_lsn: *page_lsn,
                    },
                );
            }
        }
        Ok(Self {
            checkpoint_root,
            published_current_root,
            cutover_receipt,
            page_lsn_frontier,
            frontier_bound_to_cutover: true,
        })
    }

    pub const fn checkpoint_root(&self) -> &CheckpointPublicationRoot {
        &self.checkpoint_root
    }

    pub const fn published_current_root(&self) -> CurrentPhysicalRoot {
        self.published_current_root
    }

    pub const fn cutover_receipt(&self) -> &CheckpointCutoverReceipt {
        &self.cutover_receipt
    }

    pub const fn page_lsn_frontier(&self) -> &CheckpointPageLsnFrontier {
        &self.page_lsn_frontier
    }

    pub const fn frontier_bound_to_cutover(&self) -> bool {
        self.frontier_bound_to_cutover
    }
}
