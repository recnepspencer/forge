use forge_store_physical_format::{PhysicalFormatMagic, PhysicalFormatVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BootstrapOnlyAccessPath {
    magic: PhysicalFormatMagic,
    version: PhysicalFormatVersion,
}

impl S8BootstrapOnlyAccessPath {
    pub const fn s8_fixed() -> Self {
        Self {
            magic: PhysicalFormatMagic::s1_store(),
            version: PhysicalFormatVersion::s1_initial(),
        }
    }

    pub const fn magic(self) -> PhysicalFormatMagic {
        self.magic
    }

    pub const fn physical_format_version(self) -> PhysicalFormatVersion {
        self.version
    }
}
