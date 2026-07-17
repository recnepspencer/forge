use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity, ReplicationPeerId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaHistoryClassification {
    AttestedContinuation,
    ReplayDerivedContinuation,
    PartialContinuation,
    Divergent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaHistoryObservation {
    pub(crate) peer: ReplicationPeerId,
    pub(crate) lineage: ReplicationLineageIdentity,
    pub(crate) frontier: ReplicaRecoveryFrontier,
    pub(crate) blob_closure_complete: bool,
    pub(crate) authoritative_media_admissible: bool,
    pub(crate) durable_media_identity: [u8; 32],
}

impl ReplicaHistoryObservation {
    pub fn peer(&self) -> &ReplicationPeerId {
        &self.peer
    }

    pub fn lineage(&self) -> &ReplicationLineageIdentity {
        &self.lineage
    }

    pub const fn frontier(&self) -> ReplicaRecoveryFrontier {
        self.frontier
    }

    pub const fn blob_closure_complete(&self) -> bool {
        self.blob_closure_complete
    }

    pub const fn authoritative_media_admissible(&self) -> bool {
        self.authoritative_media_admissible
    }

    pub const fn durable_media_identity(&self) -> [u8; 32] {
        self.durable_media_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivergentReplicaHistoryReport {
    observation: ReplicaHistoryObservation,
    current_lineage: ReplicationLineageIdentity,
    classification: ReplicaHistoryClassification,
}

impl DivergentReplicaHistoryReport {
    pub(crate) fn classify(
        observation: ReplicaHistoryObservation,
        current_lineage: ReplicationLineageIdentity,
    ) -> Self {
        let classification = if observation.lineage != current_lineage {
            ReplicaHistoryClassification::Divergent
        } else if !observation.authoritative_media_admissible
            || !observation.blob_closure_complete
            || observation.durable_media_identity == [0; 32]
        {
            ReplicaHistoryClassification::PartialContinuation
        } else {
            ReplicaHistoryClassification::AttestedContinuation
        };
        Self {
            observation,
            current_lineage,
            classification,
        }
    }

    pub const fn observation(&self) -> &ReplicaHistoryObservation {
        &self.observation
    }

    pub const fn classification(&self) -> ReplicaHistoryClassification {
        self.classification
    }

    pub fn current_lineage(&self) -> &ReplicationLineageIdentity {
        &self.current_lineage
    }

    pub fn stable_fingerprint(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut digest = Sha256::new();
        digest.update(b"worth-store-divergent-replica-history-v1");
        digest.update(self.observation.peer.as_str().as_bytes());
        digest.update(self.observation.lineage.stable_fingerprint());
        digest.update(self.current_lineage.stable_fingerprint());
        let frontier = self.observation.frontier;
        digest.update(frontier.observed_lsn().to_be_bytes());
        digest.update(frontier.durable_lsn().to_be_bytes());
        digest.update(frontier.client_acknowledged_lsn().to_be_bytes());
        digest.update(frontier.replication_acknowledged_lsn().to_be_bytes());
        digest.update(frontier.authority_epoch().to_be_bytes());
        digest.update([
            u8::from(self.observation.blob_closure_complete),
            u8::from(self.observation.authoritative_media_admissible),
            self.classification as u8,
        ]);
        digest.update(self.observation.durable_media_identity);
        digest.finalize().into()
    }
}
