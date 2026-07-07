use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkSecurityMetadataWitness,
    BlobGeneration, BlobObjectId, ChunkTreeRoot, LifecycleReceipt, LogicalContentDigest,
    StoredChunkDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobCompactionBasis {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    old_root: ChunkTreeRoot,
    logical_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    frame_digest: AuthenticatedFrameDigest,
    security: BlobChunkSecurityMetadataWitness,
    authority_class: BlobAuthorityClassification,
}

impl BlobCompactionBasis {
    pub(crate) fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        let declaration = receipt.declaration();
        Self {
            object_id: declaration.object_id().clone(),
            generation: declaration.generation(),
            old_root: declaration.chunk_tree_root().clone(),
            logical_digest: declaration.logical_content_digest().clone(),
            stored_digest: declaration.stored_chunk_digest().clone(),
            frame_digest: declaration.authenticated_frame_digest().clone(),
            security: declaration.security_metadata(),
            authority_class: declaration.authority_classification(),
        }
    }

    pub(crate) const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub(crate) const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub(crate) const fn old_root(&self) -> &ChunkTreeRoot {
        &self.old_root
    }

    pub(crate) const fn logical_digest(&self) -> &LogicalContentDigest {
        &self.logical_digest
    }

    pub(crate) const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub(crate) const fn frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.frame_digest
    }

    pub(crate) const fn security(&self) -> BlobChunkSecurityMetadataWitness {
        self.security
    }

    pub(crate) const fn authority_class(&self) -> BlobAuthorityClassification {
        self.authority_class
    }
}
