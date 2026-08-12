#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RecoveryObserverConclusion {
    artifact_count: u64,
    bytes_read: u64,
    artifact_set_digest: [u8; 32],
}

impl RecoveryObserverConclusion {
    pub(super) const fn new(
        artifact_count: u64,
        bytes_read: u64,
        artifact_set_digest: [u8; 32],
    ) -> Self {
        Self {
            artifact_count,
            bytes_read,
            artifact_set_digest,
        }
    }

    pub(super) const fn artifact_count(self) -> u64 {
        self.artifact_count
    }

    pub(super) const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub(super) const fn artifact_set_digest(self) -> [u8; 32] {
        self.artifact_set_digest
    }
}
