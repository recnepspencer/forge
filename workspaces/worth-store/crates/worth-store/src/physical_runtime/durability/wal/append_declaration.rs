use worth_store_physical_backend::ArtifactAppendRange;
use worth_store_wal::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use crate::physical_runtime::PhysicalWalFrameWriteDisposition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalAppendDeclaration {
    segment: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    artifact_range: ArtifactAppendRange,
    payload_digest: [u8; 32],
    disposition: PhysicalWalFrameWriteDisposition,
}

impl PhysicalWalAppendDeclaration {
    pub(in crate::physical_runtime) const fn new(
        segment: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        artifact_range: ArtifactAppendRange,
        payload_digest: [u8; 32],
        disposition: PhysicalWalFrameWriteDisposition,
    ) -> Self {
        Self {
            segment,
            generation,
            lsn_range,
            artifact_range,
            payload_digest,
            disposition,
        }
    }

    pub const fn segment(self) -> WalSegmentId {
        self.segment
    }

    pub const fn generation(self) -> WalSegmentGeneration {
        self.generation
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn artifact_range(self) -> ArtifactAppendRange {
        self.artifact_range
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn disposition(self) -> PhysicalWalFrameWriteDisposition {
        self.disposition
    }
}
