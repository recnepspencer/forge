use crate::materialization::{S8CoverageBasisKind, S8LayoutCoverageWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8DerivedIndexPartialKeySpace {
    basis_kind: S8CoverageBasisKind,
    lower_bound: u64,
    upper_bound: u64,
}

impl S8DerivedIndexPartialKeySpace {
    pub(crate) const fn from_coverage(coverage: S8LayoutCoverageWitness) -> Self {
        Self {
            basis_kind: coverage.lower_bound().basis_kind(),
            lower_bound: coverage.lower_bound().start_inclusive(),
            upper_bound: coverage.upper_bound().value(),
        }
    }

    pub const fn basis_kind(self) -> S8CoverageBasisKind {
        self.basis_kind
    }

    pub const fn lower_bound(self) -> u64 {
        self.lower_bound
    }

    pub const fn upper_bound(self) -> u64 {
        self.upper_bound
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8DerivedIndexRebuildScope {
    authority_coverage: S8LayoutCoverageWitness,
    partial_key_space: S8DerivedIndexPartialKeySpace,
}

impl S8DerivedIndexRebuildScope {
    pub(crate) const fn from_coverage(coverage: S8LayoutCoverageWitness) -> Self {
        Self {
            authority_coverage: coverage,
            partial_key_space: S8DerivedIndexPartialKeySpace::from_coverage(coverage),
        }
    }

    pub const fn authority_coverage(self) -> S8LayoutCoverageWitness {
        self.authority_coverage
    }

    pub const fn partial_key_space(self) -> S8DerivedIndexPartialKeySpace {
        self.partial_key_space
    }
}
