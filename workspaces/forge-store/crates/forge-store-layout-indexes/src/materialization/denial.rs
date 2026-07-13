use super::state::MaterializationStateClass;
use super::watermark::CoverageBasisKind;
use crate::catalog::PhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageGapClass {
    PhysicalRange,
    PrefixScope,
    BasisFrontier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageGapWitness {
    family: PhysicalArtifactFamily,
    basis_kind: CoverageBasisKind,
    class: CoverageGapClass,
    start_inclusive: u64,
    end_exclusive: u64,
}

impl CoverageGapWitness {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        basis_kind: CoverageBasisKind,
        class: CoverageGapClass,
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
        basis_kind: CoverageBasisKind,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self::new(
            family,
            basis_kind,
            CoverageGapClass::PhysicalRange,
            start_inclusive,
            end_exclusive,
        )
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn basis_kind(self) -> CoverageBasisKind {
        self.basis_kind
    }

    pub const fn class(self) -> CoverageGapClass {
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
pub enum MaterializationDenial {
    MaterializationFamilyMismatch,
    ImportedBlobFamilyRequired,
    ImportedBlobSecurityScopeMismatch,
    ImportedBlobStoreAuthorityMismatch,
    BTreeSourceStoreAuthorityMismatch,
    LsmSourceSecurityScopeMismatch,
    LsmSourceStoreAuthorityMismatch,
    RestoreOfflineReadmissionRequired,
    RestoreReplayFrontierRequired,
    RestoreCustodyReadmissionRequired,
    RestoreCurrentStoreAuthorityRequired,
    CoverageSourceMismatch,
    MaterializationFrontierMismatch,
    MaterializationStateDoesNotSupportExactAccess {
        family: PhysicalArtifactFamily,
        state: MaterializationStateClass,
    },
    CoverageBasisDoesNotMatchMaterializationState {
        family: PhysicalArtifactFamily,
        state: MaterializationStateClass,
        basis_kind: CoverageBasisKind,
    },
    CoverageIntervalIsReversed {
        family: PhysicalArtifactFamily,
        basis_kind: CoverageBasisKind,
        lower_bound: u64,
        upper_bound: u64,
    },
    LayoutCoverageIsPartial {
        gap: CoverageGapWitness,
    },
    LayoutCoverageIsStale {
        family: PhysicalArtifactFamily,
        basis_kind: CoverageBasisKind,
    },
    LayoutCoverageIsLagged {
        family: PhysicalArtifactFamily,
        basis_kind: CoverageBasisKind,
    },
    LayoutRangeIsQuarantined {
        gap: CoverageGapWitness,
    },
    LayoutRequiresRebuild {
        family: PhysicalArtifactFamily,
    },
    LayoutIsMigrating {
        family: PhysicalArtifactFamily,
    },
}
