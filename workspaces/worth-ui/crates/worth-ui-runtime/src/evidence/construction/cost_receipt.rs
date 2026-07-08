use worth_ui_inspection::UiInspectionCostReceipt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiInspectionCostMetrics {
    index_lookups: usize,
    evidence_refs_considered: usize,
    traversals_denied: usize,
    broad_scan_used: bool,
}

impl UiInspectionCostMetrics {
    pub(crate) const fn new(
        index_lookups: usize,
        evidence_refs_considered: usize,
        traversals_denied: usize,
        broad_scan_used: bool,
    ) -> Self {
        Self {
            index_lookups,
            evidence_refs_considered,
            traversals_denied,
            broad_scan_used,
        }
    }

    pub(crate) fn finalize(
        self,
        evidence_refs_returned: usize,
        materialized_records: usize,
        omitted_by_budget: usize,
    ) -> UiInspectionCostReceipt {
        UiInspectionCostReceipt::new(
            self.index_lookups,
            self.evidence_refs_considered,
            evidence_refs_returned,
            materialized_records,
            omitted_by_budget,
            self.traversals_denied,
            self.broad_scan_used,
        )
    }
}
