use super::{BlobCompactionDenial, BlobCompactionEquivalence, BlobCompactionRewriteExecution};
use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkSecurityMetadataWitness,
    BlobGeneration, BlobObjectId, ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionPublishedObservation {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    old_root: ChunkTreeRoot,
    new_root: ChunkTreeRoot,
    logical_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    frame_digest: AuthenticatedFrameDigest,
    security: BlobChunkSecurityMetadataWitness,
    authority_class: BlobAuthorityClassification,
    equivalence: BlobCompactionEquivalence,
}

impl BlobCompactionPublishedObservation {
    pub(crate) fn publish(
        execution: BlobCompactionRewriteExecution,
    ) -> Result<Self, BlobCompactionDenial> {
        let basis = execution.plan().basis();
        Ok(Self {
            object_id: basis.object_id().clone(),
            generation: basis.generation(),
            old_root: basis.old_root().clone(),
            new_root: execution.equivalence().new_root().clone(),
            logical_digest: basis.logical_digest().clone(),
            stored_digest: basis.stored_digest().clone(),
            frame_digest: basis.frame_digest().clone(),
            security: basis.security(),
            authority_class: basis.authority_class(),
            equivalence: execution.equivalence().clone(),
        })
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn old_root(&self) -> &ChunkTreeRoot {
        &self.old_root
    }

    pub const fn new_root(&self) -> &ChunkTreeRoot {
        &self.new_root
    }

    pub const fn logical_digest(&self) -> &LogicalContentDigest {
        &self.logical_digest
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.frame_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security
    }

    pub const fn authority_classification(&self) -> BlobAuthorityClassification {
        self.authority_class
    }

    pub const fn equivalence(&self) -> &BlobCompactionEquivalence {
        &self.equivalence
    }
}
