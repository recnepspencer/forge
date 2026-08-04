use worth_proof::NonEmpty;
use worth_store_physical_format::PhysicalCheckpointIdentity;

use super::super::inventory::PhysicalWalSegmentInventoryEntry;

pub(in crate::physical_runtime::durability) struct EligiblePhysicalWalReclamation {
    checkpoint: PhysicalCheckpointIdentity,
    segments: NonEmpty<EligiblePhysicalWalSegmentReclamation>,
}

pub(in crate::physical_runtime::durability) struct EligiblePhysicalWalSegmentReclamation {
    checkpoint: PhysicalCheckpointIdentity,
    retained_boundary: worth_store_wal::LogSequenceNumber,
    last_copy: ProvenNoLiveBindingLastCopyObligation,
}

struct ProvenNoLiveBindingLastCopyObligation {
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    segment: PhysicalWalSegmentInventoryEntry,
}

impl EligiblePhysicalWalReclamation {
    pub(super) fn new(
        checkpoint: PhysicalCheckpointIdentity,
        compaction_generation: u64,
        compaction_digest: [u8; 32],
        retained_boundary: worth_store_wal::LogSequenceNumber,
        segments: NonEmpty<PhysicalWalSegmentInventoryEntry>,
    ) -> Self {
        Self {
            checkpoint,
            segments: segments.map(|segment| EligiblePhysicalWalSegmentReclamation {
                checkpoint,
                retained_boundary,
                last_copy: ProvenNoLiveBindingLastCopyObligation {
                    compaction_generation,
                    compaction_digest,
                    segment,
                },
            }),
        }
    }

    pub(super) const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub(super) fn into_segments(self) -> NonEmpty<EligiblePhysicalWalSegmentReclamation> {
        self.segments
    }
}

impl EligiblePhysicalWalSegmentReclamation {
    pub(super) const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub(super) const fn compaction_generation(&self) -> u64 {
        self.last_copy.compaction_generation
    }

    pub(super) const fn compaction_digest(&self) -> [u8; 32] {
        self.last_copy.compaction_digest
    }

    pub(super) const fn retained_boundary(&self) -> worth_store_wal::LogSequenceNumber {
        self.retained_boundary
    }

    pub(super) const fn segment(&self) -> PhysicalWalSegmentInventoryEntry {
        self.last_copy.segment
    }
}
