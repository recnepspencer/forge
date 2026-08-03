use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub(in crate::physical_runtime::record_serving::residency) struct PhysicalWritebackCounterCells {
    attempts: AtomicU64,
    exact_receipts: AtomicU64,
    retryable: AtomicU64,
    indeterminate: AtomicU64,
    inspection_required: AtomicU64,
}

/// Dirty-frame writeback attempts and terminal outcome counts.
///
/// Exact receipts, retryable outcomes, and inspection-required outcomes remain
/// distinct so observation cannot turn an indeterminate effect into success.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhysicalWritebackCounterSnapshot {
    attempts: u64,
    exact_receipts: u64,
    retryable: u64,
    indeterminate: u64,
    inspection_required: u64,
}

impl PhysicalWritebackCounterCells {
    pub(in crate::physical_runtime::record_serving::residency) fn observe_attempt(&self) {
        self.attempts.fetch_add(1, Ordering::AcqRel);
    }

    pub(in crate::physical_runtime::record_serving::residency) fn observe_exact_receipt(&self) {
        self.exact_receipts.fetch_add(1, Ordering::AcqRel);
    }

    pub(in crate::physical_runtime::record_serving::residency) fn observe_retryable(&self) {
        self.retryable.fetch_add(1, Ordering::AcqRel);
    }

    pub(in crate::physical_runtime::record_serving::residency) fn observe_inspection_required(
        &self,
        indeterminate: bool,
    ) {
        self.inspection_required.fetch_add(1, Ordering::AcqRel);
        if indeterminate {
            self.indeterminate.fetch_add(1, Ordering::AcqRel);
        }
    }

    pub(in crate::physical_runtime::record_serving::residency) fn snapshot(
        &self,
    ) -> PhysicalWritebackCounterSnapshot {
        PhysicalWritebackCounterSnapshot {
            attempts: self.attempts.load(Ordering::Acquire),
            exact_receipts: self.exact_receipts.load(Ordering::Acquire),
            retryable: self.retryable.load(Ordering::Acquire),
            indeterminate: self.indeterminate.load(Ordering::Acquire),
            inspection_required: self.inspection_required.load(Ordering::Acquire),
        }
    }
}

impl PhysicalWritebackCounterSnapshot {
    pub const fn attempts(self) -> u64 {
        self.attempts
    }

    pub const fn exact_receipts(self) -> u64 {
        self.exact_receipts
    }

    pub const fn retryable(self) -> u64 {
        self.retryable
    }

    pub const fn indeterminate(self) -> u64 {
        self.indeterminate
    }

    pub const fn inspection_required(self) -> u64 {
        self.inspection_required
    }
}
