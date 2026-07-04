#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiInspectionCostReceipt {
    index_lookups: usize,
    evidence_refs_considered: usize,
    evidence_refs_returned: usize,
    materialized_records: usize,
    omitted_by_budget: usize,
    traversals_denied: usize,
    broad_scan_used: bool,
}

impl UiInspectionCostReceipt {
    pub const fn new(
        index_lookups: usize,
        evidence_refs_considered: usize,
        evidence_refs_returned: usize,
        materialized_records: usize,
        omitted_by_budget: usize,
        traversals_denied: usize,
        broad_scan_used: bool,
    ) -> Self {
        Self {
            index_lookups,
            evidence_refs_considered,
            evidence_refs_returned,
            materialized_records,
            omitted_by_budget,
            traversals_denied,
            broad_scan_used,
        }
    }

    pub fn index_lookups(self) -> usize {
        self.index_lookups
    }

    pub fn evidence_refs_considered(self) -> usize {
        self.evidence_refs_considered
    }

    pub fn evidence_refs_returned(self) -> usize {
        self.evidence_refs_returned
    }

    pub fn materialized_records(self) -> usize {
        self.materialized_records
    }

    pub fn omitted_by_budget(self) -> usize {
        self.omitted_by_budget
    }

    pub fn traversals_denied(self) -> usize {
        self.traversals_denied
    }

    pub fn broad_scan_used(self) -> bool {
        self.broad_scan_used
    }
}
