#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryCounters {
    clean_fail_sources: usize,
    admission_rows_consumed: usize,
    recovery_receipts_consumed: usize,
    diagnostic_receipts_consumed: usize,
    repair_attempts_denied: usize,
    bounded_conversions_denied: usize,
}

impl PlanarCleanFailBoundaryCounters {
    pub(crate) fn certified(
        clean_fail_sources: usize,
        admission_rows_consumed: usize,
        recovery_receipts_consumed: usize,
        diagnostic_receipts_consumed: usize,
    ) -> Self {
        Self {
            clean_fail_sources,
            admission_rows_consumed,
            recovery_receipts_consumed,
            diagnostic_receipts_consumed,
            repair_attempts_denied: 0,
            bounded_conversions_denied: 0,
        }
    }

    pub(crate) fn denied_repair() -> Self {
        Self {
            repair_attempts_denied: 1,
            ..Self::certified(0, 0, 0, 0)
        }
    }

    pub(crate) fn denied_bounded_conversion() -> Self {
        Self {
            bounded_conversions_denied: 1,
            ..Self::certified(0, 0, 0, 0)
        }
    }

    pub fn clean_fail_sources(self) -> usize {
        self.clean_fail_sources
    }

    pub fn admission_rows_consumed(self) -> usize {
        self.admission_rows_consumed
    }

    pub fn recovery_receipts_consumed(self) -> usize {
        self.recovery_receipts_consumed
    }

    pub fn diagnostic_receipts_consumed(self) -> usize {
        self.diagnostic_receipts_consumed
    }

    pub fn repair_attempts_denied(self) -> usize {
        self.repair_attempts_denied
    }

    pub fn bounded_conversions_denied(self) -> usize {
        self.bounded_conversions_denied
    }
}
