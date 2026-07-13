#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCounterObservation {
    point_lookups: u16,
    range_lookups: u16,
    wal_replays: u16,
    publications: u16,
    maintenance_reads: u16,
}

impl BaselineLsmCounterObservation {
    pub(super) const fn lookup() -> Self {
        Self {
            point_lookups: 1,
            range_lookups: 1,
            wal_replays: 0,
            publications: 0,
            maintenance_reads: 0,
        }
    }

    pub(super) const fn replay(replayed_records: u16, cleanup_batches: u16) -> Self {
        Self {
            point_lookups: 0,
            range_lookups: 0,
            wal_replays: replayed_records,
            publications: 0,
            maintenance_reads: cleanup_batches,
        }
    }

    pub(super) const fn manifest_publication(published_runs: u16) -> Self {
        Self {
            point_lookups: 0,
            range_lookups: 0,
            wal_replays: 0,
            publications: 2,
            maintenance_reads: published_runs,
        }
    }

    pub(super) const fn compaction(retired_runs: u16) -> Self {
        Self {
            point_lookups: 0,
            range_lookups: 0,
            wal_replays: 0,
            publications: 1,
            maintenance_reads: retired_runs,
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn wal_replays(self) -> u16 {
        self.wal_replays
    }

    pub const fn publications(self) -> u16 {
        self.publications
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.maintenance_reads
    }
}
