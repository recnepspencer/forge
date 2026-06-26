#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationSentinel {
    whole_store_materialization_attempts: u64,
    unbounded_allocation_attempts: u64,
    domain_object_constructions: u64,
    copied_payload_bytes: u64,
    diagnostic_materialization_bytes: u64,
}

impl AllocationSentinel {
    pub const fn no_shortcuts() -> Self {
        Self {
            whole_store_materialization_attempts: 0,
            unbounded_allocation_attempts: 0,
            domain_object_constructions: 0,
            copied_payload_bytes: 0,
            diagnostic_materialization_bytes: 0,
        }
    }

    pub const fn whole_store_materialization_attempts(&self) -> u64 {
        self.whole_store_materialization_attempts
    }

    pub const fn unbounded_allocation_attempts(&self) -> u64 {
        self.unbounded_allocation_attempts
    }

    pub const fn domain_object_constructions(&self) -> u64 {
        self.domain_object_constructions
    }

    pub const fn copied_payload_bytes(&self) -> u64 {
        self.copied_payload_bytes
    }

    pub const fn diagnostic_materialization_bytes(&self) -> u64 {
        self.diagnostic_materialization_bytes
    }
}
