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
}
