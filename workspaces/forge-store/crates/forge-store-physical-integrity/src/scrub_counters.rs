#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScrubCounterSnapshot {
    checked_page_count: u64,
    checked_byte_count: u64,
    planned_window_count: u64,
    completed_window_count: u64,
    skipped_window_count: u64,
    deferred_window_count: u64,
    over_budget_window_count: u64,
    interrupted_window_count: u64,
    revalidated_window_count: u64,
    skipped_decode_count: u64,
    yielded_background_work_count: u64,
    checksum_counter: u64,
}

impl ScrubCounterSnapshot {
    pub(crate) const fn planned(planned_window_count: u64) -> Self {
        Self {
            planned_window_count,
            ..Self::empty()
        }
    }

    pub const fn empty() -> Self {
        Self {
            checked_page_count: 0,
            checked_byte_count: 0,
            planned_window_count: 0,
            completed_window_count: 0,
            skipped_window_count: 0,
            deferred_window_count: 0,
            over_budget_window_count: 0,
            interrupted_window_count: 0,
            revalidated_window_count: 0,
            skipped_decode_count: 0,
            yielded_background_work_count: 0,
            checksum_counter: 0,
        }
    }

    pub(crate) const fn with_completed_inspection(self, bytes: u64, checksum: u64) -> Self {
        Self {
            checked_page_count: self.checked_page_count + 1,
            checked_byte_count: self.checked_byte_count + bytes,
            completed_window_count: self.completed_window_count + 1,
            skipped_decode_count: self.skipped_decode_count + 1,
            checksum_counter: self.checksum_counter + checksum,
            ..self
        }
    }

    pub(crate) const fn with_skipped(self) -> Self {
        Self {
            skipped_window_count: self.skipped_window_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_deferred_over_budget(self) -> Self {
        Self {
            deferred_window_count: self.deferred_window_count + 1,
            over_budget_window_count: self.over_budget_window_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_interruption(self) -> Self {
        Self {
            interrupted_window_count: self.interrupted_window_count + 1,
            yielded_background_work_count: self.yielded_background_work_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_revalidated(self, bytes: u64, checksum: u64) -> Self {
        Self {
            checked_page_count: self.checked_page_count + 1,
            checked_byte_count: self.checked_byte_count + bytes,
            revalidated_window_count: self.revalidated_window_count + 1,
            checksum_counter: self.checksum_counter + checksum,
            ..self
        }
    }

    pub const fn checked_page_count(self) -> u64 {
        self.checked_page_count
    }

    pub const fn checked_byte_count(self) -> u64 {
        self.checked_byte_count
    }

    pub const fn planned_window_count(self) -> u64 {
        self.planned_window_count
    }

    pub const fn completed_window_count(self) -> u64 {
        self.completed_window_count
    }

    pub const fn skipped_window_count(self) -> u64 {
        self.skipped_window_count
    }

    pub const fn deferred_window_count(self) -> u64 {
        self.deferred_window_count
    }

    pub const fn over_budget_window_count(self) -> u64 {
        self.over_budget_window_count
    }

    pub const fn interrupted_window_count(self) -> u64 {
        self.interrupted_window_count
    }

    pub const fn revalidated_window_count(self) -> u64 {
        self.revalidated_window_count
    }

    pub const fn skipped_decode_count(self) -> u64 {
        self.skipped_decode_count
    }

    pub const fn yielded_background_work_count(self) -> u64 {
        self.yielded_background_work_count
    }

    pub const fn checksum_counter(self) -> u64 {
        self.checksum_counter
    }
}
