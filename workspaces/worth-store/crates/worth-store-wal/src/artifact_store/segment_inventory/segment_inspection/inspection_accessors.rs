use super::WalSegmentInspection;
use crate::artifact_store::WalSegmentArtifactIdentity;
use crate::WalLsnRange;

impl WalSegmentInspection {
    pub const fn identity(self) -> WalSegmentArtifactIdentity {
        self.identity
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn frame_count(self) -> u64 {
        self.frame_count
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }

    pub const fn artifact_digest(self) -> [u8; 32] {
        self.artifact_digest
    }
}
