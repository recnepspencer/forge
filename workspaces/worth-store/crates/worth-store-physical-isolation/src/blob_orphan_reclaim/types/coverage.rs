use crate::blob_orphan_reclaim::types::barrier::BlobOrphanReclaimBarrier;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobOrphanReclaimCoverage {
    pub(super) barrier: BlobOrphanReclaimBarrier,
    pub(super) identity: BlobOrphanReclaimIdentity,
    pub(super) reclaim_root_epoch: u64,
    pub(super) reclaim_candidate_ranges: u64,
}

impl BlobOrphanReclaimCoverage {
    pub(crate) fn construct(
        barrier: BlobOrphanReclaimBarrier,
        identity: BlobOrphanReclaimIdentity,
        reclaim_root_epoch: u64,
        reclaim_candidate_ranges: u64,
    ) -> Self {
        Self {
            barrier,
            identity,
            reclaim_root_epoch,
            reclaim_candidate_ranges,
        }
    }

    pub(crate) fn into_proof_parts(
        self,
    ) -> (
        BlobOrphanReclaimBarrier,
        BlobOrphanReclaimIdentity,
        u64,
        u64,
    ) {
        (
            self.barrier,
            self.identity,
            self.reclaim_root_epoch,
            self.reclaim_candidate_ranges,
        )
    }
}
