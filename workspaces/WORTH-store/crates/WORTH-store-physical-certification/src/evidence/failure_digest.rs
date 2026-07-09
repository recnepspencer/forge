use crate::SimulationReplayBundle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationFailureDigest {
    transcript_digest: [u8; 32],
    failed_oracle_count: usize,
}

impl SimulationFailureDigest {
    pub(crate) fn from_replay_bundle(replay: &SimulationReplayBundle) -> Option<Self> {
        let failed_oracle_count = replay
            .oracle_verdicts()
            .iter()
            .filter(|verdict| verdict.kind() == crate::PhysicalProofOracleVerdictKind::Failed)
            .count();
        if failed_oracle_count == 0 {
            return None;
        }
        Some(Self {
            transcript_digest: *replay.transcript_identity().digest_bytes(),
            failed_oracle_count,
        })
    }

    pub const fn transcript_digest(&self) -> &[u8; 32] {
        &self.transcript_digest
    }

    pub const fn failed_oracle_count(&self) -> usize {
        self.failed_oracle_count
    }
}
