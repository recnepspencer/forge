use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProductCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceLookupReuseDecisionCounters {
    product_counters: EvidenceLookupIndexProductCounters,
    compared_basis_dimension_count: usize,
    raw_evidence_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    caller_owned_evidence_work_count: usize,
}

impl EvidenceLookupReuseDecisionCounters {
    pub(crate) fn new(
        product_counters: EvidenceLookupIndexProductCounters,
        compared_basis_dimension_count: usize,
        raw_evidence_row_scan_count: usize,
        broad_receipt_scan_count: usize,
        caller_owned_evidence_work_count: usize,
    ) -> Self {
        Self {
            product_counters,
            compared_basis_dimension_count,
            raw_evidence_row_scan_count,
            broad_receipt_scan_count,
            caller_owned_evidence_work_count,
        }
    }

    pub const fn product_counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.product_counters
    }

    pub const fn compared_basis_dimension_count(&self) -> usize {
        self.compared_basis_dimension_count
    }

    pub const fn raw_evidence_row_scan_count(&self) -> usize {
        self.raw_evidence_row_scan_count
    }

    pub const fn broad_receipt_scan_count(&self) -> usize {
        self.broad_receipt_scan_count
    }

    pub const fn caller_owned_evidence_work_count(&self) -> usize {
        self.caller_owned_evidence_work_count
    }
}
