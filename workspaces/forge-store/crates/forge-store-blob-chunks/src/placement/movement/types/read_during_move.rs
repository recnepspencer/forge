use crate::{
    AuthenticatedFrameDigest, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

use super::basis::BlobPlacementMovementBasis;
use crate::placement::movement::counters::BlobPlacementMovementCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobMovementReadPhase {
    BeforeMove,
    DuringMove,
    AfterMove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobMovementVerifiedReadEvidence {
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) chunk_tree_root: ChunkTreeRoot,
    pub(crate) logical_content_digest: LogicalContentDigest,
    pub(crate) stored_digest: StoredChunkDigest,
    pub(crate) authenticated_frame_digest: AuthenticatedFrameDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) verified_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReadDuringPlacementMove {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) phase: BlobMovementReadPhase,
    pub(crate) counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReadDuringPlacementMoveReceipt {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) phase: BlobMovementReadPhase,
    pub(crate) verified_bytes: u64,
    pub(crate) counters: BlobPlacementMovementCounterSnapshot,
}

impl BlobMovementVerifiedReadEvidence {
    pub(crate) fn from_basis(basis: &BlobPlacementMovementBasis, verified_bytes: u64) -> Self {
        Self {
            object_id: basis.object_id().clone(),
            generation: basis.generation(),
            chunk_tree_root: basis.chunk_tree_root().clone(),
            logical_content_digest: basis.logical_content_digest().clone(),
            stored_digest: basis.stored_digest().clone(),
            authenticated_frame_digest: basis.authenticated_frame_digest().clone(),
            security_metadata: basis.security_metadata(),
            verified_bytes,
        }
    }

    #[cfg(test)]
    pub fn mismatched_for_certification_test(
        basis: &super::plan::AdmittedBlobPlacementMovementPlan,
    ) -> Self {
        let mut read = Self::from_basis(basis.basis(), basis.read_hold().guarded_bytes());
        read.generation = BlobGeneration::published(read.generation.sequence() + 1);
        read
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.authenticated_frame_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn verified_bytes(&self) -> u64 {
        self.verified_bytes
    }
}

impl BlobReadDuringPlacementMove {
    pub(crate) fn from_plan(
        plan: &super::plan::AdmittedBlobPlacementMovementPlan,
        phase: BlobMovementReadPhase,
    ) -> Self {
        Self {
            basis: plan.basis.clone(),
            phase,
            counters: plan.counters(),
        }
    }

    pub(crate) fn from_executed(
        receipt: &super::execution_receipt::ExecutedBlobPlacementMovementReceipt,
        phase: BlobMovementReadPhase,
    ) -> Self {
        Self {
            basis: receipt.basis().clone(),
            phase,
            counters: receipt.counters(),
        }
    }

    pub(crate) fn from_published(
        observation: &super::execution_receipt::PublishedBlobPlacementObservation,
    ) -> Self {
        Self {
            basis: observation.basis().clone(),
            phase: BlobMovementReadPhase::AfterMove,
            counters: observation.counters(),
        }
    }

    pub const fn phase(&self) -> BlobMovementReadPhase {
        self.phase
    }
}

impl BlobReadDuringPlacementMoveReceipt {
    pub const fn object_id(&self) -> &BlobObjectId {
        self.basis.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.basis.generation()
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.basis.stored_digest()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.basis.security_metadata()
    }

    pub const fn phase(&self) -> BlobMovementReadPhase {
        self.phase
    }

    pub const fn verified_bytes(&self) -> u64 {
        self.verified_bytes
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }
}