use crate::PhysicalArtifactFamily;
use worth_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootstrapOnlyAccessDenied {
    OrdinaryFamilyAccessForbidden {
        family: PhysicalArtifactFamily,
    },
    CurrentRootReadmissionRequired {
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    },
}
