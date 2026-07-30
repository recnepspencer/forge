use super::{BlobCompactionBasis, BlobCompactionPacingAdmission};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRootCanonicalBasis,
    BlobCompactionCounterSnapshot, BlobCompactionDenial, BlobCompactionIntent,
};
use worth_store_contracts::StableDigest;
use worth_store_io_scheduler::{BackgroundPacingCounterSnapshot, BackgroundResourceBudget};
use worth_store_physical_isolation::CompactionReadInterlockPlan;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobCompactionRewritePlan {
    basis: BlobCompactionBasis,
    pacing: BlobCompactionPacingAdmission,
    physical: CompactionReadInterlockPlan,
    reachability: BlobChunkReachabilityProofSet,
    placement: AdmittedBlobPlacement,
    old_canonical_basis: BlobChunkRootCanonicalBasis,
    dedupe_reference_identities: Vec<StableDigest>,
    counters: BlobCompactionCounterSnapshot,
}

pub(crate) struct BlobCompactionRewritePlanParts {
    pub(crate) basis: BlobCompactionBasis,
    pub(crate) pacing: BlobCompactionPacingAdmission,
    pub(crate) physical: CompactionReadInterlockPlan,
    pub(crate) reachability: BlobChunkReachabilityProofSet,
    pub(crate) placement: AdmittedBlobPlacement,
    pub(crate) old_canonical_basis: BlobChunkRootCanonicalBasis,
    pub(crate) dedupe_reference_identities: Vec<StableDigest>,
    pub(crate) counters: BlobCompactionCounterSnapshot,
}

impl BlobCompactionRewritePlan {
    pub(crate) fn new(parts: BlobCompactionRewritePlanParts) -> Self {
        Self {
            basis: parts.basis,
            pacing: parts.pacing,
            physical: parts.physical,
            reachability: parts.reachability,
            placement: parts.placement,
            old_canonical_basis: parts.old_canonical_basis,
            dedupe_reference_identities: parts.dedupe_reference_identities,
            counters: parts.counters,
        }
    }

    pub(crate) fn admit(intent: BlobCompactionIntent) -> Result<Self, BlobCompactionDenial> {
        crate::compaction::transitions::admit_rewrite_plan::admit(intent)
    }

    pub const fn counters(&self) -> BlobCompactionCounterSnapshot {
        self.counters
    }

    pub const fn pacing_counters(&self) -> BackgroundPacingCounterSnapshot {
        self.pacing.counters()
    }

    pub const fn pacing_budget(&self) -> BackgroundResourceBudget {
        self.pacing.admitted_budget()
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
