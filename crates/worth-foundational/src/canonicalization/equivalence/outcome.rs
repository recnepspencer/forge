use super::basis::CanonicalEquivalenceBasis;
use super::readiness::CanonicalComparisonInput;
use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalMismatchBasis, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalComparisonOutcome {
    Equivalent(CanonicalEquivalentBasis),
    Mismatched(CanonicalMismatchBasis),
    Unsupported(CanonicalMismatchBasis),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalEquivalentBasis {
    equivalence_basis: CanonicalEquivalenceBasis,
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    domain: CanonicalBasisDomain,
    entry_count: u32,
}

impl CanonicalEquivalentBasis {
    pub(super) fn new(input: &CanonicalComparisonInput) -> Self {
        Self {
            equivalence_basis: input.equivalence_basis(),
            left_version: input.left().payload().version().clone(),
            right_version: input.right().payload().version().clone(),
            domain: input.left().payload().domain(),
            entry_count: input.left().payload().entries().len() as u32,
        }
    }

    pub const fn equivalence_basis(&self) -> CanonicalEquivalenceBasis {
        self.equivalence_basis
    }

    pub fn left_version(&self) -> &CanonicalizationRuleVersion {
        &self.left_version
    }

    pub fn right_version(&self) -> &CanonicalizationRuleVersion {
        &self.right_version
    }

    pub const fn domain(&self) -> CanonicalBasisDomain {
        self.domain
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }
}
