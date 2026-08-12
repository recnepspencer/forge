#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RecoveryObserverConclusion {
    artifact_set_digest: [u8; 32],
}

impl RecoveryObserverConclusion {
    pub(super) const fn new(artifact_set_digest: [u8; 32]) -> Self {
        Self {
            artifact_set_digest,
        }
    }

    pub(super) const fn artifact_set_digest(self) -> [u8; 32] {
        self.artifact_set_digest
    }
}
