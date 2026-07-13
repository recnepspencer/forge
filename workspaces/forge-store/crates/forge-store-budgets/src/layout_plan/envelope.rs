#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPlanBudgetScope {
    ForegroundIndexed,
    DegradedExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutPlanBudget {
    scope: LayoutPlanBudgetScope,
    max_page_reads: u32,
    max_byte_reads: u64,
    max_allocations: u32,
}

impl LayoutPlanBudget {
    pub const fn new(
        scope: LayoutPlanBudgetScope,
        max_page_reads: u32,
        max_byte_reads: u64,
        max_allocations: u32,
    ) -> Self {
        Self {
            scope,
            max_page_reads,
            max_byte_reads,
            max_allocations,
        }
    }

    pub const fn scope(self) -> LayoutPlanBudgetScope {
        self.scope
    }
    pub const fn max_page_reads(self) -> u32 {
        self.max_page_reads
    }
    pub const fn max_byte_reads(self) -> u64 {
        self.max_byte_reads
    }
    pub const fn max_allocations(self) -> u32 {
        self.max_allocations
    }
}
