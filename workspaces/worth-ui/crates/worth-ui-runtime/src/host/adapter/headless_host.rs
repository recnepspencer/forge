use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, WorthUiHostCapability, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

/// Operational host for applications that deliberately expose no native
/// measurement capabilities.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHeadlessHost;

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("headless capability admission denies before native observation")
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiHeadlessHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::CanvasSpatialDraw,
            WorthUiHostCapability::CanvasSpatialHitTest,
            WorthUiHostCapability::CanvasSpatialOverlay,
            WorthUiHostCapability::CanvasSpatialToolState,
            WorthUiHostCapability::CanvasSpatialRenderResource,
            WorthUiHostCapability::RealtimeOverlayDraw,
            WorthUiHostCapability::RealtimeOverlaySurface,
            WorthUiHostCapability::RealtimeOverlayHook,
        ])
    }

    fn perform_surface_registration(
        &self,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::Registered(
            request.confirm_known_empty(),
        )
    }

    fn perform_surface_deregistration(
        &self,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
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
