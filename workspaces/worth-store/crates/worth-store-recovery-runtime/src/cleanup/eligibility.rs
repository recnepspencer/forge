use worth_store_recovery_physics::{WalLsnRange, WalSegmentArtifactIdentity};

/// Consuming authority for one exact post-publication WAL removal.
///
/// Only the cleanup plan owner can construct this value. It is deliberately
/// neither `Clone` nor `Copy`; execution drains it once into one Store command.
pub struct RecoveryCleanupEligibility {
    artifact: WalSegmentArtifactIdentity,
    range: WalLsnRange,
    byte_count: u64,
}

impl RecoveryCleanupEligibility {
    pub(super) const fn new(
        artifact: WalSegmentArtifactIdentity,
        range: WalLsnRange,
        byte_count: u64,
    ) -> Self {
        Self {
            artifact,
            range,
            byte_count,
        }
    }

    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
    }

    pub const fn range(&self) -> WalLsnRange {
        self.range
    }

    pub const fn byte_count(&self) -> u64 {
        self.byte_count
    }
}
