use crate::runtime::WorthUiVisibleRange;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataCounters {
    data_plan_row_count: usize,
    skipped_nondata_plan_row_count: usize,
    visible_row_touch_count: usize,
    visible_column_touch_count: usize,
    query_patch_row_count: usize,
    full_collection_scan_count: usize,
    offset_pagination_substitute_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiVirtualizedDataCounters {
    pub(crate) fn record_data_plan_row(&mut self) {
        self.data_plan_row_count += 1;
    }

    pub(crate) fn record_skipped_nondata_plan_row(&mut self) {
        self.skipped_nondata_plan_row_count += 1;
    }

    pub(crate) fn record_visible_range(&mut self, range: WorthUiVisibleRange) {
        self.visible_row_touch_count += range.row_count() as usize;
        self.visible_column_touch_count += range.column_count() as usize;
        self.query_patch_row_count += range.row_count() as usize;
    }

    #[cfg(test)]
    pub(crate) fn record_full_collection_scan(&mut self) {
        self.full_collection_scan_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_offset_pagination_substitute(&mut self) {
        self.offset_pagination_substitute_count += 1;
    }

    pub(crate) fn record_certification_failure(&mut self) {
        self.certification_failure_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub(crate) fn merge_plan_counters(&mut self, plan_counters: Self) {
        self.data_plan_row_count = plan_counters.data_plan_row_count;
        self.skipped_nondata_plan_row_count = plan_counters.skipped_nondata_plan_row_count;
    }

    pub fn data_plan_row_count(self) -> usize {
        self.data_plan_row_count
    }

    pub fn skipped_nondata_plan_row_count(self) -> usize {
        self.skipped_nondata_plan_row_count
    }

    pub fn visible_row_touch_count(self) -> usize {
        self.visible_row_touch_count
    }

    pub fn visible_column_touch_count(self) -> usize {
        self.visible_column_touch_count
    }

    pub fn query_patch_row_count(self) -> usize {
        self.query_patch_row_count
    }

    pub fn full_collection_scan_count(self) -> usize {
        self.full_collection_scan_count
    }

    pub fn offset_pagination_substitute_count(self) -> usize {
        self.offset_pagination_substitute_count
    }

    pub fn certification_failure_count(self) -> usize {
        self.certification_failure_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
