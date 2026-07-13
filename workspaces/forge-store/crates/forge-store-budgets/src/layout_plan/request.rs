#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutPlanWork {
    page_reads: u32,
    byte_reads: u64,
    allocations: u32,
}

impl LayoutPlanWork {
    pub const fn exact(page_reads: u32, byte_reads: u64, allocations: u32) -> Self {
        Self {
            page_reads,
            byte_reads,
            allocations,
        }
    }

    pub const fn page_reads(self) -> u32 {
        self.page_reads
    }
    pub const fn byte_reads(self) -> u64 {
        self.byte_reads
    }
    pub const fn allocations(self) -> u32 {
        self.allocations
    }
}
