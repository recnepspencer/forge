use crate::{
    AuthenticatedFrameDigest, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    BlobStreamingVerifiedRead, ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

use super::plan::BlobPlacementMovementBasis;
use super::{
    AdmittedBlobPlacementMovementPlan, BlobPlacementMovementCounterSnapshot,
    BlobPlacementMovementDenial, BlobPlacementMovementReadHold,
    ExecutedBlobPlacementMovementReceipt, PublishedBlobPlacementObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobMovementReadPhase {
    BeforeMove,
    DuringMove,
    AfterMove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobMovementVerifiedReadEvidence {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    authenticated_frame_digest: AuthenticatedFrameDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    verified_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReadDuringPlacementMove {
    basis: BlobPlacementMovementBasis,
    phase: BlobMovementReadPhase,
    counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReadDuringPlacementMoveReceipt {
    basis: BlobPlacementMovementBasis,
    phase: BlobMovementReadPhase,
    verified_bytes: u64,
    counters: BlobPlacementMovementCounterSnapshot,
}

impl BlobMovementVerifiedReadEvidence {
    pub fn from_streaming_verified_read(
        basis: &AdmittedBlobPlacementMovementPlan,
        read_hold: BlobPlacementMovementReadHold,
        streaming_read: &BlobStreamingVerifiedRead,
    ) -> Result<Self, BlobPlacementMovementDenial> {
        if streaming_read.object_id() != basis.basis().object_id()
            || streaming_read.generation() != basis.basis().generation()
            || streaming_read.chunk_tree_root() != basis.basis().chunk_tree_root()
            || streaming_read.logical_content_digest() != basis.basis().logical_content_digest()
            || streaming_read.counters().bytes_read() > read_hold.guarded_bytes()
        {
            return Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch {
                counters: basis.counters().record_protected_denial(),
            });
        }
        Ok(Self::from_basis(
            basis.basis(),
            streaming_read.counters().bytes_read(),
        ))
    }

    #[cfg(test)]
    pub fn mismatched_for_certification_test(basis: &AdmittedBlobPlacementMovementPlan) -> Self {
        let mut read = Self::from_basis(basis.basis(), basis.read_hold().guarded_bytes());
        read.generation = BlobGeneration::published(read.generation.sequence() + 1);
        read
    }

    fn from_basis(basis: &BlobPlacementMovementBasis, verified_bytes: u64) -> Self {
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
        plan: &AdmittedBlobPlacementMovementPlan,
        phase: BlobMovementReadPhase,
    ) -> Self {
        Self {
            basis: plan.basis().clone(),
            phase,
            counters: plan.counters(),
        }
    }

    pub(crate) fn from_executed(
        receipt: &ExecutedBlobPlacementMovementReceipt,
        phase: BlobMovementReadPhase,
    ) -> Self {
        Self {
            basis: receipt.basis().clone(),
            phase,
            counters: receipt.counters(),
        }
    }

    pub(crate) fn from_published(observation: &PublishedBlobPlacementObservation) -> Self {
        Self {
            basis: observation.basis().clone(),
            phase: BlobMovementReadPhase::AfterMove,
            counters: observation.counters(),
        }
    }

    pub fn admit_verified_read(
        self,
        read: BlobMovementVerifiedReadEvidence,
    ) -> Result<BlobReadDuringPlacementMoveReceipt, BlobPlacementMovementDenial> {
        if !self.basis.matches_verified_basis(&read) {
            return Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch {
                counters: self.counters.record_protected_denial(),
            });
        }
        Ok(BlobReadDuringPlacementMoveReceipt {
            basis: self.basis,
            phase: self.phase,
            verified_bytes: read.verified_bytes(),
            counters: self.counters,
        })
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
