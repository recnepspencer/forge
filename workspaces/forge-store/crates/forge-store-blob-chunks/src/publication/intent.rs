use crate::{
    BlobChunkIntegrityCounterSnapshot, BlobChunkRootCounterSnapshot, BlobChunkRootPublication,
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobGenerationObservation,
    BlobGenerationRegistryCounterSnapshot, BlobObjectClassification, BlobObjectId, ChunkTreeRoot,
    LogicalContentDigest,
};

use super::{BlobPublicationCounterSnapshot, BlobPublicationDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationIntent {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    classification: BlobObjectClassification,
    root_counters: BlobChunkRootCounterSnapshot,
    source_chunk_counters: BlobChunkIntegrityCounterSnapshot,
    registry_counters: BlobGenerationRegistryCounterSnapshot,
    counters: BlobPublicationCounterSnapshot,
}

impl BlobPublicationIntent {
    pub(crate) fn from_registry_observation(
        observation: BlobGenerationObservation<'_>,
        root_publication: &BlobChunkRootPublication,
    ) -> Result<Self, BlobPublicationDenial> {
        let counters = BlobPublicationCounterSnapshot::start().with_root_candidate();
        if observation.chunk_tree_root() != root_publication.chunk_tree_root()
            || observation.logical_content_digest() != root_publication.logical_content_digest()
        {
            return Err(BlobPublicationDenial::RootCandidateRegistryMismatch { counters });
        }
        Ok(Self {
            object_id: observation.object_id().clone(),
            generation: observation.generation(),
            chunk_tree_root: observation.chunk_tree_root().clone(),
            logical_content_digest: observation.logical_content_digest().clone(),
            security_metadata: observation
                .lifecycle_receipt()
                .reachability()
                .security_metadata(),
            classification: observation.classification(),
            root_counters: root_publication.counters(),
            source_chunk_counters: root_publication.source_counters(),
            registry_counters: observation.counters(),
            counters,
        })
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

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn classification(&self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn root_counters(&self) -> BlobChunkRootCounterSnapshot {
        self.root_counters
    }

    pub const fn source_chunk_counters(&self) -> BlobChunkIntegrityCounterSnapshot {
        self.source_chunk_counters
    }

    pub const fn registry_counters(&self) -> BlobGenerationRegistryCounterSnapshot {
        self.registry_counters
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn with_counters(mut self, counters: BlobPublicationCounterSnapshot) -> Self {
        self.counters = counters;
        self
    }
}
