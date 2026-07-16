use crate::{BlobGeneration, BlobObjectId, BlobPlacementClass, StoredChunkDigest};

use super::{BlobPlacementMovementCounterSnapshot, ExecutedBlobPlacementMovementReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPlacementMovementResidue {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    stored_digest: StoredChunkDigest,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobPlacementMovementRestartOutcome {
    ResumeFromExecutedReceipt(Box<ExecutedBlobPlacementMovementReceipt>),
    LocalizedResidue(BlobPlacementMovementResidue),
}

impl BlobPlacementMovementRestartOutcome {
    pub fn resume_from_receipt(receipt: ExecutedBlobPlacementMovementReceipt) -> Self {
        Self::ResumeFromExecutedReceipt(Box::new(receipt))
    }

    pub fn localize_residue(receipt: &ExecutedBlobPlacementMovementReceipt) -> Self {
        Self::LocalizedResidue(BlobPlacementMovementResidue {
            object_id: receipt.object_id().clone(),
            generation: receipt.generation(),
            stored_digest: receipt.stored_digest().clone(),
            source_class: receipt.source_class(),
            target_class: receipt.target_class(),
            counters: receipt.counters().record_tier_move_retry(),
        })
    }

    pub const fn publishes_mixed_placement(&self) -> bool {
        false
    }
}

impl BlobPlacementMovementResidue {
    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn source_class(&self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(&self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }
}
