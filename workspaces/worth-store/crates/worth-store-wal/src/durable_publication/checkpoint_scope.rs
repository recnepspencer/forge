use crate::StoreCheckpointRecordIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckpointDurablePublicationScope {
    checkpoint: StoreCheckpointRecordIdentity,
    manifest_digest: String,
    covered_lsn_start: u64,
    covered_lsn_end: u64,
}

impl CheckpointDurablePublicationScope {
    pub fn new(
        checkpoint: StoreCheckpointRecordIdentity,
        manifest_digest: impl Into<String>,
        covered_lsn_start: u64,
        covered_lsn_end: u64,
    ) -> Option<Self> {
        if covered_lsn_start >= covered_lsn_end {
            return None;
        }
        let manifest_digest = manifest_digest.into();
        if manifest_digest.is_empty() {
            return None;
        }
        Some(Self {
            checkpoint,
            manifest_digest,
            covered_lsn_start,
            covered_lsn_end,
        })
    }

    pub const fn checkpoint(&self) -> StoreCheckpointRecordIdentity {
        self.checkpoint
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub const fn covered_lsn_start(&self) -> u64 {
        self.covered_lsn_start
    }

    pub const fn covered_lsn_end(&self) -> u64 {
        self.covered_lsn_end
    }
}
