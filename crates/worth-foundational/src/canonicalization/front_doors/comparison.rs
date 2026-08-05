use worth_proof::TransitionOutcome;

use super::super::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalBasisReadyArtifact,
    CanonicalComparisonOutcome, CanonicalComparisonReadyArtifact, CanonicalEquivalenceBasis,
    CanonicalEquivalentBasis, CanonicalMismatchBasis,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalComparisonFrontDoor;

impl CanonicalComparisonFrontDoor {
    pub fn left(self, left: CanonicalBasisReadyArtifact) -> CanonicalComparisonRightStep {
        CanonicalComparisonRightStep { left }
    }

    pub fn evaluate(self, ready: &CanonicalComparisonReadyArtifact) -> CanonicalComparisonOutcome {
        compare_canonical_basis(ready)
    }

    pub fn equivalent_basis(
        self,
        outcome: &CanonicalComparisonOutcome,
    ) -> Option<&CanonicalEquivalentBasis> {
        match outcome {
            CanonicalComparisonOutcome::Equivalent(equivalent) => Some(equivalent),
            CanonicalComparisonOutcome::Mismatched(_)
            | CanonicalComparisonOutcome::Unsupported(_) => None,
        }
    }

    pub fn mismatch_basis(
        self,
        outcome: &CanonicalComparisonOutcome,
    ) -> Option<&CanonicalMismatchBasis> {
        match outcome {
            CanonicalComparisonOutcome::Mismatched(mismatch) => Some(mismatch),
            CanonicalComparisonOutcome::Equivalent(_)
            | CanonicalComparisonOutcome::Unsupported(_) => None,
        }
    }

    pub fn unsupported_basis(
        self,
        outcome: &CanonicalComparisonOutcome,
    ) -> Option<&CanonicalMismatchBasis> {
        match outcome {
            CanonicalComparisonOutcome::Unsupported(mismatch) => Some(mismatch),
            CanonicalComparisonOutcome::Equivalent(_)
            | CanonicalComparisonOutcome::Mismatched(_) => None,
        }
    }
}

pub struct CanonicalComparisonRightStep {
    left: CanonicalBasisReadyArtifact,
}

impl CanonicalComparisonRightStep {
    pub fn right(self, right: CanonicalBasisReadyArtifact) -> CanonicalComparisonBasisStep {
        CanonicalComparisonBasisStep {
            left: self.left,
            right,
        }
    }
}

pub struct CanonicalComparisonBasisStep {
    left: CanonicalBasisReadyArtifact,
    right: CanonicalBasisReadyArtifact,
}

impl CanonicalComparisonBasisStep {
    pub fn under(
        self,
        equivalence_basis: CanonicalEquivalenceBasis,
    ) -> TransitionOutcome<CanonicalComparisonReadyArtifact> {
        prepare_canonical_comparison(equivalence_basis, self.left, self.right)
    }
}
