#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DerivedIndexParityCounterSnapshot {
    authority_rows_materialized: u64,
    authority_bytes_materialized: u64,
    authority_row_order_comparisons: u64,
    authority_unique_key_comparisons: u64,
    authority_counter_shape_order_comparisons: u64,
    coverage_comparisons: u64,
    key_comparisons: u64,
    value_comparisons: u64,
    counter_shape_comparisons: u64,
    bytes_compared: u64,
}

impl DerivedIndexParityCounterSnapshot {
    pub(super) fn from_authoritative_basis(basis: &super::super::DerivedIndexParityBasis) -> Self {
        let row_count = basis.row_count() as u64;
        let authority_bytes_materialized = basis.ordered_rows().iter().fold(0_u64, |total, row| {
            total
                .saturating_add(row.key().as_bytes().len() as u64)
                .saturating_add(row.value_fingerprint().len() as u64)
        });
        Self {
            authority_rows_materialized: row_count,
            authority_bytes_materialized,
            authority_row_order_comparisons: row_count.saturating_sub(1),
            authority_unique_key_comparisons: row_count.saturating_sub(1),
            authority_counter_shape_order_comparisons: (basis.counter_shape().len() as u64)
                .saturating_sub(1),
            ..Self::default()
        }
    }

    pub(super) fn record_coverage(&mut self) {
        self.coverage_comparisons += 1;
    }

    pub(super) fn record_key(&mut self, left: &[u8], right: &[u8]) {
        self.key_comparisons += 1;
        self.bytes_compared = self
            .bytes_compared
            .saturating_add(left.len() as u64)
            .saturating_add(right.len() as u64);
    }

    pub(super) fn record_value(&mut self, left: &str, right: &str) {
        self.value_comparisons += 1;
        self.bytes_compared = self
            .bytes_compared
            .saturating_add(left.len() as u64)
            .saturating_add(right.len() as u64);
    }

    pub(super) fn record_counter_shape(&mut self, comparisons: usize) {
        self.counter_shape_comparisons = comparisons as u64;
    }

    pub const fn key_comparisons(self) -> u64 {
        self.key_comparisons
    }

    pub const fn authority_rows_materialized(self) -> u64 {
        self.authority_rows_materialized
    }

    pub const fn authority_bytes_materialized(self) -> u64 {
        self.authority_bytes_materialized
    }

    pub const fn authority_row_order_comparisons(self) -> u64 {
        self.authority_row_order_comparisons
    }

    pub const fn authority_unique_key_comparisons(self) -> u64 {
        self.authority_unique_key_comparisons
    }

    pub const fn authority_counter_shape_order_comparisons(self) -> u64 {
        self.authority_counter_shape_order_comparisons
    }

    pub const fn coverage_comparisons(self) -> u64 {
        self.coverage_comparisons
    }

    pub const fn value_comparisons(self) -> u64 {
        self.value_comparisons
    }

    pub const fn counter_shape_comparisons(self) -> u64 {
        self.counter_shape_comparisons
    }

    pub const fn bytes_compared(self) -> u64 {
        self.bytes_compared
    }
}
