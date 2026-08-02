use worth_store_physical_format::PhysicalCheckpointIdentity;
use worth_store_wal::{LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity};

/// Exact proof-bound target for one obsolete WAL artifact removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalWalReclamationScope {
    checkpoint: PhysicalCheckpointIdentity,
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
    segment: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
}

impl PhysicalWalReclamationScope {
    pub(in crate::physical_runtime) fn new(
        checkpoint: PhysicalCheckpointIdentity,
        compaction_generation: u64,
        compaction_digest: [u8; 32],
        retained_boundary: LogSequenceNumber,
        segment: WalSegmentArtifactIdentity,
        lsn_range: WalLsnRange,
        byte_count: u64,
    ) -> Option<Self> {
        (compaction_generation != 0
            && byte_count != 0
            && lsn_range.end_exclusive() <= retained_boundary)
            .then_some(Self {
                checkpoint,
                compaction_generation,
                compaction_digest,
                retained_boundary,
                segment,
                lsn_range,
                byte_count,
            })
    }

    pub(in crate::physical_runtime) const fn checkpoint(self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub(in crate::physical_runtime) const fn compaction_generation(self) -> u64 {
        self.compaction_generation
    }

    pub(in crate::physical_runtime) const fn compaction_digest(self) -> [u8; 32] {
        self.compaction_digest
    }

    pub(in crate::physical_runtime) const fn retained_boundary(self) -> LogSequenceNumber {
        self.retained_boundary
    }

    pub(in crate::physical_runtime) const fn segment(self) -> WalSegmentArtifactIdentity {
        self.segment
    }

    pub(in crate::physical_runtime) const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub(in crate::physical_runtime) const fn byte_count(self) -> u64 {
        self.byte_count
    }
}
