#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkloadEvidenceStageLookupCounters {
    required_stage_count: usize,
    indexed_lookup_count: usize,
    raw_row_scan_count: usize,
    rejected_raw_row_scan_count: usize,
    rejected_string_prefix_stage_link_count: usize,
}

impl WorkloadEvidenceStageLookupCounters {
    pub(crate) const fn indexed(required_stage_count: usize) -> Self {
        Self {
            required_stage_count,
            indexed_lookup_count: required_stage_count,
            raw_row_scan_count: 0,
            rejected_raw_row_scan_count: 0,
            rejected_string_prefix_stage_link_count: 0,
        }
    }

    pub const fn required_stage_count(self) -> usize {
        self.required_stage_count
    }

    pub const fn indexed_lookup_count(self) -> usize {
        self.indexed_lookup_count
    }

    pub const fn raw_row_scan_count(self) -> usize {
        self.raw_row_scan_count
    }

    pub const fn rejected_raw_row_scan_count(self) -> usize {
        self.rejected_raw_row_scan_count
    }

    pub const fn rejected_string_prefix_stage_link_count(self) -> usize {
        self.rejected_string_prefix_stage_link_count
    }
}
