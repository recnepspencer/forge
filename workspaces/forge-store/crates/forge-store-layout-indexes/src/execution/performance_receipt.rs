use forge_store_budgets::CounterEvidenceStrength;

use crate::planning::S8PlanFingerprint;

use super::{S8AccessPathAmplificationReceipt, S8PlannedVsObservedCounterReceipt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8StoreLayoutPerformanceReceipt {
    fingerprint: S8PlanFingerprint,
    planned_vs_observed: S8PlannedVsObservedCounterReceipt,
    amplification: S8AccessPathAmplificationReceipt,
    counter_strength: CounterEvidenceStrength,
}

impl S8StoreLayoutPerformanceReceipt {
    pub(crate) const fn new(
        fingerprint: S8PlanFingerprint,
        planned_vs_observed: S8PlannedVsObservedCounterReceipt,
        amplification: S8AccessPathAmplificationReceipt,
        counter_strength: CounterEvidenceStrength,
    ) -> Self {
        Self {
            fingerprint,
            planned_vs_observed,
            amplification,
            counter_strength,
        }
    }

    pub const fn fingerprint(self) -> S8PlanFingerprint {
        self.fingerprint
    }

    pub const fn planned_vs_observed(self) -> S8PlannedVsObservedCounterReceipt {
        self.planned_vs_observed
    }

    pub const fn amplification(self) -> S8AccessPathAmplificationReceipt {
        self.amplification
    }

    pub const fn counter_strength(self) -> CounterEvidenceStrength {
        self.counter_strength
    }
}
