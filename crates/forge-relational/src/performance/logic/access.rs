use crate::logic::runtime::RelationalRuntime;
use crate::performance::data::{
    ComplexityContract, RuntimeComplexityCounters, COMPLEXITY_CONTRACTS,
};

pub struct PerformanceAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn performance_access(&self) -> PerformanceAccess<'_> {
        PerformanceAccess::new(self)
    }
}

impl<'runtime> PerformanceAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn contracts(&self) -> &'static [ComplexityContract] {
        COMPLEXITY_CONTRACTS
    }

    pub fn counters(&self) -> RuntimeComplexityCounters {
        self.runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone()
    }

    pub fn reset_counters(&self) {
        *self
            .runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned") = RuntimeComplexityCounters::default();
    }

    pub(crate) fn count_invariant_entity_slot_scans(&self, slots: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.invariant_entity_slot_scans += slots);
    }

    pub(crate) fn count_invariant_relation_slot_scans(&self, slots: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.invariant_relation_slot_scans += slots);
    }

    pub(crate) fn count_preparation_packets(&self, packets: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_packet_count += packets);
    }

    pub(crate) fn count_preparation_parallel_legal(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_parallel_legal_count += 1);
    }

    pub(crate) fn count_preparation_parallel_profitable(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_parallel_profitable_count += 1);
    }

    pub(crate) fn count_preparation_serial_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_serial_strategy_count += 1);
    }

    pub(crate) fn count_preparation_staged_parallel_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_staged_parallel_strategy_count += 1);
    }

    pub(crate) fn count_preparation_reducer_conflicts(&self, conflicts: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_reducer_conflict_count += conflicts);
    }
}
