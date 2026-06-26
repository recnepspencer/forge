#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRealtimeLaneCounters {
    hud_plan_row_count: usize,
    skipped_nonrealtime_plan_row_count: usize,
    overlay_hook_count: usize,
    renderer_surface_admission_count: usize,
    frame_synchronized_pass_count: usize,
    renderer_surface_handoff_count: usize,
    command_identity_preservation_count: usize,
    accessibility_posture_count: usize,
    diagnostics_posture_count: usize,
    ordinary_layout_pass_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    allocation_count: usize,
    diagnostic_materialization_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiRealtimeLaneCounters {
    pub(crate) fn record_hud_plan_row(&mut self) {
        self.hud_plan_row_count += 1;
    }

    pub(crate) fn record_skipped_nonrealtime_plan_row(&mut self) {
        self.skipped_nonrealtime_plan_row_count += 1;
    }

    pub(crate) fn record_overlay_hook(&mut self) {
        self.overlay_hook_count += 1;
    }

    pub(crate) fn record_renderer_surface_admission(&mut self) {
        self.renderer_surface_admission_count += 1;
    }

    pub(crate) fn record_frame_synchronized_pass(&mut self) {
        self.frame_synchronized_pass_count += 1;
    }

    pub(crate) fn record_renderer_surface_handoff(&mut self) {
        self.renderer_surface_handoff_count += 1;
    }

    pub(crate) fn record_command_identity_preservation(&mut self, count: usize) {
        self.command_identity_preservation_count += count;
    }

    pub(crate) fn record_accessibility_posture(&mut self, count: usize) {
        self.accessibility_posture_count += count;
    }

    pub(crate) fn record_diagnostics_posture(&mut self, count: usize) {
        self.diagnostics_posture_count += count;
    }

    #[cfg(test)]
    pub(crate) fn record_ordinary_layout_pass(&mut self) {
        self.ordinary_layout_pass_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_forbidden_work(&mut self) {
        self.source_parse_count += 1;
        self.registry_lookup_count += 1;
        self.allocation_count += 1;
        self.diagnostic_materialization_count += 1;
    }

    pub(crate) fn record_certification_failure(&mut self) {
        self.certification_failure_count += 1;
        self.record_denial();
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub(crate) fn merge_plan_counters(&mut self, plan_counters: Self) {
        self.hud_plan_row_count = plan_counters.hud_plan_row_count;
        self.skipped_nonrealtime_plan_row_count = plan_counters.skipped_nonrealtime_plan_row_count;
        self.overlay_hook_count = plan_counters.overlay_hook_count;
        self.renderer_surface_admission_count = plan_counters.renderer_surface_admission_count;
        self.command_identity_preservation_count =
            plan_counters.command_identity_preservation_count;
        self.accessibility_posture_count = plan_counters.accessibility_posture_count;
        self.diagnostics_posture_count = plan_counters.diagnostics_posture_count;
    }

    pub fn hud_plan_row_count(self) -> usize {
        self.hud_plan_row_count
    }

    pub fn skipped_nonrealtime_plan_row_count(self) -> usize {
        self.skipped_nonrealtime_plan_row_count
    }

    pub fn overlay_hook_count(self) -> usize {
        self.overlay_hook_count
    }

    pub fn renderer_surface_admission_count(self) -> usize {
        self.renderer_surface_admission_count
    }

    pub fn frame_synchronized_pass_count(self) -> usize {
        self.frame_synchronized_pass_count
    }

    pub fn renderer_surface_handoff_count(self) -> usize {
        self.renderer_surface_handoff_count
    }

    pub fn command_identity_preservation_count(self) -> usize {
        self.command_identity_preservation_count
    }

    pub fn accessibility_posture_count(self) -> usize {
        self.accessibility_posture_count
    }

    pub fn diagnostics_posture_count(self) -> usize {
        self.diagnostics_posture_count
    }

    pub fn ordinary_layout_pass_count(self) -> usize {
        self.ordinary_layout_pass_count
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(self) -> usize {
        self.registry_lookup_count
    }

    pub fn allocation_count(self) -> usize {
        self.allocation_count
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
