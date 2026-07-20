use crate::runtime::WorthUiVisibleRange;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataCounters {
    data_plan_row_count: usize,
    unrelated_plan_row_count: usize,
    family_index_read_count: usize,
    regional_executable_read_count: usize,
    direct_row_lookup_count: usize,
    evidence_reference_lookup_count: usize,
    visible_row_touch_count: usize,
    visible_column_touch_count: usize,
    visible_cell_touch_count: usize,
    full_collection_scan_count: usize,
    offset_pagination_substitute_count: usize,
    query_collection_execution_count: usize,
    diagnostic_materialization_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiVirtualizedDataCounters {
    pub(crate) fn record_data_plan_rows(&mut self, count: usize) {
        self.data_plan_row_count += count;
    }

    pub(crate) fn record_unrelated_plan_rows(&mut self, count: usize) {
        self.unrelated_plan_row_count += count;
    }

    pub(crate) fn record_family_index_read(&mut self) {
        self.family_index_read_count += 1;
    }

    pub(crate) fn record_direct_row_lookup(&mut self) {
        self.direct_row_lookup_count += 1;
    }

    pub(crate) fn record_evidence_reference_lookup(&mut self) {
        self.evidence_reference_lookup_count += 1;
    }

    pub(crate) fn record_visible_range(&mut self, range: WorthUiVisibleRange) {
        self.visible_row_touch_count += range.row_count() as usize;
        self.visible_column_touch_count += range.column_count() as usize;
        self.visible_cell_touch_count += range.row_count() as usize * range.column_count() as usize;
    }

    pub(crate) fn record_certification_failure(&mut self) {
        self.certification_failure_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn data_plan_row_count(self) -> usize {
        self.data_plan_row_count
    }

    pub fn unrelated_plan_row_count(self) -> usize {
        self.unrelated_plan_row_count
    }

    pub fn family_index_read_count(self) -> usize {
        self.family_index_read_count
    }

    pub fn regional_executable_read_count(self) -> usize {
        self.regional_executable_read_count
    }

    pub fn direct_row_lookup_count(self) -> usize {
        self.direct_row_lookup_count
    }

    pub fn evidence_reference_lookup_count(self) -> usize {
        self.evidence_reference_lookup_count
    }

    pub fn visible_row_touch_count(self) -> usize {
        self.visible_row_touch_count
    }

    pub fn visible_column_touch_count(self) -> usize {
        self.visible_column_touch_count
    }

    pub fn visible_cell_touch_count(self) -> usize {
        self.visible_cell_touch_count
    }

    pub fn full_collection_scan_count(self) -> usize {
        self.full_collection_scan_count
    }

    pub fn offset_pagination_substitute_count(self) -> usize {
        self.offset_pagination_substitute_count
    }

    pub fn query_collection_execution_count(self) -> usize {
        self.query_collection_execution_count
    }

    pub fn diagnostic_materialization_count(self) -> usize {
        self.diagnostic_materialization_count
    }

    pub fn certification_failure_count(self) -> usize {
        self.certification_failure_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
