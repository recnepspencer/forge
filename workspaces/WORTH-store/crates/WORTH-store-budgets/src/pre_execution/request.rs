use super::admission::S8PreExecutionBudgetScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PreExecutionPlanBinding {
    identity_word: u64,
    lookup_word: u64,
    publication_word: u64,
    recovery_word: u64,
    budget_rows: u64,
}

impl S8PreExecutionPlanBinding {
    pub const fn new(
        identity_word: u64,
        lookup_word: u64,
        publication_word: u64,
        recovery_word: u64,
        budget_rows: u64,
    ) -> Self {
        Self {
            identity_word,
            lookup_word,
            publication_word,
            recovery_word,
            budget_rows,
        }
    }

    pub const fn identity_word(self) -> u64 {
        self.identity_word
    }

    pub const fn lookup_word(self) -> u64 {
        self.lookup_word
    }

    pub const fn publication_word(self) -> u64 {
        self.publication_word
    }

    pub const fn recovery_word(self) -> u64 {
        self.recovery_word
    }

    pub const fn budget_rows(self) -> u64 {
        self.budget_rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PreExecutionBudgetRequest {
    plan_binding: S8PreExecutionPlanBinding,
    scope: S8PreExecutionBudgetScope,
    estimated_memory_bytes: u64,
    estimated_page_reads: u16,
    estimated_chunk_reads: u16,
    estimated_range_touches: u16,
    estimated_byte_reads: u64,
}

impl S8PreExecutionBudgetRequest {
    pub const fn new(
        plan_binding: S8PreExecutionPlanBinding,
        scope: S8PreExecutionBudgetScope,
        estimated_memory_bytes: u64,
        estimated_page_reads: u16,
        estimated_chunk_reads: u16,
        estimated_range_touches: u16,
        estimated_byte_reads: u64,
    ) -> Self {
        Self {
            plan_binding,
            scope,
            estimated_memory_bytes,
            estimated_page_reads,
            estimated_chunk_reads,
            estimated_range_touches,
            estimated_byte_reads,
        }
    }

    pub const fn plan_binding(self) -> S8PreExecutionPlanBinding {
        self.plan_binding
    }

    pub const fn scope(self) -> S8PreExecutionBudgetScope {
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
