#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialCounters {
    canvas_plan_row_count: usize,
    skipped_noncanvas_plan_row_count: usize,
    draw_hook_count: usize,
    hit_test_hook_count: usize,
    tool_state_hook_count: usize,
    overlay_plan_count: usize,
    viewport_transform_count: usize,
    draw_pass_count: usize,
    spatial_hit_test_count: usize,
    tool_state_attachment_count: usize,
    command_identity_preservation_count: usize,
    selection_identity_preservation_count: usize,
    diagnostics_posture_count: usize,
    renderer_reference_count: usize,
    domain_geometry_truth_read_count: usize,
    renderer_internal_read_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiCanvasSpatialCounters {
    pub(crate) fn record_canvas_plan_row(&mut self) {
        self.canvas_plan_row_count += 1;
    }

    pub(crate) fn record_skipped_noncanvas_plan_row(&mut self) {
        self.skipped_noncanvas_plan_row_count += 1;
    }

    pub(crate) fn record_admitted_hook_family(&mut self) {
        self.draw_hook_count += 1;
        self.hit_test_hook_count += 1;
        self.tool_state_hook_count += 1;
    }

    pub(crate) fn record_overlay_plan(&mut self) {
        self.overlay_plan_count += 1;
    }

    pub(crate) fn record_viewport_transform(&mut self) {
        self.viewport_transform_count += 1;
    }

    pub(crate) fn record_draw_pass(&mut self) {
        self.draw_pass_count += 1;
    }

    pub(crate) fn record_spatial_hit_test(&mut self) {
        self.spatial_hit_test_count += 1;
    }

    pub(crate) fn record_tool_state_attachment(&mut self) {
        self.tool_state_attachment_count += 1;
        self.selection_identity_preservation_count += 1;
    }

    pub(crate) fn record_command_identity_preservation(&mut self, count: usize) {
        self.command_identity_preservation_count += count;
    }

    pub(crate) fn record_diagnostics_posture(&mut self, count: usize) {
        self.diagnostics_posture_count += count;
    }

    pub(crate) fn record_renderer_reference(&mut self, count: usize) {
        self.renderer_reference_count += count;
    }

    #[cfg(test)]
    pub(crate) fn record_domain_geometry_truth_read(&mut self) {
        self.domain_geometry_truth_read_count += 1;
        self.record_denial();
    }

    #[cfg(test)]
    pub(crate) fn record_renderer_internal_read(&mut self) {
        self.renderer_internal_read_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_certification_failure(&mut self) {
        self.certification_failure_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub(crate) fn merge_plan_counters(&mut self, plan_counters: Self) {
        self.canvas_plan_row_count = plan_counters.canvas_plan_row_count;
        self.skipped_noncanvas_plan_row_count = plan_counters.skipped_noncanvas_plan_row_count;
        self.draw_hook_count = plan_counters.draw_hook_count;
        self.hit_test_hook_count = plan_counters.hit_test_hook_count;
        self.tool_state_hook_count = plan_counters.tool_state_hook_count;
        self.overlay_plan_count = plan_counters.overlay_plan_count;
        self.viewport_transform_count = plan_counters.viewport_transform_count;
        self.command_identity_preservation_count =
            plan_counters.command_identity_preservation_count;
        self.diagnostics_posture_count = plan_counters.diagnostics_posture_count;
        self.renderer_reference_count = plan_counters.renderer_reference_count;
    }

    pub fn canvas_plan_row_count(self) -> usize {
        self.canvas_plan_row_count
    }

    pub fn skipped_noncanvas_plan_row_count(self) -> usize {
        self.skipped_noncanvas_plan_row_count
    }

    pub fn draw_hook_count(self) -> usize {
        self.draw_hook_count
    }

    pub fn hit_test_hook_count(self) -> usize {
        self.hit_test_hook_count
    }

    pub fn tool_state_hook_count(self) -> usize {
        self.tool_state_hook_count
    }

    pub fn overlay_plan_count(self) -> usize {
        self.overlay_plan_count
    }

    pub fn viewport_transform_count(self) -> usize {
        self.viewport_transform_count
    }

    pub fn draw_pass_count(self) -> usize {
        self.draw_pass_count
    }

    pub fn spatial_hit_test_count(self) -> usize {
        self.spatial_hit_test_count
    }

    pub fn tool_state_attachment_count(self) -> usize {
        self.tool_state_attachment_count
    }

    pub fn command_identity_preservation_count(self) -> usize {
        self.command_identity_preservation_count
    }

    pub fn selection_identity_preservation_count(self) -> usize {
        self.selection_identity_preservation_count
    }

    pub fn diagnostics_posture_count(self) -> usize {
        self.diagnostics_posture_count
    }

    pub fn renderer_reference_count(self) -> usize {
        self.renderer_reference_count
    }

    pub fn domain_geometry_truth_read_count(self) -> usize {
        self.domain_geometry_truth_read_count
    }

    pub fn renderer_internal_read_count(self) -> usize {
        self.renderer_internal_read_count
    }

    pub fn certification_failure_count(self) -> usize {
        self.certification_failure_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
