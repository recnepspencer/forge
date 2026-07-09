use crate::execution::S8AccessPathCounterSnapshot;
use crate::strategy::S8StrategyCounterProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PlannedCounterEnvelope {
    lookup: S8AccessPathCounterSnapshot,
    publication: S8AccessPathCounterSnapshot,
    recovery: S8AccessPathCounterSnapshot,
}

impl S8PlannedCounterEnvelope {
    pub(crate) const fn new(
        lookup: S8AccessPathCounterSnapshot,
        publication: S8AccessPathCounterSnapshot,
        recovery: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            lookup,
            publication,
            recovery,
        }
    }

    pub const fn lookup(self) -> S8AccessPathCounterSnapshot {
        self.lookup
    }

    pub const fn publication(self) -> S8AccessPathCounterSnapshot {
        self.publication
    }

    pub const fn recovery(self) -> S8AccessPathCounterSnapshot {
        self.recovery
    }

    pub const fn aggregate_profile(self) -> S8StrategyCounterProfile {
        S8StrategyCounterProfile::new(
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
