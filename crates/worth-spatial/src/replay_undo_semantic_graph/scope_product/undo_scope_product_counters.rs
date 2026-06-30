#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialUndoScopeProductCounters {
    touched_subject_count: usize,
    lookup_consumed_workload_handoff_count: usize,
    raw_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    caller_owned_scan_count: usize,
}

impl SpatialUndoScopeProductCounters {
    pub(crate) const fn new(
        touched_subject_count: usize,
        lookup_consumed_workload_handoff_count: usize,
        raw_row_scan_count: usize,
        broad_receipt_scan_count: usize,
        caller_owned_scan_count: usize,
    ) -> Self {
        Self {
            touched_subject_count,
            lookup_consumed_workload_handoff_count,
            raw_row_scan_count,
            broad_receipt_scan_count,
            caller_owned_scan_count,
        }
    }

    pub const fn touched_subject_count(&self) -> usize {
        self.touched_subject_count
    }

    pub const fn lookup_consumed_workload_handoff_count(&self) -> usize {
        self.lookup_consumed_workload_handoff_count
    }

    pub const fn raw_row_scan_count(&self) -> usize {
        self.raw_row_scan_count
    }

    pub const fn broad_receipt_scan_count(&self) -> usize {
        self.broad_receipt_scan_count
    }

    pub const fn caller_owned_scan_count(&self) -> usize {
        self.caller_owned_scan_count
    }
}
