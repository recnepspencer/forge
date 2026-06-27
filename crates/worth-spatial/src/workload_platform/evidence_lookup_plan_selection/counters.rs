#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupPlanSelectionCounters {
    candidate_family_count: usize,
    selected_family_count: usize,
    unaffected_family_count: usize,
    denied_family_count: usize,
    required_query_posture_row_count: usize,
    capped_residue_row_count: usize,
    selected_spatial_region_count: usize,
    selected_stage_receipt_count: usize,
    topology_receipt_ref_consumed_count: usize,
    topology_support_rows_consumed_count: usize,
    query_support_rows_consumed_count: usize,
    selected_family_membership_probe_count: usize,
    raw_evidence_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    sparse_lookup_plan_count: usize,
    bounded_dense_lookup_plan_count: usize,
    caller_owned_evidence_work_count: usize,
}

impl EvidenceLookupPlanSelectionCounters {
    pub(crate) fn with_candidate_family_count(candidate_family_count: usize) -> Self {
        Self {
            candidate_family_count,
            ..Self::default()
        }
    }

    pub(crate) fn count_selected_spatial_region(&mut self) {
        self.selected_spatial_region_count += 1;
    }

    pub(crate) fn count_selected_stage_receipt(&mut self) {
        self.selected_stage_receipt_count += 1;
    }

    pub(crate) fn count_topology_receipt_ref_consumed(&mut self) {
        self.topology_receipt_ref_consumed_count += 1;
    }

    pub(crate) fn count_topology_support_row_consumed(&mut self) {
        self.topology_support_rows_consumed_count += 1;
    }

    pub(crate) fn count_query_support_row_consumed(&mut self) {
        self.query_support_rows_consumed_count += 1;
    }

    pub(crate) fn count_selected_family_membership_probe(&mut self) {
        self.selected_family_membership_probe_count += 1;
    }

    pub(crate) fn count_selected_family(&mut self) {
        self.selected_family_count += 1;
    }

    pub(crate) fn count_unaffected_family(&mut self) {
        self.unaffected_family_count += 1;
    }

    pub(crate) fn count_denied_family(&mut self) {
        self.denied_family_count += 1;
    }

    pub(crate) fn count_required_query_posture_row(&mut self) {
        self.required_query_posture_row_count += 1;
    }

    pub(crate) fn count_sparse_lookup_plan(&mut self) {
        self.sparse_lookup_plan_count += 1;
    }

    pub(crate) fn count_bounded_dense_lookup_plan(&mut self) {
        self.bounded_dense_lookup_plan_count += 1;
    }

    pub const fn candidate_family_count(&self) -> usize {
        self.candidate_family_count
    }

    pub const fn selected_family_count(&self) -> usize {
        self.selected_family_count
    }

    pub const fn unaffected_family_count(&self) -> usize {
        self.unaffected_family_count
    }

    pub const fn denied_family_count(&self) -> usize {
        self.denied_family_count
    }

    pub const fn required_query_posture_row_count(&self) -> usize {
        self.required_query_posture_row_count
    }

    pub const fn capped_residue_row_count(&self) -> usize {
        self.capped_residue_row_count
    }

    pub const fn selected_spatial_region_count(&self) -> usize {
        self.selected_spatial_region_count
    }

    pub const fn selected_stage_receipt_count(&self) -> usize {
        self.selected_stage_receipt_count
    }

    pub const fn topology_receipt_ref_consumed_count(&self) -> usize {
        self.topology_receipt_ref_consumed_count
    }

    pub const fn topology_support_rows_consumed_count(&self) -> usize {
        self.topology_support_rows_consumed_count
    }

    pub const fn query_support_rows_consumed_count(&self) -> usize {
        self.query_support_rows_consumed_count
    }

    pub const fn selected_family_membership_probe_count(&self) -> usize {
        self.selected_family_membership_probe_count
    }

    pub const fn raw_evidence_row_scan_count(&self) -> usize {
        self.raw_evidence_row_scan_count
    }

    pub const fn broad_receipt_scan_count(&self) -> usize {
        self.broad_receipt_scan_count
    }

    pub const fn sparse_lookup_plan_count(&self) -> usize {
        self.sparse_lookup_plan_count
    }

    pub const fn bounded_dense_lookup_plan_count(&self) -> usize {
        self.bounded_dense_lookup_plan_count
    }

    pub const fn caller_owned_evidence_work_count(&self) -> usize {
        self.caller_owned_evidence_work_count
    }
}
