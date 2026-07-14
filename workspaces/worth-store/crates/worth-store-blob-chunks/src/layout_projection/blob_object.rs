use worth_store_contracts::DurableArtifactFamilyId;

use super::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence};
use crate::{
    BlobGeneration, BlobGenerationPublished, BlobObjectClassification, BlobObjectId, ChunkTreeRoot,
    LogicalContentDigest,
};

pub fn reject_chunk_tree_root_as_blob_object_layout_authority(
    _root: &ChunkTreeRoot,
) -> Result<(), BlobLayoutAccessDenial> {
    Err(BlobLayoutAccessDenial::new(
        BlobLayoutAccessDenialKind::ChunkTreeRootCannotStandInForBlobObjectLayoutAuthority,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobObjectLayoutReport {
    family_id: DurableArtifactFamilyId,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobGenerationPublicationLayoutReport {
    family_id: DurableArtifactFamilyId,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    logical_content_digest: LogicalContentDigest,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl BlobObjectLayoutReport {
    fn from_published(published: &BlobGenerationPublished) -> Self {
        let family_id = DurableArtifactFamilyId::BlobManifest;
        Self {
            family_id,
            object_id: published.object_id().clone(),
            generation: published.generation(),
            chunk_tree_root: published.chunk_tree_root().clone(),
            logical_content_digest: published.logical_content_digest().clone(),
            classification: published.classification(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_publication(
                family_id,
                published.counters(),
            ),
        }
    }

    pub fn project_generation_publication_layout(&self) -> BlobGenerationPublicationLayoutReport {
        BlobGenerationPublicationLayoutReport {
            family_id: DurableArtifactFamilyId::PublicationWalPublicationProgress,
            object_id: self.object_id.clone(),
            generation: self.generation,
            logical_content_digest: self.logical_content_digest.clone(),
            counter_evidence: self.counter_evidence,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
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

    pub const fn classification(&self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl BlobGenerationPublicationLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl BlobGenerationPublished {
    pub fn project_blob_object_layout(
        &self,
    ) -> Result<BlobObjectLayoutReport, BlobLayoutAccessDenial> {
        Ok(BlobObjectLayoutReport::from_published(self))
    }
}
