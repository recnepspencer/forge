//! Deterministic operational host used by the authority-closure scenario.

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostObservationValue, UiMeasurementRequest,
    UiMeasurementRequestFamily, UiViewportExtentObservation, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter,
};

pub(super) struct AuthorityClosureHost;

impl WorthUiMeasurementHostAdapter for AuthorityClosureHost {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::ViewportExtent => {
                UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: 1280.0,
                    height: 720.0,
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: 1.0,
                })
            }
            family => unreachable!(
                "authority-closure host capability report does not admit {family:?} observation"
            ),
        }
    }
}

impl WorthUiOperationalHostAdapter for AuthorityClosureHost {
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
