#![forbid(unsafe_code)]

use forge_store_physical_format::PhysicalPageId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolBudget {
    resident_byte_limit: u64,
}

impl BufferPoolBudget {
    pub const fn new(resident_byte_limit: u64) -> Self {
        Self {
            resident_byte_limit,
        }
    }

    pub const fn resident_byte_limit(&self) -> u64 {
        self.resident_byte_limit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageLease {
    page_id: PhysicalPageId,
}

impl PageLease {
    pub const fn new(page_id: PhysicalPageId) -> Self {
        Self { page_id }
    }
}
