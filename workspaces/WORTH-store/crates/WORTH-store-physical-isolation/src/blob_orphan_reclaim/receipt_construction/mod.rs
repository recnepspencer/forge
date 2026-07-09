use crate::blob_orphan_reclaim::types::barrier::BlobOrphanReclaimBarrier;
use crate::blob_orphan_reclaim::types::coverage::BlobOrphanReclaimCoverage;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;
use crate::blob_orphan_reclaim::types::proof::BlobOrphanReclaimProof;
use crate::{ReclaimReachabilityRemovalReceipt};

pub(crate) fn construct_coverage(
    barrier: BlobOrphanReclaimBarrier,
    identity: BlobOrphanReclaimIdentity,
    receipt: &ReclaimReachabilityRemovalReceipt,
) -> BlobOrphanReclaimCoverage {
    BlobOrphanReclaimCoverage::construct(
        barrier,
        identity,
        receipt.evidence().root_epoch().get(),
        receipt.evidence().candidates().candidate_ranges().len() as u64,
    )
}

pub fn construct_proof_from_coverage(coverage: BlobOrphanReclaimCoverage) -> BlobOrphanReclaimProof {
    let (barrier, identity, reclaim_root_epoch, reclaim_candidate_ranges) =
        coverage.into_proof_parts();
    let counters = barrier.counters().with_proof();
    BlobOrphanReclaimProof::construct(
        barrier,
        identity,
        reclaim_root_epoch,
        reclaim_candidate_ranges,
        counters,
    )
}