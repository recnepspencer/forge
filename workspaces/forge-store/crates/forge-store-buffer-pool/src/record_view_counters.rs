#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordCopyCounterSnapshot {
    zero_copy_admission_attempt_count: u64,
    zero_copy_admission_count: u64,
    bounded_copy_attempt_count: u64,
    bounded_copy_count: u64,
    copied_bytes: u64,
    materialized_bytes: u64,
    cow_fallback_count: u64,
    denied_before_view_construction_count: u64,
    dirty_mutation_conflict_denial_count: u64,
    publication_conflict_denial_count: u64,
}

impl RecordCopyCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            zero_copy_admission_attempt_count: 0,
            zero_copy_admission_count: 0,
            bounded_copy_attempt_count: 0,
            bounded_copy_count: 0,
            copied_bytes: 0,
            materialized_bytes: 0,
            cow_fallback_count: 0,
            denied_before_view_construction_count: 0,
            dirty_mutation_conflict_denial_count: 0,
            publication_conflict_denial_count: 0,
        }
    }

    pub(crate) const fn with_zero_copy_attempt(mut self) -> Self {
        self.zero_copy_admission_attempt_count += 1;
        self
    }

    pub(crate) const fn with_zero_copy_admission(mut self) -> Self {
        self.zero_copy_admission_count += 1;
        self
    }

    pub(crate) const fn with_bounded_copy_attempt(mut self) -> Self {
        self.bounded_copy_attempt_count += 1;
        self
    }

    pub(crate) const fn with_bounded_copy(mut self, bytes: u64) -> Self {
        self.bounded_copy_count += 1;
        self.copied_bytes += bytes;
        self
    }

    pub(crate) const fn with_materialized_copy(mut self, bytes: u64) -> Self {
        self.bounded_copy_count += 1;
        self.copied_bytes += bytes;
        self.materialized_bytes += bytes;
        self
    }

    pub(crate) const fn with_denied_before_view_construction(mut self) -> Self {
        self.denied_before_view_construction_count += 1;
        self
    }

    pub(crate) const fn with_dirty_mutation_conflict_denial(mut self) -> Self {
        self.dirty_mutation_conflict_denial_count += 1;
        self
    }

    pub(crate) const fn with_publication_conflict_denial(mut self) -> Self {
        self.publication_conflict_denial_count += 1;
        self
    }

    pub const fn zero_copy_admission_attempt_count(self) -> u64 {
        self.zero_copy_admission_attempt_count
    }

    pub const fn zero_copy_admission_count(self) -> u64 {
        self.zero_copy_admission_count
    }

    pub const fn bounded_copy_attempt_count(self) -> u64 {
        self.bounded_copy_attempt_count
    }

    pub const fn bounded_copy_count(self) -> u64 {
        self.bounded_copy_count
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn materialized_bytes(self) -> u64 {
        self.materialized_bytes
    }

    pub const fn cow_fallback_count(self) -> u64 {
        self.cow_fallback_count
    }

    pub const fn denied_before_view_construction_count(self) -> u64 {
        self.denied_before_view_construction_count
    }

    pub const fn dirty_mutation_conflict_denial_count(self) -> u64 {
        self.dirty_mutation_conflict_denial_count
    }

    pub const fn publication_conflict_denial_count(self) -> u64 {
        self.publication_conflict_denial_count
    }
}
