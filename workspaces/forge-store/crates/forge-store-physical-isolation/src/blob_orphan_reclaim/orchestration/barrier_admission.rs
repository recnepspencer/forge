use crate::blob_orphan_reclaim::denial::BlobOrphanReclaimDenial;
use crate::blob_orphan_reclaim::receipt_construction::construct_proof_from_coverage;
use crate::blob_orphan_reclaim::transitions::admit_barrier::transition_admit_barrier;
use crate::blob_orphan_reclaim::transitions::admit_coverage::transition_admit_coverage;
use crate::blob_orphan_reclaim::types::{
    BlobOrphanReclaimBarrier, BlobOrphanReclaimCoverage, BlobOrphanReclaimProof,
    BlobPartialChunkOrphan,
};
use crate::ReclaimEligibilityProof;

impl BlobOrphanReclaimBarrier {
    pub fn from_unreached_orphan(
        orphan: BlobPartialChunkOrphan,
        reachable: bool,
    ) -> Result<Self, BlobOrphanReclaimDenial> {
        transition_admit_barrier(orphan, reachable)
    }

    pub fn admit_reclaim_coverage(
        self,
        reclaim_eligibility: ReclaimEligibilityProof,
    ) -> Result<BlobOrphanReclaimCoverage, BlobOrphanReclaimDenial> {
        transition_admit_coverage(self, reclaim_eligibility)
    }
}

impl BlobOrphanReclaimProof {
    pub fn from_reclaim_coverage(coverage: BlobOrphanReclaimCoverage) -> Self {
        construct_proof_from_coverage(coverage)
    }
}
