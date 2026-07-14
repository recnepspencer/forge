use worth_store_recovery_physics::PartialPublicationCounterSnapshot;
use worth_store_wal::DurablePublicationDeclaration;

use crate::BlobChunkSecurityMetadataWitness;

use super::super::{BlobPublicationCounterSnapshot, BlobPublicationDenial, BlobPublicationIntent};
use super::reachability_staging::{BlobReachabilityStaging, BlobReachabilityStagingIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalRecord {
    pub(crate) intent: BlobPublicationIntent,
    pub(crate) wal_commit: BlobPublicationWalCommit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalPayload {
    pub(crate) staging_identity: BlobReachabilityStagingIdentity,
    pub(crate) frame_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalCommit {
    pub(crate) intent: BlobPublicationIntent,
    pub(crate) durable_publication: DurablePublicationDeclaration,
    pub(crate) replay_classification_digest: String,
    pub(crate) replay_counters: PartialPublicationCounterSnapshot,
    pub(crate) staging_identity: BlobReachabilityStagingIdentity,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobPublicationWalPayload {
    pub fn from_staged_reachability(staged: &BlobReachabilityStaging) -> Self {
        super::super::transitions::wal_payload::from_staged_reachability(staged)
    }

    pub const fn staging_identity(&self) -> &BlobReachabilityStagingIdentity {
        &self.staging_identity
    }

    pub fn frame_digest(&self) -> &str {
        &self.frame_digest
    }
}

impl BlobPublicationWalRecord {
    pub fn append(wal_commit: BlobPublicationWalCommit) -> Self {
        super::super::transitions::wal_record::append(wal_commit)
    }

    pub const fn intent(&self) -> &BlobPublicationIntent {
        &self.intent
    }

    pub const fn wal_commit(&self) -> &BlobPublicationWalCommit {
        &self.wal_commit
    }

    pub const fn durable_publication(&self) -> &DurablePublicationDeclaration {
        self.wal_commit.durable_publication()
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.intent.counters()
    }

    pub(crate) fn into_parts(self) -> (BlobPublicationIntent, BlobPublicationWalCommit) {
        (self.intent, self.wal_commit)
    }
}

impl BlobPublicationWalCommit {
    pub fn from_replayable_wal_record(
        staged: BlobReachabilityStaging,
        payload: BlobPublicationWalPayload,
        durable_publication: DurablePublicationDeclaration,
        replay_report: &worth_store_recovery_physics::CrashBoundaryLayoutReport,
    ) -> Result<Self, BlobPublicationDenial> {
        super::super::transitions::wal_commit::from_replayable_wal_record(
            staged,
            payload,
            durable_publication,
            replay_report,
        )
    }

    pub const fn durable_publication(&self) -> &DurablePublicationDeclaration {
        &self.durable_publication
    }

    pub const fn intent(&self) -> &BlobPublicationIntent {
        &self.intent
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
}
