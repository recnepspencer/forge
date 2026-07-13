use forge_store_physical_format::{PhysicalFormatVersion, PhysicalRootReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinimalRootDiscoveryLayout {
    root_reference: PhysicalRootReference,
    physical_format_version: PhysicalFormatVersion,
    checksum_bytes_checked: u64,
}

impl MinimalRootDiscoveryLayout {
    pub(crate) const fn new(
        root_reference: PhysicalRootReference,
        physical_format_version: PhysicalFormatVersion,
        checksum_bytes_checked: u64,
    ) -> Self {
        Self {
            root_reference,
            physical_format_version,
            checksum_bytes_checked,
        }
    }

    pub const fn root_reference(self) -> PhysicalRootReference {
        self.root_reference
    }

    pub const fn physical_format_version(self) -> PhysicalFormatVersion {
        self.physical_format_version
    }

    pub const fn checksum_bytes_checked(self) -> u64 {
        self.checksum_bytes_checked
    }
}
