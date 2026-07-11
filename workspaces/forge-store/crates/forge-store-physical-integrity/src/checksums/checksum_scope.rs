use crate::ChecksumAlgorithmMismatchDenial;
use forge_store_physical_format::{
    ChecksumCoverageMap, PhysicalFormatIdentity, PhysicalFormatVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumScopeDeclaration {
    physical_format_identity: PhysicalFormatIdentity,
    coverage_map: ChecksumCoverageMap,
}

impl ChecksumScopeDeclaration {
    pub fn for_physical_format(
        physical_format_identity: PhysicalFormatIdentity,
        coverage_map: ChecksumCoverageMap,
    ) -> Result<Self, ChecksumAlgorithmMismatchDenial> {
        if physical_format_identity.version() != coverage_map.physical_format_version() {
            return Err(ChecksumAlgorithmMismatchDenial::ScopeFormatVersionMismatch);
        }
        if coverage_map.physical_format_version() != PhysicalFormatVersion::s1_initial() {
            return Err(ChecksumAlgorithmMismatchDenial::ScopeFormatVersionMismatch);
        }
        Ok(Self {
            physical_format_identity,
            coverage_map,
        })
    }

    pub const fn physical_format_identity(&self) -> PhysicalFormatIdentity {
        self.physical_format_identity
    }

    pub const fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.physical_format_identity.version()
    }

    pub fn coverage_map(&self) -> &ChecksumCoverageMap {
        &self.coverage_map
    }
}
