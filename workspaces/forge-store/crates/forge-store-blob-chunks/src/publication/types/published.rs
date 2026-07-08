use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_recovery_physics::PartialPublicationCounterSnapshot;
use forge_store_wal::DurablePublicationDeclaration;

use crate::{
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectClassification, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest,
};

use super::super::{BlobPublicationCounterSnapshot, BlobPublicationSessionCloseout};
use super::reachability_staging::BlobReachabilityStagingIdentity;

#[derive(Debug)]
pub struct BlobPublicationAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobGenerationPublished {
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) chunk_tree_root: ChunkTreeRoot,
    pub(crate) logical_content_digest: LogicalContentDigest,
    pub(crate) classification: BlobObjectClassification,
    pub(crate) durable_publication: DurablePublicationDeclaration,
    pub(crate) replay_classification_digest: String,
    pub(crate) replay_counters: PartialPublicationCounterSnapshot,
    pub(crate) staging_identity: BlobReachabilityStagingIdentity,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) counters: BlobPublicationCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobVisibleGeneration {
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) chunk_tree_root: ChunkTreeRoot,
    pub(crate) logical_content_digest: LogicalContentDigest,
    pub(crate) classification: BlobObjectClassification,
    pub(crate) counters: BlobPublicationCounterSnapshot,
}

impl BlobPublicationAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub(crate) fn into_current_authority(self) -> StoreCurrentAuthorityWitness {
        self.current_authority
    }
}

impl BlobGenerationPublished {
    pub fn commit_visible(
        session_closeout: BlobPublicationSessionCloseout,
        authority: BlobPublicationAuthority,
    ) -> Self {
        super::super::transitions::commit_visible::commit_visible(session_closeout, authority)
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

    pub const fn durable_publication(&self) -> &DurablePublicationDeclaration {
        &self.durable_publication
    }

    pub fn replay_classification_digest(&self) -> &str {
        &self.replay_classification_digest
    }

    pub const fn replay_counters(&self) -> PartialPublicationCounterSnapshot {
        self.replay_counters
    }

    pub const fn staging_identity(&self) -> &BlobReachabilityStagingIdentity {
        &self.staging_identity
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.counters
    }
}

impl BlobVisibleGeneration {
    pub fn from_published(published: &BlobGenerationPublished) -> Self {
        super::super::receipt_construction::visibility::from_published(published)
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

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.counters
    }
}
