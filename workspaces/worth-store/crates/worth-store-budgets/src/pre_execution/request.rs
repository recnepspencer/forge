use super::admission::PreExecutionBudgetScope;

/// Resource demand presented to the budget owner before execution.
///
/// This request deliberately contains no plan identity. Layout planning owns
/// plan identity; the budget subsystem owns only resource admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetRequest {
    scope: PreExecutionBudgetScope,
    estimated_memory_bytes: u64,
    estimated_page_reads: u16,
    estimated_chunk_reads: u16,
    estimated_range_touches: u16,
    estimated_byte_reads: u64,
}

impl PreExecutionBudgetRequest {
    pub const fn new(
        scope: PreExecutionBudgetScope,
        estimated_memory_bytes: u64,
        estimated_page_reads: u16,
        estimated_chunk_reads: u16,
        estimated_range_touches: u16,
        estimated_byte_reads: u64,
    ) -> Self {
        Self {
            scope,
            estimated_memory_bytes,
            estimated_page_reads,
            estimated_chunk_reads,
            estimated_range_touches,
            estimated_byte_reads,
        }
    }

    pub const fn scope(self) -> PreExecutionBudgetScope {
        self.scope
    }
    pub const fn estimated_memory_bytes(self) -> u64 {
        self.estimated_memory_bytes
    }
    pub const fn estimated_page_reads(self) -> u16 {
        self.estimated_page_reads
    }
    pub const fn estimated_chunk_reads(self) -> u16 {
        self.estimated_chunk_reads
    }
    pub const fn estimated_range_touches(self) -> u16 {
        self.estimated_range_touches
    }
    pub const fn estimated_byte_reads(self) -> u64 {
        self.estimated_byte_reads
    }
}
