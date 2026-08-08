//! Structural work established by one exact committed-outbox owner read.

/// Bounded structural cost of a successful exact-commit outbox observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCommittedDispatchOutboxReadWork {
    exact_commit_snapshots: usize,
    examined_index_entries: usize,
    projected_records: usize,
    projected_fields: usize,
}

impl WorthQueryCommittedDispatchOutboxReadWork {
    pub(super) const fn exact_read(examined_index_entries: usize) -> Self {
        Self {
            exact_commit_snapshots: 1,
            examined_index_entries,
            projected_records: 1,
            projected_fields: 8,
        }
    }

    pub const fn exact_commit_snapshots(self) -> usize {
        self.exact_commit_snapshots
    }

    pub const fn examined_index_entries(self) -> usize {
        self.examined_index_entries
    }

    pub const fn projected_records(self) -> usize {
        self.projected_records
    }

    pub const fn projected_fields(self) -> usize {
        self.projected_fields
    }
}
