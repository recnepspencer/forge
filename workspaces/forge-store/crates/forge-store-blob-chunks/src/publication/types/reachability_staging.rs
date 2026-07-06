use crate::{
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId, BlobReachabilityCounterSnapshot,
    ChunkTreeRoot, LogicalContentDigest,
};

use super::super::evidence::BlobPublicationCounterReceiptIdentity;
use super::super::{BlobPublicationCounterSnapshot, BlobPublicationIntent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityStaging {
    pub(crate) intent: BlobPublicationIntent,
    pub(crate) staging_identity: BlobReachabilityStagingIdentity,
    pub(crate) staged_digest: LogicalContentDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) reachability_counters: BlobReachabilityCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityStagingIdentity {
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) chunk_tree_root: ChunkTreeRoot,
    pub(crate) logical_content_digest: LogicalContentDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) counter_receipt_identity: BlobPublicationCounterReceiptIdentity,
}

impl BlobReachabilityStaging {
    pub fn stage(
        candidate: super::root_candidate::BlobRootCandidateForPublication,
        reachability: crate::BlobChunkReachabilityProofSet,
    ) -> Result<Self, super::super::BlobPublicationDenial> {
        super::super::transitions::reachability_staging::stage(candidate, reachability)
    }

    pub const fn intent(&self) -> &BlobPublicationIntent {
        &self.intent
    }

    pub const fn staged_digest(&self) -> &LogicalContentDigest {
        &self.staged_digest
    }

    pub const fn staging_identity(&self) -> &BlobReachabilityStagingIdentity {
        &self.staging_identity
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn reachability_counters(&self) -> BlobReachabilityCounterSnapshot {
        self.reachability_counters
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.intent.counters()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        BlobPublicationIntent,
        BlobReachabilityStagingIdentity,
        BlobChunkSecurityMetadataWitness,
    ) {
        (self.intent, self.staging_identity, self.security_metadata)
    }
}

impl BlobReachabilityStagingIdentity {
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

    pub fn counter_receipt_identity(&self) -> &BlobPublicationCounterReceiptIdentity {
        &self.counter_receipt_identity
    }

    pub(crate) fn publication_record_digest(&self) -> String {
        super::super::evidence::publication_payload_frame_digest(self)
    }
}