use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostObservationValue, UiMeasurementRequest,
    UiMeasurementRequestFamily, UiViewportExtentObservation, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter,
};

#[derive(Clone, Default)]
pub struct WorthUiHostEgui {
    context: egui::Context,
}

impl WorthUiHostEgui {
    pub fn new(context: egui::Context) -> Self {
        Self { context }
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiHostEgui {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::ViewportExtent => {
                let size = self.context.input(|input| input.screen_rect().size());
                UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: size.x,
                    height: size.y,
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: self.context.pixels_per_point(),
                })
            }
            family => unreachable!(
                "egui operational capability report does not admit {family:?} observation"
            ),
        }
    }
}

impl WorthUiOperationalHostAdapter for WorthUiHostEgui {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::egui()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::DpiObservation,
            WorthUiHostCapability::ViewportObservation,
        ])
    }
}
