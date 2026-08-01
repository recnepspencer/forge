use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementObservationValue, UiHostMeasurementRequest,
    UiMeasurementRequestFamily, UiViewportExtentObservation, WorthUiMeasurementHostAdapter,
};

impl WorthUiMeasurementHostAdapter for super::WorthUiHostEgui {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::ViewportExtent => {
                let size = self.context.input(|input| input.viewport_rect().size());
                UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: size.x,
                    height: size.y,
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: self.context.pixels_per_point(),
                })
            }
            family => unreachable!(
                "egui operational capability report does not admit {family:?} observation"
            ),
        }
    }
}
