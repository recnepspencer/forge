use crate::PhysicalBinaryFormatError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPageSizeClass {
    KiB16,
    KiB32,
    KiB64,
}

impl PhysicalPageSizeClass {
    pub const fn bytes(self) -> u32 {
        match self {
            Self::KiB16 => 16 * 1024,
            Self::KiB32 => 32 * 1024,
            Self::KiB64 => 64 * 1024,
        }
    }

    pub fn from_bytes(bytes: u32) -> Result<Self, PhysicalBinaryFormatError> {
        match bytes {
            16_384 => Ok(Self::KiB16),
            32_768 => Ok(Self::KiB32),
            65_536 => Ok(Self::KiB64),
            _ => Err(PhysicalBinaryFormatError::UnsupportedPageSize(bytes)),
        }
    }
}
