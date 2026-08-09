use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
use worth_ui_runtime::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};

#[derive(Clone, Copy, Default)]
pub(super) struct MissingRealtimeHookHost;

impl WorthUiMeasurementHostAdapter for MissingRealtimeHookHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("missing host capabilities deny before observation")
    }
}

impl WorthUiOperationalHostAdapter for MissingRealtimeHookHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::RealtimeOverlayDraw,
            WorthUiHostCapability::RealtimeOverlaySurface,
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
