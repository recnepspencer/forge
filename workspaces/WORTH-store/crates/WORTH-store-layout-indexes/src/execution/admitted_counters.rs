use worth_store_budgets::CounterEvidenceStrength;

use super::{S8AccessLoweringBasis, S8AccessPathCounterSnapshot, S8ObservedAccessPathCounters};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AdmittedExecutedCounters {
    observed: S8ObservedAccessPathCounters,
}

impl S8AdmittedExecutedCounters {
    pub(crate) const fn new(observed: S8ObservedAccessPathCounters) -> Self {
        Self { observed }
    }

    pub const fn basis(self) -> S8AccessLoweringBasis {
        self.observed.basis()
    }

    pub const fn snapshot(self) -> S8AccessPathCounterSnapshot {
        self.observed.snapshot()
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.observed.strength()
    }

    pub(crate) const fn observed(self) -> S8ObservedAccessPathCounters {
        self.observed
    }
}
