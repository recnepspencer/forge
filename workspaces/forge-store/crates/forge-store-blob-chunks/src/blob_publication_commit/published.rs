use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_recovery_physics::PartialPublicationCounterSnapshot;
use forge_store_wal::DurablePublicationDeclaration;

use crate::{
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectClassification, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest,
};

use super::{
    BlobPublicationCounterSnapshot, BlobPublicationIntent, BlobPublicationSessionCloseout,
    BlobPublicationWalCommit, BlobReachabilityStagingIdentity,
};

#[derive(Debug)]
pub struct BlobPublicationAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobGenerationPublished {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    durable_publication: DurablePublicationDeclaration,
    replay_classification_digest: String,
    replay_counters: PartialPublicationCounterSnapshot,
    staging_identity: BlobReachabilityStagingIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobPublicationCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobVisibleGeneration {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    counters: BlobPublicationCounterSnapshot,
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
        let _current_authority = authority.into_current_authority();
        let (intent, wal_commit) = session_closeout.into_parts();
        Self::from_committed_intent(intent, wal_commit)
    }

    fn from_committed_intent(
        intent: BlobPublicationIntent,
        wal_commit: BlobPublicationWalCommit,
    ) -> Self {
        Self {
            object_id: intent.object_id().clone(),
            generation: intent.generation(),
            chunk_tree_root: intent.chunk_tree_root().clone(),
            logical_content_digest: intent.logical_content_digest().clone(),
            classification: intent.classification(),
            durable_publication: wal_commit.durable_publication().clone(),
            replay_classification_digest: wal_commit.replay_classification_digest().to_owned(),
            replay_counters: wal_commit.replay_counters(),
            staging_identity: wal_commit.staging_identity().clone(),
            security_metadata: wal_commit.security_metadata(),
            counters: intent.counters().with_committed_publication(),
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
        Self {
            object_id: published.object_id.clone(),
            generation: published.generation,
            chunk_tree_root: published.chunk_tree_root.clone(),
            logical_content_digest: published.logical_content_digest.clone(),
            classification: published.classification,
            counters: published.counters.with_visible_observation(),
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

    pub const fn classification(&self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.counters
    }
}
