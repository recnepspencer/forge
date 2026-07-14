use crate::PhysicalArtifactFamily;

use super::denial::BootstrapOnlyAccessDenied;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCatalogAccess;

impl BootstrapCatalogAccess {
    pub const fn new() -> Self {
        Self
    }

    pub const fn deny_ordinary_family_access(
        &self,
        family: PhysicalArtifactFamily,
    ) -> BootstrapOnlyAccessDenied {
        BootstrapOnlyAccessDenied::OrdinaryFamilyAccessForbidden { family }
    }
}

impl Default for BootstrapCatalogAccess {
    fn default() -> Self {
        Self::new()
    }
}

pub const fn bootstrap_catalog() -> BootstrapCatalogAccess {
    BootstrapCatalogAccess::new()
}
