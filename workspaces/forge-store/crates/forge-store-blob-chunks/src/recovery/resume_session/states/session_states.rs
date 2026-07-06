use forge_store_physical_isolation::CurrentGenerationPhysicalReference;
use forge_store_wal::BlobWalRecordEnvelope;

use crate::{
    BlobChunkProofLeaf, BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission,
    BlobReachabilityStaging, BlobRootCandidateForPublication, BlobStreamingContentFrontier,
};

use super::super::{BlobResumeCounterSnapshot, BlobResumeSessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionDeclaration {
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) chunking_rule: BlobChunkingRuleAdmission,
    pub(crate) declared_total_bytes: u64,
    pub(crate) counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionAdmitted {
    pub(crate) session_id: BlobResumeSessionId,
    pub(crate) authority_digest: String,
    pub(crate) declaration: BlobResumeSessionDeclaration,
    pub(crate) counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkAppendStarted {
    pub(crate) admitted: BlobResumeSessionAdmitted,
    pub(crate) ordinal: crate::BlobChunkOrdinal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkBytesDurable {
    pub(crate) admitted: BlobResumeSessionAdmitted,
    pub(crate) ordinal: crate::BlobChunkOrdinal,
    pub(crate) wal_record: BlobWalRecordEnvelope,
    pub(crate) durable_bytes: u64,
    pub(crate) physical_reference: CurrentGenerationPhysicalReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkIntegrityAdmitted {
    pub(crate) durable: BlobResumeChunkBytesDurable,
    pub(crate) leaf: BlobChunkProofLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeFrontierCheckpointed {
    pub(crate) integrity: BlobResumeChunkIntegrityAdmitted,
    pub(crate) frontier: BlobStreamingContentFrontier,
    pub(crate) checkpoint_record: BlobWalRecordEnvelope,
    pub(crate) checkpoint_identity: super::super::BlobResumeCheckpointIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeRootCandidateBuilt {
    pub(crate) checkpointed: BlobResumeFrontierCheckpointed,
    pub(crate) root_candidate: BlobRootCandidateForPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeRootPublicationReady {
    pub(crate) root_candidate: BlobResumeRootCandidateBuilt,
    pub(crate) reachability_staging: BlobReachabilityStaging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionClosed {
    pub(crate) ready: BlobResumeRootPublicationReady,
    pub(crate) closeout_record: BlobWalRecordEnvelope,
}