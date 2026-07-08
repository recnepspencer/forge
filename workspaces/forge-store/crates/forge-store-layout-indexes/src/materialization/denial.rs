use super::state::S8MaterializationStateClass;
use super::watermark::S8CoverageBasisKind;
use crate::artifact_family::PhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CoverageGapClass {
    PhysicalRange,
    PrefixScope,
    BasisFrontier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8CoverageGapWitness {
    family: PhysicalArtifactFamily,
    basis_kind: S8CoverageBasisKind,
    class: S8CoverageGapClass,
    start_inclusive: u64,
    end_exclusive: u64,
}

impl S8CoverageGapWitness {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
        class: S8CoverageGapClass,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self {
            family,
            basis_kind,
            class,
            start_inclusive,
            end_exclusive,
        }
    }

    pub(crate) const fn physical_range(
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self::new(
            family,
            basis_kind,
            S8CoverageGapClass::PhysicalRange,
            start_inclusive,
            end_exclusive,
        )
    }

    pub(crate) const fn prefix_scope(
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self::new(
            family,
            basis_kind,
            S8CoverageGapClass::PrefixScope,
            start_inclusive,
            end_exclusive,
        )
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn basis_kind(self) -> S8CoverageBasisKind {
        self.basis_kind
    }

    pub const fn class(self) -> S8CoverageGapClass {
        self.class
    }

    pub const fn start_inclusive(self) -> u64 {
        self.start_inclusive
    }

    pub const fn end_exclusive(self) -> u64 {
        self.end_exclusive
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MaterializationDenial {
    MaterializationStateDoesNotSupportExactAccess {
        family: PhysicalArtifactFamily,
        state: S8MaterializationStateClass,
    },
    CoverageBasisDoesNotMatchMaterializationState {
        family: PhysicalArtifactFamily,
        state: S8MaterializationStateClass,
        basis_kind: S8CoverageBasisKind,
    },
    CoverageIntervalIsReversed {
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
        lower_bound: u64,
        upper_bound: u64,
    },
    LayoutCoverageIsPartial {
        gap: S8CoverageGapWitness,
    },
    LayoutCoverageIsStale {
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
    },
    LayoutCoverageIsLagged {
        family: PhysicalArtifactFamily,
        basis_kind: S8CoverageBasisKind,
    },
    LayoutRangeIsQuarantined {
        gap: S8CoverageGapWitness,
    },
    LayoutRequiresRebuild {
        family: PhysicalArtifactFamily,
    },
    LayoutIsMigrating {
        family: PhysicalArtifactFamily,
    },
    NonExactAbsenceProofRequested {
        family: PhysicalArtifactFamily,
    },
    RangeCompletenessDenied {
        family: PhysicalArtifactFamily,
    },
    PrefixCompletenessDenied {
        family: PhysicalArtifactFamily,
    },
}
