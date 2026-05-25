use forge_foundational::facade::{
    CanonicalComparisonOutcome, CanonicalEquivalentBasis, CanonicalMismatchBasis,
};

#[derive(Clone, Debug)]
pub struct ForgeQueryCanonicalDeclarationComparison {
    outcome: CanonicalComparisonOutcome,
}

impl ForgeQueryCanonicalDeclarationComparison {
    pub(crate) fn new(outcome: CanonicalComparisonOutcome) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &CanonicalComparisonOutcome {
        &self.outcome
    }

    pub fn equivalent_basis(&self) -> Option<&CanonicalEquivalentBasis> {
        match &self.outcome {
            CanonicalComparisonOutcome::Equivalent(equivalent) => Some(equivalent),
            CanonicalComparisonOutcome::Mismatched(_)
            | CanonicalComparisonOutcome::Unsupported(_) => None,
        }
    }

    pub fn mismatch_basis(&self) -> Option<&CanonicalMismatchBasis> {
        match &self.outcome {
            CanonicalComparisonOutcome::Mismatched(mismatch) => Some(mismatch),
            CanonicalComparisonOutcome::Equivalent(_)
            | CanonicalComparisonOutcome::Unsupported(_) => None,
        }
    }

    pub fn unsupported_basis(&self) -> Option<&CanonicalMismatchBasis> {
        match &self.outcome {
            CanonicalComparisonOutcome::Unsupported(mismatch) => Some(mismatch),
            CanonicalComparisonOutcome::Equivalent(_)
            | CanonicalComparisonOutcome::Mismatched(_) => None,
        }
    }
}
