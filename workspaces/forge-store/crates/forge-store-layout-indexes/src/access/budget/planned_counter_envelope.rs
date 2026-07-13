use crate::access::execution::AccessPathCounterSnapshot;
use crate::strategy::StrategyCounterProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedCounterEnvelope {
    lookup: AccessPathCounterSnapshot,
    publication: AccessPathCounterSnapshot,
    recovery: AccessPathCounterSnapshot,
}

impl PlannedCounterEnvelope {
    pub(crate) const fn new(
        lookup: AccessPathCounterSnapshot,
        publication: AccessPathCounterSnapshot,
        recovery: AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            lookup,
            publication,
            recovery,
        }
    }

    pub const fn lookup(self) -> AccessPathCounterSnapshot {
        self.lookup
    }

    pub const fn publication(self) -> AccessPathCounterSnapshot {
        self.publication
    }

    pub const fn recovery(self) -> AccessPathCounterSnapshot {
        self.recovery
    }

    pub const fn aggregate_profile(self) -> StrategyCounterProfile {
        StrategyCounterProfile::new(
            self.lookup.point_lookups()
                + self.publication.point_lookups()
                + self.recovery.point_lookups(),
            self.lookup.range_lookups()
                + self.publication.range_lookups()
                + self.recovery.range_lookups(),
            self.lookup.wal_replays()
                + self.publication.wal_replays()
                + self.recovery.wal_replays(),
            self.lookup.publications()
                + self.publication.publications()
                + self.recovery.publications(),
            self.lookup.maintenance_reads()
                + self.publication.maintenance_reads()
                + self.recovery.maintenance_reads(),
        )
    }
}
