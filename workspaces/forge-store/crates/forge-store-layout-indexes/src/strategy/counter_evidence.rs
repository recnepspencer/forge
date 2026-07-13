use super::invariant_suite::StrategyCounterProfile;
use crate::access::budget::PlannedCounterEnvelope;
use crate::access::execution::AccessPathCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrategyCounterEvidence {
    point_lookup: Option<PlannedCounterEnvelope>,
    range_lookup: Option<PlannedCounterEnvelope>,
    prefix_lookup: Option<PlannedCounterEnvelope>,
    publication: AccessPathCounterSnapshot,
    recovery: AccessPathCounterSnapshot,
    aggregate: StrategyCounterProfile,
}

impl StrategyCounterEvidence {
    pub(crate) const fn new(
        point_lookup: Option<PlannedCounterEnvelope>,
        range_lookup: Option<PlannedCounterEnvelope>,
        prefix_lookup: Option<PlannedCounterEnvelope>,
        publication: AccessPathCounterSnapshot,
        recovery: AccessPathCounterSnapshot,
        aggregate: StrategyCounterProfile,
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

    pub const fn lookup(self) -> Option<PlannedCounterEnvelope> {
        if self.range_lookup.is_some() || self.prefix_lookup.is_some() {
            None
        } else {
            self.point_lookup
        }
    }

    pub const fn point_lookup(self) -> Option<PlannedCounterEnvelope> {
        self.point_lookup
    }

    pub const fn range_lookup(self) -> Option<PlannedCounterEnvelope> {
        self.range_lookup
    }

    pub const fn prefix_lookup(self) -> Option<PlannedCounterEnvelope> {
        self.prefix_lookup
    }

    pub const fn publication(self) -> AccessPathCounterSnapshot {
        self.publication
    }

    pub const fn recovery(self) -> AccessPathCounterSnapshot {
        self.recovery
    }

    pub const fn aggregate_profile(self) -> StrategyCounterProfile {
        self.aggregate
    }
}
