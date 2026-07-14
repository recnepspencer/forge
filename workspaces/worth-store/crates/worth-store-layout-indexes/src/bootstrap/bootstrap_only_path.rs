use worth_store_physical_format::{PhysicalFormatMagic, PhysicalFormatVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOnlyAccessPath {
    magic: PhysicalFormatMagic,
    version: PhysicalFormatVersion,
}

impl BootstrapOnlyAccessPath {
    pub const fn fixed_bootstrap_access_path() -> Self {
        Self {
            magic: PhysicalFormatMagic::store_format_magic(),
            version: PhysicalFormatVersion::initial_format_version(),
        }
    }

    pub const fn magic(self) -> PhysicalFormatMagic {
        self.magic
    }

    pub const fn physical_format_version(self) -> PhysicalFormatVersion {
        self.version
    }
}
