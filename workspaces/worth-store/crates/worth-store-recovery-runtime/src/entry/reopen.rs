use worth_store::physical_runtime::PhysicalRecoveryFreshReopenDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalRecoveryReopenCounters {
    pub selector_reads_completed: u64,
    pub root_reads_completed: u64,
    pub bytes_read: u64,
}

pub struct PhysicalRecoveryReopenFailure {
    counters: PhysicalRecoveryReopenCounters,
    denial: PhysicalRecoveryFreshReopenDenial,
}

impl PhysicalRecoveryReopenFailure {
    pub(crate) const fn new(
        counters: PhysicalRecoveryReopenCounters,
        denial: PhysicalRecoveryFreshReopenDenial,
    ) -> Self {
        Self { counters, denial }
    }

    pub const fn counters(&self) -> PhysicalRecoveryReopenCounters {
        self.counters
    }

    pub const fn denial(&self) -> &PhysicalRecoveryFreshReopenDenial {
        &self.denial
    }
}

impl std::fmt::Debug for PhysicalRecoveryReopenFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PhysicalRecoveryReopenFailure")
            .field("counters", &self.counters)
            .field("stage", &self.denial.stage())
            .field("kind", &self.denial.kind())
            .finish()
    }
}
