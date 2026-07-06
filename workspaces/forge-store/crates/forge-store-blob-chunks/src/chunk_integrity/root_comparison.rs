use forge_foundational::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis,
};
use forge_proof::TransitionOutcome;

use crate::{
    BlobChunkRootCanonicalBasis, BlobChunkRootCounterSnapshot, BlobChunkRootPublication,
    BlobChunkRootPublicationDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkRootCanonicalComparison {
    outcome: CanonicalComparisonOutcome,
    left_basis: BlobChunkRootCanonicalBasis,
    right_basis: BlobChunkRootCanonicalBasis,
    counters: BlobChunkRootCounterSnapshot,
}

impl BlobChunkRootCanonicalComparison {
    pub fn compare(
        left: &BlobChunkRootPublication,
        right: &BlobChunkRootPublication,
    ) -> Result<Self, BlobChunkRootPublicationDenial> {
        let left_basis = left.canonical_basis().clone();
        let right_basis = right.canonical_basis().clone();
        let ready = match prepare_canonical_comparison(
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
            left_basis.ready_basis().clone(),
            right_basis.ready_basis().clone(),
        ) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(_) => {
                return Err(
                    BlobChunkRootPublicationDenial::CanonicalComparisonPreparationDenied {
                        counters: comparison_counters().record_denial(),
                    },
                )
            }
        };
        let outcome = compare_canonical_basis(&ready);
        Ok(Self {
            outcome,
            left_basis,
            right_basis,
            counters: comparison_counters(),
        })
    }

    pub const fn outcome(&self) -> &CanonicalComparisonOutcome {
        &self.outcome
    }

    pub const fn is_equivalent(&self) -> bool {
        matches!(self.outcome, CanonicalComparisonOutcome::Equivalent(_))
    }

    pub const fn left_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.left_basis
    }

    pub const fn right_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.right_basis
    }

    pub const fn counters(&self) -> BlobChunkRootCounterSnapshot {
        self.counters
    }

    pub(crate) fn matches_candidate_identities(
        &self,
        left: &crate::BlobChunkIdentity,
        right: &crate::BlobChunkIdentity,
    ) -> bool {
        self.left_basis.contains_chunk_identity(left)
            && self.right_basis.contains_chunk_identity(right)
    }
}

const fn comparison_counters() -> BlobChunkRootCounterSnapshot {
    BlobChunkRootCounterSnapshot::start().record_canonical_comparison()
}
