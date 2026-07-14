use crate::{
    PhysicalChunkChecksum, PhysicalFormatVersion, PhysicalGenerationOwner, PhysicalRootReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBootstrapCatalogIdentity {
    root_owner: PhysicalGenerationOwner,
    root_reference: PhysicalRootReference,
    physical_format_version: PhysicalFormatVersion,
    checksum: PhysicalChunkChecksum,
}

impl PhysicalBootstrapCatalogIdentity {
    pub(crate) fn new(
        root_owner: PhysicalGenerationOwner,
        root_reference: PhysicalRootReference,
        physical_format_version: PhysicalFormatVersion,
        checksum: PhysicalChunkChecksum,
    ) -> Self {
        Self {
            root_owner,
            root_reference,
            physical_format_version,
            checksum,
        }
    }

    pub const fn root_owner(&self) -> PhysicalGenerationOwner {
        self.root_owner
    }

    pub const fn root_reference(&self) -> PhysicalRootReference {
        self.root_reference
    }

    pub const fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.physical_format_version
    }

    pub const fn checksum(&self) -> &PhysicalChunkChecksum {
        &self.checksum
    }
}
