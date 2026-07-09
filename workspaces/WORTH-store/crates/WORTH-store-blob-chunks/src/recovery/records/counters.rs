#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BlobRecoveryRecordCounterSnapshot {
    wal_records: u64,
    checkpoint_records: u64,
    manifest_rows: u64,
    replayed_outcomes: u64,
    denials: u64,
}

impl BlobRecoveryRecordCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            wal_records: 0,
            checkpoint_records: 0,
            manifest_rows: 0,
            replayed_outcomes: 0,
            denials: 0,
        }
    }

    pub const fn with_wal_record(mut self) -> Self {
        self.wal_records += 1;
        self
    }

    pub const fn with_checkpoint_record(mut self) -> Self {
        self.checkpoint_records += 1;
        self
    }

    pub const fn with_manifest_row(mut self) -> Self {
        self.manifest_rows += 1;
        self
    }

    pub const fn with_replayed_outcome(mut self) -> Self {
        self.replayed_outcomes += 1;
        self
    }

    pub const fn with_denial(mut self) -> Self {
        self.denials += 1;
        self
    }

    pub const fn merge(mut self, other: Self) -> Self {
        self.wal_records += other.wal_records;
        self.checkpoint_records += other.checkpoint_records;
        self.manifest_rows += other.manifest_rows;
        self.replayed_outcomes += other.replayed_outcomes;
        self.denials += other.denials;
        self
    }

    pub const fn wal_records(self) -> u64 {
        self.wal_records
    }

    pub const fn checkpoint_records(self) -> u64 {
        self.checkpoint_records
    }

    pub const fn manifest_rows(self) -> u64 {
        self.manifest_rows
    }

    pub const fn replayed_outcomes(self) -> u64 {
        self.replayed_outcomes
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
