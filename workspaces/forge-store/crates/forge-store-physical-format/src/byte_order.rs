#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalByteOrder {
    LittleEndian,
}

impl PhysicalByteOrder {
    pub const fn code(self) -> u8 {
        match self {
            Self::LittleEndian => 1,
        }
    }

    pub const fn write_u16(self, value: u16) -> [u8; 2] {
        match self {
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    pub const fn write_u32(self, value: u32) -> [u8; 4] {
        match self {
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    pub const fn write_u64(self, value: u64) -> [u8; 8] {
        match self {
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    pub const fn read_u16(self, bytes: [u8; 2]) -> u16 {
        match self {
            Self::LittleEndian => u16::from_le_bytes(bytes),
        }
    }

    pub const fn read_u32(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::LittleEndian => u32::from_le_bytes(bytes),
        }
    }

    pub const fn read_u64(self, bytes: [u8; 8]) -> u64 {
        match self {
            Self::LittleEndian => u64::from_le_bytes(bytes),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalByteOrderDeclaration {
    Explicit(PhysicalByteOrder),
    HostEndian,
}

impl From<PhysicalByteOrder> for PhysicalByteOrderDeclaration {
    fn from(value: PhysicalByteOrder) -> Self {
        Self::Explicit(value)
    }
}
