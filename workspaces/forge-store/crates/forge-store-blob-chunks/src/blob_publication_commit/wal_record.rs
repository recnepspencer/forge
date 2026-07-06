use forge_store_recovery_physics::{
    PartialPublicationClassification, PartialPublicationCounterSnapshot, UnacknowledgedDurableWal,
};
use forge_store_wal::{
    DurablePublicationDeclaration, DurablePublicationScope, WalFrameDurablePublicationScope,
};

use crate::BlobChunkSecurityMetadataWitness;

use super::{
    evidence_identity::publication_payload_frame_digest, BlobPublicationCounterSnapshot,
    BlobPublicationDenial, BlobPublicationIntent, BlobReachabilityStaging,
    BlobReachabilityStagingIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalRecord {
    intent: BlobPublicationIntent,
    wal_commit: BlobPublicationWalCommit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalPayload {
    staging_identity: BlobReachabilityStagingIdentity,
    frame_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationWalCommit {
    intent: BlobPublicationIntent,
    durable_publication: DurablePublicationDeclaration,
    replay_classification_digest: String,
    replay_counters: PartialPublicationCounterSnapshot,
    staging_identity: BlobReachabilityStagingIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobPublicationWalPayload {
    pub fn from_staged_reachability(staged: &BlobReachabilityStaging) -> Self {
        let staging_identity = staged.staging_identity().clone();
        let frame_digest = publication_payload_frame_digest(&staging_identity);
        Self {
            staging_identity,
            frame_digest,
        }
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
        let intent = wal_commit.intent.clone();
        let counters = intent.counters().with_wal_record();
        Self {
            intent: intent.with_counters(counters),
            wal_commit,
        }
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
        replay_classification: &PartialPublicationClassification,
    ) -> Result<Self, BlobPublicationDenial> {
        let (intent, staging_identity, security_metadata) = staged.into_parts();
        let counters = intent.counters();
        if payload.staging_identity() != &staging_identity {
            return Err(BlobPublicationDenial::WalReplayIdentityMismatch { counters });
        }
        let DurablePublicationScope::WalFrame(wal_scope) = durable_publication.scope() else {
            return Err(BlobPublicationDenial::WalPublicationScopeRequired { counters });
        };
        let Some(durable_wal) = replay_classification
            .recovered_or_rejected()
            .replayable_durable_wal()
        else {
            return Err(BlobPublicationDenial::WalReplayEvidenceRequired { counters });
        };
        require_matching_replay_identity(wal_scope, durable_wal, &payload, counters)?;
        Ok(Self {
            intent,
            durable_publication,
            replay_classification_digest: replay_classification.classification_digest().to_owned(),
            replay_counters: replay_classification.counters(),
            staging_identity,
            security_metadata,
        })
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

fn require_matching_replay_identity(
    declared: &WalFrameDurablePublicationScope,
    replayed: &UnacknowledgedDurableWal,
    payload: &BlobPublicationWalPayload,
    counters: BlobPublicationCounterSnapshot,
) -> Result<(), BlobPublicationDenial> {
    let replayed_range = replayed.lsn_range();
    let identities_match = declared.segment_id() == replayed.segment_id().get()
        && declared.generation() == replayed.generation().get()
        && declared.lsn_start() == replayed_range.start().get()
        && declared.lsn_end() == replayed_range.end_exclusive().get()
        && declared.frame_digest() == replayed.frame_digest().as_str()
        && declared.frame_digest() == payload.frame_digest()
        && declared.expected_bytes() == replayed.expected_bytes();
    if identities_match {
        Ok(())
    } else {
        Err(BlobPublicationDenial::WalReplayIdentityMismatch { counters })
    }
}
