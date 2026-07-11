use super::invariant_suite::S8StrategyCounterProfile;
use crate::budget::S8PlannedCounterEnvelope;
use crate::execution::S8AccessPathCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8StrategyCounterEvidence {
    point_lookup: Option<S8PlannedCounterEnvelope>,
    range_lookup: Option<S8PlannedCounterEnvelope>,
    prefix_lookup: Option<S8PlannedCounterEnvelope>,
    publication: S8AccessPathCounterSnapshot,
    recovery: S8AccessPathCounterSnapshot,
    aggregate: S8StrategyCounterProfile,
}

impl S8StrategyCounterEvidence {
    pub(crate) const fn new(
        point_lookup: Option<S8PlannedCounterEnvelope>,
        range_lookup: Option<S8PlannedCounterEnvelope>,
        prefix_lookup: Option<S8PlannedCounterEnvelope>,
        publication: S8AccessPathCounterSnapshot,
        recovery: S8AccessPathCounterSnapshot,
        aggregate: S8StrategyCounterProfile,
    ) -> Self {
        Self {
            point_lookup,
            range_lookup,
            prefix_lookup,
            publication,
            recovery,
            aggregate,
        }
    }

    pub const fn lookup(self) -> Option<S8PlannedCounterEnvelope> {
        if self.range_lookup.is_some() || self.prefix_lookup.is_some() {
            None
        } else {
            self.point_lookup
        }
    }

    pub const fn point_lookup(self) -> Option<S8PlannedCounterEnvelope> {
        self.point_lookup
    }

    pub const fn range_lookup(self) -> Option<S8PlannedCounterEnvelope> {
        self.range_lookup
    }

    pub const fn prefix_lookup(self) -> Option<S8PlannedCounterEnvelope> {
        self.prefix_lookup
    }

    pub const fn publication(self) -> S8AccessPathCounterSnapshot {
        self.publication
    }

    pub const fn recovery(self) -> S8AccessPathCounterSnapshot {
        self.recovery
    }

    pub const fn aggregate_profile(self) -> S8StrategyCounterProfile {
        self.aggregate
    }
}
