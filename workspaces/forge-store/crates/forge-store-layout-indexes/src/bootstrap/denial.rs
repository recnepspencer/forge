use crate::PhysicalArtifactFamily;
use forge_store_physical_format::{
    PhysicalBootstrapCatalogDenial, PhysicalFormatVersion, PhysicalGenerationOwner,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootstrapOnlyAccessDenied {
    OrdinaryFamilyAccessForbidden {
        family: PhysicalArtifactFamily,
    },
    PhysicalBootstrapDenied(PhysicalBootstrapCatalogDenial),
    CurrentRootReadmissionRequired {
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    },
    BootstrapPathVersionMismatch {
        expected: PhysicalFormatVersion,
        actual: PhysicalFormatVersion,
    },
}
