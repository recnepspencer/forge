#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedIndexRebuildCounterSnapshot {
    source_artifacts_read: u64,
    source_rows_read: u64,
    source_bytes_read: u64,
    candidate_rows_written: u64,
    candidate_bytes_written: u64,
    canonical_row_order_comparisons: u64,
    unique_key_comparisons: u64,
    counter_shape_order_comparisons: u64,
}

impl DerivedIndexRebuildCounterSnapshot {
    pub(super) fn from_candidate(
        source_artifacts_read: u64,
        candidate: &super::DerivedIndexParityBasis,
    ) -> Self {
        let rows = candidate.ordered_rows();
        let bytes = rows.iter().fold(0_u64, |total, row| {
            total
                .saturating_add(row.key().as_bytes().len() as u64)
                .saturating_add(row.value_fingerprint().len() as u64)
        });
        let row_count = rows.len() as u64;
        Self {
            source_artifacts_read,
            source_rows_read: row_count,
            source_bytes_read: bytes,
            candidate_rows_written: row_count,
            candidate_bytes_written: bytes,
            canonical_row_order_comparisons: row_count.saturating_sub(1),
            unique_key_comparisons: row_count.saturating_sub(1),
            counter_shape_order_comparisons: (candidate.counter_shape().len() as u64)
                .saturating_sub(1),
        }
    }

    pub const fn source_artifacts_read(self) -> u64 {
        self.source_artifacts_read
    }

    pub const fn source_rows_read(self) -> u64 {
        self.source_rows_read
    }

    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }

    pub const fn candidate_rows_written(self) -> u64 {
        self.candidate_rows_written
    }

    pub const fn candidate_bytes_written(self) -> u64 {
        self.candidate_bytes_written
    }

    pub const fn canonical_row_order_comparisons(self) -> u64 {
        self.canonical_row_order_comparisons
    }

    pub const fn unique_key_comparisons(self) -> u64 {
        self.unique_key_comparisons
    }

    pub const fn counter_shape_order_comparisons(self) -> u64 {
        self.counter_shape_order_comparisons
    }
}
