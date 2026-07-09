use crate::blob_orphan_reclaim::counters::BlobOrphanReclaimCounterSnapshot;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;
use crate::blob_orphan_reclaim::types::partial_orphan::BlobPartialChunkOrphan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimBarrier {
    pub(super) orphan: BlobPartialChunkOrphan,
    pub(super) counters: BlobOrphanReclaimCounterSnapshot,
}

impl BlobOrphanReclaimBarrier {
    pub(crate) fn construct(
        orphan: BlobPartialChunkOrphan,
        counters: BlobOrphanReclaimCounterSnapshot,
    ) -> Self {
        Self { orphan, counters }
    }

    pub const fn orphan(&self) -> &BlobPartialChunkOrphan {
        &self.orphan
    }

    pub fn reclaim_identity(&self) -> BlobOrphanReclaimIdentity {
        self.orphan.reclaim_identity()
    }

    pub const fn counters(&self) -> BlobOrphanReclaimCounterSnapshot {
        self.counters
    }
}