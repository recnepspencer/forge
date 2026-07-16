#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBoundaryFault {
    Interrupt,
    TearWrite { retained_bytes: u64 },
    CorruptByte { relative_offset: u64, xor_mask: u8 },
    AbortBeforeDurabilityBarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageBoundaryRegion {
    offset: u64,
    bytes: u64,
}

impl StorageBoundaryRegion {
    pub const fn new(offset: u64, bytes: u64) -> Self {
        Self { offset, bytes }
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn bytes(self) -> u64 {
        self.bytes
    }
}
