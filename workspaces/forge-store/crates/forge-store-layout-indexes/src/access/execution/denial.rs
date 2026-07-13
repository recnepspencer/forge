use super::DegradedScanLoweringBasis;
use crate::catalog::PhysicalArtifactFamily;
use crate::materialization::LayoutCoverageWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalDegradedExecutionDenial {
    StoreAuthorityMismatch {
        expected: forge_store_authority::StoreCurrentAuthorityIdentity,
        actual: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
    Admission(forge_store_physical_format::PlatformPhysicalOperationAdmissionDenial),
    Physical(forge_store_physical_format::PlatformPhysicalFacadeDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DegradedScanAdmissionDenied {
    ReadmissionWitnessMismatch {
        basis: DegradedScanLoweringBasis,
        expected: DegradedScanLoweringBasis,
        actual: DegradedScanLoweringBasis,
    },
    LifecycleFamilyMismatch {
        basis: DegradedScanLoweringBasis,
        expected: PhysicalArtifactFamily,
        actual: PhysicalArtifactFamily,
    },
    ArtifactFamilyAuthorityMismatch {
        basis: DegradedScanLoweringBasis,
        expected_security: forge_store_security::StoreSecurityScopeIdentity,
        actual_security: forge_store_security::StoreSecurityScopeIdentity,
        expected_store: forge_store_authority::StoreCurrentAuthorityIdentity,
        actual_store: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
    CurrentCoverageMismatch {
        basis: DegradedScanLoweringBasis,
        expected: LayoutCoverageWitness,
        actual: LayoutCoverageWitness,
    },
    ReadmissionCurrentCoverageMismatch {
        basis: DegradedScanLoweringBasis,
        expected: LayoutCoverageWitness,
        actual: LayoutCoverageWitness,
    },
}

impl DegradedScanAdmissionDenied {
    pub const fn basis(&self) -> &DegradedScanLoweringBasis {
        match self {
            Self::ReadmissionWitnessMismatch { basis, .. }
            | Self::LifecycleFamilyMismatch { basis, .. }
            | Self::ArtifactFamilyAuthorityMismatch { basis, .. }
            | Self::CurrentCoverageMismatch { basis, .. }
            | Self::ReadmissionCurrentCoverageMismatch { basis, .. } => basis,
        }
    }
}
