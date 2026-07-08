use crate::execution::S8AccessPathCounterSnapshot;
use crate::S8AccessLoweringBasis;
use forge_store_budgets::CounterEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8ObservedAccessPathCounters {
    basis: S8AccessLoweringBasis,
    snapshot: S8AccessPathCounterSnapshot,
    strength: CounterEvidenceStrength,
}

impl S8ObservedAccessPathCounters {
    pub(crate) const fn admitted(
        basis: S8AccessLoweringBasis,
        snapshot: S8AccessPathCounterSnapshot,
        strength: CounterEvidenceStrength,
    ) -> Self {
        Self {
            basis,
            snapshot,
            strength,
        }
    }

    pub const fn basis(self) -> S8AccessLoweringBasis {
        self.basis
    }

    pub const fn snapshot(self) -> S8AccessPathCounterSnapshot {
        self.snapshot
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }
}
