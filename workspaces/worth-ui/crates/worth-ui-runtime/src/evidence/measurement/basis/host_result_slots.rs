use crate::evidence::measurement::UiMeasurementResult;

#[derive(Clone, Copy, Default)]
pub(super) struct HostResultSlots<'a> {
    pub text_intrinsic_size: Option<&'a UiMeasurementResult>,
    pub font_metrics: Option<&'a UiMeasurementResult>,
    pub native_control_intrinsic_size: Option<&'a UiMeasurementResult>,
    pub viewport_extent: Option<&'a UiMeasurementResult>,
    pub portal_anchor_rect: Option<&'a UiMeasurementResult>,
    pub scroll_container_viewport: Option<&'a UiMeasurementResult>,
}

impl<'a> HostResultSlots<'a> {
    pub(super) fn relevant_results(self) -> [Option<&'a UiMeasurementResult>; 6] {
        [
            self.text_intrinsic_size,
            self.font_metrics,
            self.native_control_intrinsic_size,
            self.viewport_extent,
            self.portal_anchor_rect,
            self.scroll_container_viewport,
        ]
    }

    pub(super) fn has_intrinsic_results(self) -> bool {
        self.text_intrinsic_size.is_some() || self.native_control_intrinsic_size.is_some()
    }
}
