use crate::blob_orphan_reclaim::counters::BlobOrphanReclaimCounterSnapshot;
use crate::blob_orphan_reclaim::types::barrier::BlobOrphanReclaimBarrier;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimProof {
    pub(super) barrier: BlobOrphanReclaimBarrier,
    pub(super) identity: BlobOrphanReclaimIdentity,
    pub(super) reclaim_root_epoch: u64,
    pub(super) reclaim_candidate_ranges: u64,
    pub(super) counters: BlobOrphanReclaimCounterSnapshot,
}

impl BlobOrphanReclaimProof {
    pub(crate) fn construct(
        barrier: BlobOrphanReclaimBarrier,
        identity: BlobOrphanReclaimIdentity,
        reclaim_root_epoch: u64,
        reclaim_candidate_ranges: u64,
        counters: BlobOrphanReclaimCounterSnapshot,
    ) -> Self {
        Self {
            barrier,
            identity,
            reclaim_root_epoch,
            reclaim_candidate_ranges,
            counters,
        }
    }

    pub const fn barrier(&self) -> &BlobOrphanReclaimBarrier {
        &self.barrier
    }

    pub const fn identity(&self) -> &BlobOrphanReclaimIdentity {
        &self.identity
    }

    pub const fn reclaim_root_epoch(&self) -> u64 {
        self.reclaim_root_epoch
    }

    pub const fn reclaim_candidate_ranges(&self) -> u64 {
        self.reclaim_candidate_ranges
    }

    pub const fn counters(&self) -> BlobOrphanReclaimCounterSnapshot {
        self.counters
    }
}
