#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PreExecutionBudgetDenial {
    ScopeMismatch {
        requested: super::admission::S8PreExecutionBudgetScope,
        admitted: super::admission::S8PreExecutionBudgetScope,
    },
    MemoryBytesExceeded {
        estimated: u64,
        admitted: u64,
    },
    PageReadsExceeded {
        estimated: u16,
        admitted: u16,
    },
    ChunkReadsExceeded {
        estimated: u16,
        admitted: u16,
    },
    RangeTouchesExceeded {
        estimated: u16,
        admitted: u16,
    },
    ByteReadsExceeded {
        estimated: u64,
        admitted: u64,
    },
}
