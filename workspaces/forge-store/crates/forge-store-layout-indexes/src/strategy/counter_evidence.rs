use crate::execution::S8PlannedVsObservedCounterReceipt;

use super::invariant_suite::S8StrategyCounterProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8StrategyCounterEvidence {
    lookup: S8PlannedVsObservedCounterReceipt,
    publication: S8PlannedVsObservedCounterReceipt,
    recovery: S8PlannedVsObservedCounterReceipt,
    aggregate: S8StrategyCounterProfile,
}

impl S8StrategyCounterEvidence {
    pub(crate) const fn new(
        lookup: S8PlannedVsObservedCounterReceipt,
        publication: S8PlannedVsObservedCounterReceipt,
        recovery: S8PlannedVsObservedCounterReceipt,
        aggregate: S8StrategyCounterProfile,
    ) -> Self {
        Self {
            lookup,
            publication,
            recovery,
            aggregate,
        }
    }

    pub const fn lookup(self) -> S8PlannedVsObservedCounterReceipt {
        self.lookup
    }

    pub const fn publication(self) -> S8PlannedVsObservedCounterReceipt {
        self.publication
    }

    pub const fn recovery(self) -> S8PlannedVsObservedCounterReceipt {
        self.recovery
    }

    pub const fn aggregate_profile(self) -> S8StrategyCounterProfile {
        self.aggregate
    }
}
