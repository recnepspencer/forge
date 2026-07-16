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
}
