use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, UiHostSurfaceRegistrationDenial, UiHostSurfaceRegistrationOutcome,
    UiHostSurfaceRegistrationRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

/// Fixed record-only host that cannot establish a known-empty surface baseline.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHeadlessBaselineUnavailableHost;

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessBaselineUnavailableHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("baseline-denial host admits no measurements")
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiHeadlessBaselineUnavailableHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(Vec::new())
    }

    fn perform_surface_registration(
        &self,
        _request: UiHostSurfaceRegistrationRequest,
    ) -> UiHostSurfaceRegistrationOutcome {
        UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
            UiHostSurfaceRegistrationDenial::KnownEmptyBaselineUnavailable,
        )
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            0,
        ))
    }
}
