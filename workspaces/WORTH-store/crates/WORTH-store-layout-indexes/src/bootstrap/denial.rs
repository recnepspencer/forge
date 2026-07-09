use crate::PhysicalArtifactFamily;
use worth_store_physical_format::{
    PhysicalBootstrapCatalogDenial, PhysicalFormatVersion, PhysicalGenerationOwner,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8BootstrapOnlyAccessDenied {
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
