use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, WorthUiHostCapability, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiHeadlessCapabilityProfile {
    MissingCanvasHitTest,
    MissingRealtimeHook,
}

/// Fixed record-only capability profiles used to prove admission denials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHeadlessCapabilityProfileHost {
    profile: UiHeadlessCapabilityProfile,
}

impl WorthUiHeadlessCapabilityProfileHost {
    pub const fn missing_canvas_hit_test() -> Self {
        Self {
            profile: UiHeadlessCapabilityProfile::MissingCanvasHitTest,
        }
    }

    pub const fn missing_realtime_hook() -> Self {
        Self {
            profile: UiHeadlessCapabilityProfile::MissingRealtimeHook,
        }
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessCapabilityProfileHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("fixed capability-denial profiles admit no measurements")
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiHeadlessCapabilityProfileHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        let capabilities = match self.profile {
            UiHeadlessCapabilityProfile::MissingCanvasHitTest => vec![
                WorthUiHostCapability::CanvasSpatialDraw,
                WorthUiHostCapability::CanvasSpatialOverlay,
                WorthUiHostCapability::CanvasSpatialToolState,
                WorthUiHostCapability::CanvasSpatialRenderResource,
            ],
            UiHeadlessCapabilityProfile::MissingRealtimeHook => vec![
                WorthUiHostCapability::RealtimeOverlayDraw,
                WorthUiHostCapability::RealtimeOverlaySurface,
            ],
        };
        WorthUiHostCapabilityReport::available(capabilities)
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
