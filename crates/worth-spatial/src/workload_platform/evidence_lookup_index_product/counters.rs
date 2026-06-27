#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupIndexProductCounters {
    selected_basis_row_count: usize,
    total_ledger_row_count: usize,
    indexed_family_count: usize,
    topology_receipt_ref_count: usize,
    query_support_row_count: usize,
    resident_byte_count: usize,
    reused_index_count: usize,
    rebuilt_index_count: usize,
    caller_owned_scan_count: usize,
}

impl EvidenceLookupIndexProductCounters {
    pub(crate) fn new(
        selected_basis_row_count: usize,
        total_ledger_row_count: usize,
        indexed_family_count: usize,
        topology_receipt_ref_count: usize,
        query_support_row_count: usize,
        resident_byte_count: usize,
    ) -> Self {
        Self {
            selected_basis_row_count,
            total_ledger_row_count,
            indexed_family_count,
            topology_receipt_ref_count,
            query_support_row_count,
            resident_byte_count,
            reused_index_count: 0,
            rebuilt_index_count: 1,
            caller_owned_scan_count: 0,
        }
    }

    pub(crate) fn reused_from(self) -> Self {
        Self {
            reused_index_count: self.reused_index_count + 1,
            rebuilt_index_count: 0,
            ..self
        }
    }

    pub fn selected_basis_row_count(&self) -> usize {
        self.selected_basis_row_count
    }

    pub fn total_ledger_row_count(&self) -> usize {
        self.total_ledger_row_count
    }

    pub fn indexed_family_count(&self) -> usize {
        self.indexed_family_count
    }

    pub fn topology_receipt_ref_count(&self) -> usize {
        self.topology_receipt_ref_count
    }

    pub fn query_support_row_count(&self) -> usize {
        self.query_support_row_count
    }

    pub fn resident_byte_count(&self) -> usize {
        self.resident_byte_count
    }

    pub fn reused_index_count(&self) -> usize {
        self.reused_index_count
    }

    pub fn rebuilt_index_count(&self) -> usize {
        self.rebuilt_index_count
    }

    pub fn caller_owned_scan_count(&self) -> usize {
        self.caller_owned_scan_count
    }
}
