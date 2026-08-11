use worth_store_wal::{WalLsnRange, WalSegmentArtifactIdentity};

use super::PhysicalWalSegmentCandidate;

/// One fully verified WAL artifact whose complete admitted range is covered by
/// the selected checkpoint. This is descriptive recovery truth, not cleanup
/// authority; the post-publication cleanup owner must independently prove the
/// artifact is still removable immediately before the effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointCoveredWalArtifact {
    identity: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
    cleanup_safe: bool,
}

impl CheckpointCoveredWalArtifact {
    pub(super) fn from_candidate(candidate: &PhysicalWalSegmentCandidate) -> Self {
        let interruption = candidate.interrupted_tail();
        Self {
            identity: candidate.identity(),
            lsn_range: candidate.inspection().lsn_range(),
            byte_count: interruption.map_or(candidate.inspection().byte_count(), |tail| {
                tail.observed_bytes()
            }),
            cleanup_safe: interruption.is_none(),
        }
    }

    pub const fn identity(self) -> WalSegmentArtifactIdentity {
        self.identity
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }

    pub const fn cleanup_safe(self) -> bool {
        self.cleanup_safe
    }
}
