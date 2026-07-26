//! Deterministic operational host used by the authority-closure scenario.

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementObservationValue, UiHostMeasurementRequest,
    UiMeasurementRequestFamily, UiViewportExtentObservation, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
use worth_ui_runtime::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};

#[derive(Clone, Copy, Default)]
pub(super) struct AuthorityClosureHost;

impl WorthUiMeasurementHostAdapter for AuthorityClosureHost {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::ViewportExtent => {
                UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: 1280.0,
                    height: 720.0,
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
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

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            0,
        ))
    }
}
