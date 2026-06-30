#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReplayScopeProductCounters {
    touched_subject_count: usize,
    covered_family_count: usize,
    indexed_lookup_count: usize,
    topology_receipt_ref_count: usize,
    raw_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    caller_owned_scan_count: usize,
    retained_replay_binding_count: usize,
}

impl SpatialReplayScopeProductCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        touched_subject_count: usize,
        covered_family_count: usize,
        indexed_lookup_count: usize,
        topology_receipt_ref_count: usize,
        raw_row_scan_count: usize,
        broad_receipt_scan_count: usize,
        caller_owned_scan_count: usize,
        retained_replay_binding_count: usize,
    ) -> Self {
        Self {
            touched_subject_count,
            covered_family_count,
            indexed_lookup_count,
            topology_receipt_ref_count,
            raw_row_scan_count,
            broad_receipt_scan_count,
            caller_owned_scan_count,
            retained_replay_binding_count,
        }
    }

    pub const fn touched_subject_count(&self) -> usize {
        self.touched_subject_count
    }

    pub const fn covered_family_count(&self) -> usize {
        self.covered_family_count
    }

    pub const fn indexed_lookup_count(&self) -> usize {
        self.indexed_lookup_count
    }

    pub const fn topology_receipt_ref_count(&self) -> usize {
        self.topology_receipt_ref_count
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

    pub const fn retained_replay_binding_count(&self) -> usize {
        self.retained_replay_binding_count
    }
}
