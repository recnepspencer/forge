use super::BlobCompactionBasis;
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRootCanonicalBasis,
    BlobCompactionCounterSnapshot, BlobCompactionDenial, BlobCompactionIntent,
};
use forge_store_contracts::StableDigest;
use forge_store_physical_isolation::CompactionReadInterlockPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionRewritePlan {
    basis: BlobCompactionBasis,
    physical: CompactionReadInterlockPlan,
    reachability: BlobChunkReachabilityProofSet,
    placement: AdmittedBlobPlacement,
    old_canonical_basis: BlobChunkRootCanonicalBasis,
    dedupe_reference_identities: Vec<StableDigest>,
    counters: BlobCompactionCounterSnapshot,
}

impl BlobCompactionRewritePlan {
    pub(crate) fn new(
        basis: BlobCompactionBasis,
        physical: CompactionReadInterlockPlan,
        reachability: BlobChunkReachabilityProofSet,
        placement: AdmittedBlobPlacement,
        old_canonical_basis: BlobChunkRootCanonicalBasis,
        dedupe_reference_identities: Vec<StableDigest>,
        counters: BlobCompactionCounterSnapshot,
    ) -> Self {
        Self {
            basis,
            physical,
            reachability,
            placement,
            old_canonical_basis,
            dedupe_reference_identities,
            counters,
        }
    }

    pub(crate) fn admit(intent: BlobCompactionIntent) -> Result<Self, BlobCompactionDenial> {
        crate::compaction::transitions::admit_rewrite_plan::admit(intent)
    }

    pub const fn counters(&self) -> BlobCompactionCounterSnapshot {
        self.counters
    }

    pub const fn old_root(&self) -> &crate::ChunkTreeRoot {
        self.basis.old_root()
    }

    pub(crate) const fn basis(&self) -> &BlobCompactionBasis {
        &self.basis
    }

    pub const fn physical(&self) -> &CompactionReadInterlockPlan {
        &self.physical
    }

    pub const fn reachability(&self) -> &BlobChunkReachabilityProofSet {
        &self.reachability
    }

    pub const fn placement(&self) -> &AdmittedBlobPlacement {
        &self.placement
    }

    pub const fn old_canonical_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.old_canonical_basis
    }

    pub fn dedupe_reference_identities(&self) -> &[StableDigest] {
        &self.dedupe_reference_identities
    }
}
