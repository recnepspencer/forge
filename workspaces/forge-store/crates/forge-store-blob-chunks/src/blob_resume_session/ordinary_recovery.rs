use super::{
    BlobResumeCheckpoint, BlobResumeDenial, BlobResumeReadmissionAuthority, BlobResumeReplay,
    BlobResumeReplayOutcome,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobInterruptedIngestRecovery {
    outcome: BlobResumeReplayOutcome,
}

impl BlobInterruptedIngestRecovery {
    pub fn from_persisted_checkpoint(
        checkpoint: BlobResumeCheckpoint,
        authority: BlobResumeReadmissionAuthority,
    ) -> Result<Self, BlobResumeDenial> {
        Ok(Self {
            outcome: BlobResumeReplay::readmit_checkpoint(checkpoint, authority)?,
        })
    }

    pub const fn outcome(&self) -> &BlobResumeReplayOutcome {
        &self.outcome
    }
}
