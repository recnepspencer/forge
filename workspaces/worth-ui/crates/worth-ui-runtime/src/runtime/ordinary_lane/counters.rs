#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneCounters {
    ordinary_plan_row_count: usize,
    skipped_nonordinary_plan_row_count: usize,
    ordinary_frame_row_touch_count: usize,
    intentional_subtree_row_touch_count: usize,
    root_shell_row_touch_count: usize,
    child_range_touch_count: usize,
    command_surface_touch_count: usize,
    token_support_touch_count: usize,
    state_slot_touch_count: usize,
    text_shape_count: usize,
    glyph_upload_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
    full_plan_scan_count: usize,
    component_string_resolution_count: usize,
    command_string_resolution_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiOrdinaryLaneCounters {
    pub(crate) fn record_ordinary_plan_rows(&mut self, count: usize) {
        self.ordinary_plan_row_count += count;
    }

    pub(crate) fn record_skipped_nonordinary_plan_rows(&mut self, count: usize) {
        self.skipped_nonordinary_plan_row_count += count;
    }

    pub(crate) fn record_frame_row_touch(&mut self) {
        self.ordinary_frame_row_touch_count += 1;
    }

    pub(crate) fn record_intentional_subtree_rows(&mut self, count: usize) {
        self.intentional_subtree_row_touch_count += count;
    }

    pub(crate) fn record_root_shell_rows(&mut self, count: usize) {
        self.root_shell_row_touch_count += count;
    }

    pub(crate) fn record_child_range_touch(&mut self) {
        self.child_range_touch_count += 1;
    }

    pub(crate) fn record_command_surface_touch(&mut self) {
        self.command_surface_touch_count += 1;
    }

    pub(crate) fn record_token_support_touch(&mut self) {
        self.token_support_touch_count += 1;
    }

    pub(crate) fn record_state_slot_touch(&mut self) {
        self.state_slot_touch_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_text_shape(&mut self) {
        self.text_shape_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_glyph_upload(&mut self) {
        self.glyph_upload_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_source_parse(&mut self) {
        self.source_parse_count += 1;
    }

    pub(crate) fn record_certification_failure(&mut self) {
        self.certification_failure_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn ordinary_plan_row_count(self) -> usize {
        self.ordinary_plan_row_count
    }

    pub fn skipped_nonordinary_plan_row_count(self) -> usize {
        self.skipped_nonordinary_plan_row_count
    }

    pub fn ordinary_frame_row_touch_count(self) -> usize {
        self.ordinary_frame_row_touch_count
    }

    pub fn child_range_touch_count(self) -> usize {
        self.child_range_touch_count
    }

    pub fn intentional_subtree_row_touch_count(self) -> usize {
        self.intentional_subtree_row_touch_count
    }

    pub fn root_shell_row_touch_count(self) -> usize {
        self.root_shell_row_touch_count
    }

    pub fn command_surface_touch_count(self) -> usize {
        self.command_surface_touch_count
    }

    pub fn token_support_touch_count(self) -> usize {
        self.token_support_touch_count
    }

    pub fn state_slot_touch_count(self) -> usize {
        self.state_slot_touch_count
    }

    pub fn text_shape_count(self) -> usize {
        self.text_shape_count
    }

    pub fn glyph_upload_count(self) -> usize {
        self.glyph_upload_count
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(self) -> usize {
        self.registry_lookup_count
    }

    pub fn artifact_tree_scan_count(self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn full_plan_scan_count(self) -> usize {
        self.full_plan_scan_count
    }

    pub fn component_string_resolution_count(self) -> usize {
        self.component_string_resolution_count
    }

    pub fn command_string_resolution_count(self) -> usize {
        self.command_string_resolution_count
    }

    pub fn certification_failure_count(self) -> usize {
        self.certification_failure_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
