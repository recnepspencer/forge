use crate::{
    BlobChunkReachabilityProofSet, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    BlobReachabilityCounterSnapshot, ChunkTreeRoot, LogicalContentDigest,
};

use super::{
    BlobPublicationCounterReceiptIdentity, BlobPublicationCounterSnapshot, BlobPublicationDenial,
    BlobPublicationIntent, BlobRootCandidateForPublication,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityStaging {
    intent: BlobPublicationIntent,
    staging_identity: BlobReachabilityStagingIdentity,
    staged_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    reachability_counters: BlobReachabilityCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityStagingIdentity {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counter_receipt_identity: BlobPublicationCounterReceiptIdentity,
}

impl BlobReachabilityStaging {
    pub fn stage(
        candidate: BlobRootCandidateForPublication,
        reachability: BlobChunkReachabilityProofSet,
    ) -> Result<Self, BlobPublicationDenial> {
        let intent = candidate.into_intent();
        let counters = intent.counters().with_reachability_staged();
        if !reachability.matches_publication_intent(&intent) {
            return Err(BlobPublicationDenial::ReachabilityDigestMismatch { counters });
        }
        let staged_intent = intent.with_counters(counters);
        Ok(Self {
            staging_identity: BlobReachabilityStagingIdentity::from_intent_and_receipt(
                &staged_intent,
                &reachability,
            ),
            staged_digest: staged_intent.logical_content_digest().clone(),
            security_metadata: reachability.security_metadata(),
            reachability_counters: reachability.counters(),
            intent: staged_intent,
        })
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
    fn from_intent_and_receipt(
        intent: &BlobPublicationIntent,
        reachability: &BlobChunkReachabilityProofSet,
    ) -> Self {
        Self {
            object_id: intent.object_id().clone(),
            generation: intent.generation(),
            chunk_tree_root: intent.chunk_tree_root().clone(),
            logical_content_digest: intent.logical_content_digest().clone(),
            security_metadata: reachability.security_metadata(),
            counter_receipt_identity:
                BlobPublicationCounterReceiptIdentity::from_reachability_staging(
                    intent.counters(),
                    reachability.counters(),
                ),
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

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn counter_receipt_identity(&self) -> &BlobPublicationCounterReceiptIdentity {
        &self.counter_receipt_identity
    }

    pub(crate) fn publication_record_digest(&self) -> String {
        super::evidence_identity::publication_payload_frame_digest(self)
    }
}
