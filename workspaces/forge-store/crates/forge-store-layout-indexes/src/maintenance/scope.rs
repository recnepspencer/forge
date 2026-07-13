use crate::materialization::{CoverageBasisKind, LayoutCoverageWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedIndexPartialKeySpace {
    basis_kind: CoverageBasisKind,
    lower_bound: u64,
    upper_bound: u64,
}

impl DerivedIndexPartialKeySpace {
    pub(crate) fn from_coverage(coverage: &LayoutCoverageWitness) -> Self {
        Self {
            basis_kind: coverage.lower_bound().basis_kind(),
            lower_bound: coverage.lower_bound().start_inclusive(),
            upper_bound: coverage.upper_bound().value(),
        }
    }

    pub const fn basis_kind(self) -> CoverageBasisKind {
        self.basis_kind
    }

    pub const fn lower_bound(self) -> u64 {
        self.lower_bound
    }

    pub const fn upper_bound(self) -> u64 {
        self.upper_bound
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexRebuildScope {
    authority_coverage: LayoutCoverageWitness,
    partial_key_space: DerivedIndexPartialKeySpace,
}

impl DerivedIndexRebuildScope {
    pub(crate) fn from_coverage(coverage: LayoutCoverageWitness) -> Self {
        Self {
            partial_key_space: DerivedIndexPartialKeySpace::from_coverage(&coverage),
            authority_coverage: coverage,
        }
    }

    pub const fn authority_coverage(&self) -> &LayoutCoverageWitness {
        &self.authority_coverage
    }

    pub const fn partial_key_space(&self) -> DerivedIndexPartialKeySpace {
        self.partial_key_space
    }
}
