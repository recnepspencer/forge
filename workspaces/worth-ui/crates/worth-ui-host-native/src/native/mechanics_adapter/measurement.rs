use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementObservationValue, UiHostMeasurementRequest,
    UiViewportExtentObservation, WorthUiMeasurementHostAdapter,
};

use super::WorthUiNativeMechanicsAdapter;

impl WorthUiMeasurementHostAdapter for WorthUiNativeMechanicsAdapter {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        let state = self.state.borrow();
        let graphics = state
            .presentation_access()
            .expect("native measurement requires a live qualified surface");
        match request.family() {
            worth_ui_host_contract::UiMeasurementRequestFamily::ViewportExtent => {
                let [width, height] = graphics.extent();
                UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: width as f32 / graphics.scale_factor() as f32,
                    height: height as f32 / graphics.scale_factor() as f32,
                })
            }
            worth_ui_host_contract::UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: graphics.scale_factor() as f32,
                })
            }
            _ => unreachable!("the Phase 2 seed admits only viewport and DPI measurement"),
        }
    }
}
