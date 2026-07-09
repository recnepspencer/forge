use worth_store_physical_format::{PhysicalBootstrapCatalogIdentity, PhysicalFormatVersion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8BootstrapCatalogReadAdmission {
    identity: PhysicalBootstrapCatalogIdentity,
}

impl S8BootstrapCatalogReadAdmission {
    pub(crate) fn new(identity: PhysicalBootstrapCatalogIdentity) -> Self {
        Self { identity }
    }

    pub(crate) fn identity(&self) -> &PhysicalBootstrapCatalogIdentity {
        &self.identity
    }

    pub fn root_owner(&self) -> worth_store_physical_format::PhysicalGenerationOwner {
        self.identity().root_owner()
    }

    pub fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.identity().physical_format_version()
    }
}
