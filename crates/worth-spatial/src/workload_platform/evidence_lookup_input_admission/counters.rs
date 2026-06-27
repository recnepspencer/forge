use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyStageSelectionCounters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupInputAdmissionCounters {
    catalog_candidate_family_count: usize,
    receipt_family_match_count: usize,
    stage_match_count: usize,
    topology_required_count: usize,
    topology_satisfied_count: usize,
    query_required_count: usize,
    query_satisfied_count: usize,
    denied_before_family_selection_count: usize,
    raw_row_scan_count: usize,
    lookup_product_construction_count: usize,
}

impl EvidenceLookupInputAdmissionCounters {
    pub(crate) fn from_selection(selection: &EvidenceLookupFamilyStageSelectionCounters) -> Self {
        Self {
            catalog_candidate_family_count: selection.candidate_family_count(),
            receipt_family_match_count: selection.receipt_family_match_count(),
            stage_match_count: selection.stage_match_count(),
            ..Self::default()
        }
    }

    pub(crate) fn count_topology_required(&mut self) {
        self.topology_required_count += 1;
    }

    pub(crate) fn count_topology_satisfied(&mut self) {
        self.topology_satisfied_count += 1;
    }

    pub(crate) fn count_query_required(&mut self) {
        self.query_required_count += 1;
    }

    pub(crate) fn count_query_satisfied(&mut self) {
        self.query_satisfied_count += 1;
    }

    pub const fn catalog_candidate_family_count(&self) -> usize {
        self.catalog_candidate_family_count
    }

    pub const fn receipt_family_match_count(&self) -> usize {
        self.receipt_family_match_count
    }

    pub const fn stage_match_count(&self) -> usize {
        self.stage_match_count
    }

    pub const fn topology_required_count(&self) -> usize {
        self.topology_required_count
    }

    pub const fn topology_satisfied_count(&self) -> usize {
        self.topology_satisfied_count
    }

    pub const fn query_required_count(&self) -> usize {
        self.query_required_count
    }

    pub const fn query_satisfied_count(&self) -> usize {
        self.query_satisfied_count
    }

    pub const fn denied_before_family_selection_count(&self) -> usize {
        self.denied_before_family_selection_count
    }

    pub const fn raw_row_scan_count(&self) -> usize {
        self.raw_row_scan_count
    }

    pub const fn lookup_product_construction_count(&self) -> usize {
        self.lookup_product_construction_count
    }
}
