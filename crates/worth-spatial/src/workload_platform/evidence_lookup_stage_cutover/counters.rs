#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupStageCutoverCounters {
    covered_family_count: usize,
    topology_receipt_ref_count: usize,
    indexed_lookup_count: usize,
    raw_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    caller_owned_scan_count: usize,
    query_artifact_count: usize,
}

impl EvidenceLookupStageCutoverCounters {
    pub(crate) fn new(
        covered_family_count: usize,
        topology_receipt_ref_count: usize,
        indexed_lookup_count: usize,
        raw_row_scan_count: usize,
        broad_receipt_scan_count: usize,
        caller_owned_scan_count: usize,
        query_artifact_count: usize,
    ) -> Self {
        Self {
            covered_family_count,
            topology_receipt_ref_count,
            indexed_lookup_count,
            raw_row_scan_count,
            broad_receipt_scan_count,
            caller_owned_scan_count,
            query_artifact_count,
        }
    }

    pub const fn covered_family_count(&self) -> usize {
        self.covered_family_count
    }

    pub const fn topology_receipt_ref_count(&self) -> usize {
        self.topology_receipt_ref_count
    }

    pub const fn indexed_lookup_count(&self) -> usize {
        self.indexed_lookup_count
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

    pub const fn query_artifact_count(&self) -> usize {
        self.query_artifact_count
    }
}
