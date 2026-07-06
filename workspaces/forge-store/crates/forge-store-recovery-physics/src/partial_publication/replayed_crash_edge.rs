use super::{
    PartialPublicationCounterSnapshot, PartialPublicationReplayReadArtifact,
    PartialPublicationReplayReadDenial, PartialPublicationReplayReadRecord,
    PartialPublicationReplayReadWitness, UnacknowledgedPublicationOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationReplayedCrashEdge {
    witness: PartialPublicationReplayReadWitness,
    replay_read_recovery_entry_digest: String,
    replay_read_persisted_bytes_digest: String,
    replay_read_identity: String,
}

impl PartialPublicationReplayedCrashEdge {
    pub fn from_replay_read_artifact(
        artifact: PartialPublicationReplayReadArtifact,
    ) -> Result<Self, PartialPublicationReplayReadDenial> {
        let record = PartialPublicationReplayReadRecord::from_replay_read_artifact(artifact);
        let witness = PartialPublicationReplayReadWitness::readmitted_before_wal_append(record)?;
        Ok(Self::from_replay_read_witness(witness))
    }

    pub fn from_replay_read_witness(witness: PartialPublicationReplayReadWitness) -> Self {
        Self {
            replay_read_recovery_entry_digest: witness.recovery_entry_digest().to_owned(),
            replay_read_persisted_bytes_digest: witness.persisted_bytes_digest().to_owned(),
            replay_read_identity: witness.replay_read_identity().to_owned(),
            witness,
        }
    }

    pub fn outcome(&self) -> UnacknowledgedPublicationOutcome {
        self.witness.classification().outcome()
    }

    pub fn classification_digest(&self) -> &str {
        self.witness.classification().classification_digest()
    }

    pub fn before_wal_append_operation_digest(&self) -> Option<&str> {
        self.witness
            .classification()
            .before_wal_append_operation_digest()
    }

    pub fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.witness.counters()
    }

    pub fn replay_read_recovery_entry_digest(&self) -> &str {
        &self.replay_read_recovery_entry_digest
    }

    pub fn replay_read_persisted_bytes_digest(&self) -> &str {
        &self.replay_read_persisted_bytes_digest
    }

    pub fn replay_read_identity(&self) -> &str {
        &self.replay_read_identity
    }
}
