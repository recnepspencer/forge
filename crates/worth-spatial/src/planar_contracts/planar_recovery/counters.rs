#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarRecoveryPostureCounters {
    source_rows_inspected: usize,
    basis_receipts_consumed: usize,
    recovery_rows_emitted: usize,
    rejected_basis_rows: usize,
    recovery_breadth: usize,
}

impl PlanarRecoveryPostureCounters {
    pub(crate) fn certified(
        source_rows_inspected: usize,
        basis_receipts_consumed: usize,
        recovery_rows_emitted: usize,
        recovery_breadth: usize,
    ) -> Self {
        Self {
            source_rows_inspected,
            basis_receipts_consumed,
            recovery_rows_emitted,
            rejected_basis_rows: 0,
            recovery_breadth,
        }
    }

    pub fn source_rows_inspected(self) -> usize {
        self.source_rows_inspected
    }

    pub fn basis_receipts_consumed(self) -> usize {
        self.basis_receipts_consumed
    }

    pub fn recovery_rows_emitted(self) -> usize {
        self.recovery_rows_emitted
    }

    pub fn rejected_basis_rows(self) -> usize {
        self.rejected_basis_rows
    }

    pub fn recovery_breadth(self) -> usize {
        self.recovery_breadth
    }
}
