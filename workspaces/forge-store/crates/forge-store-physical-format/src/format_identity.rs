#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFormatMagic {
    bytes: [u8; 8],
}

impl PhysicalFormatMagic {
    pub const fn s1_store() -> Self {
        Self {
            bytes: *b"FGS1FMT\0",
        }
    }

    pub const fn bytes(&self) -> [u8; 8] {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFormatVersion {
    value: u16,
}

impl PhysicalFormatVersion {
    pub const fn s1_initial() -> Self {
        Self { value: 1 }
    }

    pub const fn reserved_future(value: u16) -> Option<Self> {
        if value > 1 {
            Some(Self { value })
        } else {
            None
        }
    }

    pub const fn value(&self) -> u16 {
        self.value
    }

    pub const fn is_reserved_future(&self) -> bool {
        self.value > 1
    }
}
