#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRealtimeLaneCounters {
    hud_plan_row_count: usize,
    overlay_hook_count: usize,
    renderer_surface_admission_count: usize,
    frame_synchronized_pass_count: usize,
    renderer_surface_handoff_count: usize,
    targeted_overlay_row_count: usize,
    ordinary_layout_pass_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    allocation_count: usize,
    diagnostic_materialization_count: usize,
    certification_failure_count: usize,
    denial_count: usize,
}

impl WorthUiRealtimeLaneCounters {
    pub(crate) fn record_plan_rows(&mut self, count: usize) {
        self.hud_plan_row_count += count;
        self.overlay_hook_count += count;
        self.renderer_surface_admission_count += count;
    }

    pub(crate) fn record_frame_synchronized_pass(&mut self) {
        self.frame_synchronized_pass_count += 1;
    }

    pub(crate) fn record_renderer_surface_handoff(&mut self) {
        self.renderer_surface_handoff_count += 1;
    }

    pub(crate) fn record_targeted_overlay_rows(&mut self, count: u16) {
        self.targeted_overlay_row_count += usize::from(count);
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

    pub fn hud_plan_row_count(self) -> usize {
        self.hud_plan_row_count
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

    pub fn targeted_overlay_row_count(self) -> usize {
        self.targeted_overlay_row_count
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
