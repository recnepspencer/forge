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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaterializationDenialKind {
    MaterializationFamilyMismatch,
    ImportedBlobFamilyRequired,
    ImportedBlobSecurityScopeMismatch,
    ImportedBlobStoreAuthorityMismatch,
    BTreeSourceStoreAuthorityMismatch,
    LsmSourceSecurityScopeMismatch,
    LsmSourceStoreAuthorityMismatch,
    CoverageSourceMismatch,
    MaterializationFrontierMismatch,
    MaterializationStateDoesNotSupportExactAccess,
    CoverageBasisDoesNotMatchMaterializationState,
    CoverageIntervalIsReversed,
    LayoutCoverageIsPartial,
    LayoutCoverageIsStale,
    LayoutCoverageIsLagged,
    LayoutRangeIsQuarantined,
    LayoutRequiresRebuild,
    LayoutIsMigrating,
}

impl MaterializationDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaterializationFamilyMismatch => "denied.materialization_family_mismatch",
            Self::ImportedBlobFamilyRequired => "denied.imported_blob_family_required",
            Self::ImportedBlobSecurityScopeMismatch => "denied.imported_blob_security_scope",
            Self::ImportedBlobStoreAuthorityMismatch => "denied.imported_blob_store_authority",
            Self::BTreeSourceStoreAuthorityMismatch => "denied.btree_source_store_authority",
            Self::LsmSourceSecurityScopeMismatch => "denied.lsm_source_security_scope",
            Self::LsmSourceStoreAuthorityMismatch => "denied.lsm_source_store_authority",
            Self::CoverageSourceMismatch => "denied.coverage_source",
            Self::MaterializationFrontierMismatch => "denied.materialization_frontier",
            Self::MaterializationStateDoesNotSupportExactAccess => "denied.materialization_state",
            Self::CoverageBasisDoesNotMatchMaterializationState => "denied.coverage_basis",
            Self::CoverageIntervalIsReversed => "denied.coverage_interval_reversed",
            Self::LayoutCoverageIsPartial => "denied.coverage_partial",
            Self::LayoutCoverageIsStale => "denied.coverage_stale",
            Self::LayoutCoverageIsLagged => "denied.coverage_lagged",
            Self::LayoutRangeIsQuarantined => "denied.range_quarantined",
            Self::LayoutRequiresRebuild => "denied.rebuild_required",
            Self::LayoutIsMigrating => "denied.migrating",
        }
    }
}

impl MaterializationDenial {
    pub const fn kind(self) -> MaterializationDenialKind {
        match self {
            Self::MaterializationFamilyMismatch => {
                MaterializationDenialKind::MaterializationFamilyMismatch
            }
            Self::ImportedBlobFamilyRequired => {
                MaterializationDenialKind::ImportedBlobFamilyRequired
            }
            Self::ImportedBlobSecurityScopeMismatch => {
                MaterializationDenialKind::ImportedBlobSecurityScopeMismatch
            }
            Self::ImportedBlobStoreAuthorityMismatch => {
                MaterializationDenialKind::ImportedBlobStoreAuthorityMismatch
            }
            Self::BTreeSourceStoreAuthorityMismatch => {
                MaterializationDenialKind::BTreeSourceStoreAuthorityMismatch
            }
            Self::LsmSourceSecurityScopeMismatch => {
                MaterializationDenialKind::LsmSourceSecurityScopeMismatch
            }
            Self::LsmSourceStoreAuthorityMismatch => {
                MaterializationDenialKind::LsmSourceStoreAuthorityMismatch
            }
            Self::CoverageSourceMismatch => MaterializationDenialKind::CoverageSourceMismatch,
            Self::MaterializationFrontierMismatch => {
                MaterializationDenialKind::MaterializationFrontierMismatch
            }
            Self::MaterializationStateDoesNotSupportExactAccess { .. } => {
                MaterializationDenialKind::MaterializationStateDoesNotSupportExactAccess
            }
            Self::CoverageBasisDoesNotMatchMaterializationState { .. } => {
                MaterializationDenialKind::CoverageBasisDoesNotMatchMaterializationState
            }
            Self::CoverageIntervalIsReversed { .. } => {
                MaterializationDenialKind::CoverageIntervalIsReversed
            }
            Self::LayoutCoverageIsPartial { .. } => {
                MaterializationDenialKind::LayoutCoverageIsPartial
            }
            Self::LayoutCoverageIsStale { .. } => MaterializationDenialKind::LayoutCoverageIsStale,
            Self::LayoutCoverageIsLagged { .. } => {
                MaterializationDenialKind::LayoutCoverageIsLagged
            }
            Self::LayoutRangeIsQuarantined { .. } => {
                MaterializationDenialKind::LayoutRangeIsQuarantined
            }
            Self::LayoutRequiresRebuild { .. } => MaterializationDenialKind::LayoutRequiresRebuild,
            Self::LayoutIsMigrating { .. } => MaterializationDenialKind::LayoutIsMigrating,
        }
    }
}
