use worth_store_wal::BlobWalRecordEnvelope;

use worth_store_physical_isolation::CurrentGenerationPhysicalReference;

use crate::{
    BlobChunkProofLeaf, BlobChunkSecurityMetadataWitness, BlobReachabilityStaging,
    BlobRootCandidateForPublication, BlobStreamingContentFrontier,
};

use super::{
    BlobResumeCheckpointIdentity, BlobResumeCheckpointStateKind, BlobResumeCounterSnapshot,
    BlobResumeReadmissionAuthority, BlobResumeSessionAdmitted, BlobResumeSessionId,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobResumeCheckpoint {
    session_id: BlobResumeSessionId,
    authority_digest: String,
    security_metadata: BlobChunkSecurityMetadataWitness,
    declared_total_bytes: u64,
    state: BlobResumeCheckpointStateKind,
    checkpoint_identity: BlobResumeCheckpointIdentity,
    physical_reference: Option<CurrentGenerationPhysicalReference>,
    latest_leaf: Option<BlobChunkProofLeaf>,
    frontier: Option<BlobStreamingContentFrontier>,
    root_candidate: Option<BlobRootCandidateForPublication>,
    reachability_staging: Option<BlobReachabilityStaging>,
    stale: bool,
    counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPersistedResumeCheckpointSource {
    checkpoint_identity: BlobResumeCheckpointIdentity,
    session_id: BlobResumeSessionId,
    authority_digest: String,
    state: BlobResumeCheckpointStateKind,
    counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobResumeCheckpointReadmission {
    checkpoint: BlobResumeCheckpoint,
    source: BlobPersistedResumeCheckpointSource,
}

impl BlobResumeCheckpoint {
    pub const fn state(&self) -> BlobResumeCheckpointStateKind {
        self.state
    }

    pub fn session_id(&self) -> &BlobResumeSessionId {
        &self.session_id
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn declared_total_bytes(&self) -> u64 {
        self.declared_total_bytes
    }

    pub fn checkpoint_identity(&self) -> &BlobResumeCheckpointIdentity {
        &self.checkpoint_identity
    }

    pub const fn latest_leaf(&self) -> Option<&BlobChunkProofLeaf> {
        self.latest_leaf.as_ref()
    }

    pub const fn physical_reference(&self) -> Option<CurrentGenerationPhysicalReference> {
        self.physical_reference
    }

    pub const fn frontier(&self) -> Option<&BlobStreamingContentFrontier> {
        self.frontier.as_ref()
    }

    pub const fn root_candidate(&self) -> Option<&BlobRootCandidateForPublication> {
        self.root_candidate.as_ref()
    }

    pub const fn reachability_staging(&self) -> Option<&BlobReachabilityStaging> {
        self.reachability_staging.as_ref()
    }

    pub const fn stale(&self) -> bool {
        self.stale
    }

    #[cfg(test)]
    pub(crate) fn mark_stale_for_replay_test(mut self) -> Self {
        self.stale = true;
        self
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.counters
    }

    pub(crate) fn with_state(mut self, state: BlobResumeCheckpointStateKind) -> Self {
        self.state = state;
        self
    }

    pub fn readmit(
        self,
        authority: &BlobResumeReadmissionAuthority,
    ) -> Option<BlobResumeCheckpointReadmission> {
        if self.stale || self.authority_digest != authority.authority_digest() {
            return None;
        }
        if self.checkpoint_identity.as_str() != authority.replay_checkpoint_source_digest() {
            return None;
        }
        let source = BlobPersistedResumeCheckpointSource {
            checkpoint_identity: self.checkpoint_identity.clone(),
            session_id: self.session_id.clone(),
            authority_digest: self.authority_digest.clone(),
            state: self.state,
            counters: self.counters,
        };
        Some(BlobResumeCheckpointReadmission {
            checkpoint: self,
            source,
        })
    }
}

impl BlobResumeCheckpointReadmission {
    pub const fn checkpoint(&self) -> &BlobResumeCheckpoint {
        &self.checkpoint
    }

    pub const fn source(&self) -> &BlobPersistedResumeCheckpointSource {
        &self.source
    }

    pub(crate) fn into_parts(self) -> (BlobResumeCheckpoint, BlobPersistedResumeCheckpointSource) {
        (self.checkpoint, self.source)
    }
}

impl BlobPersistedResumeCheckpointSource {
    pub fn checkpoint_identity(&self) -> &BlobResumeCheckpointIdentity {
        &self.checkpoint_identity
    }

    pub fn session_id(&self) -> &BlobResumeSessionId {
        &self.session_id
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub const fn state(&self) -> BlobResumeCheckpointStateKind {
        self.state
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.counters
    }
}

pub(super) fn checkpoint_identity(
    session_id: &BlobResumeSessionId,
    record: &BlobWalRecordEnvelope,
    state: &str,
) -> BlobResumeCheckpointIdentity {
    BlobResumeCheckpointIdentity::from_parts(session_id, record.payload_digest(), state)
}

pub(super) fn checkpoint_from_parts(
    admitted: BlobResumeSessionAdmitted,
    state: BlobResumeCheckpointStateKind,
    checkpoint_identity: BlobResumeCheckpointIdentity,
    latest_leaf: Option<BlobChunkProofLeaf>,
    physical_reference: Option<CurrentGenerationPhysicalReference>,
    frontier: Option<BlobStreamingContentFrontier>,
    root_candidate: Option<BlobRootCandidateForPublication>,
    reachability_staging: Option<BlobReachabilityStaging>,
) -> BlobResumeCheckpoint {
    BlobResumeCheckpoint {
        session_id: admitted.session_id,
        authority_digest: admitted.authority_digest,
        security_metadata: admitted.declaration.security_metadata,
        declared_total_bytes: admitted.declaration.declared_total_bytes,
        state,
        checkpoint_identity,
        physical_reference,
        latest_leaf,
        frontier,
        root_candidate,
        reachability_staging,
        stale: false,
        counters: admitted.counters,
    }
}
