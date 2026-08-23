use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, UiHostSurfaceDeregistrationOutcome,
    UiHostSurfaceDeregistrationReceipt, UiHostSurfaceRegistrationOutcome,
    UiHostSurfaceRegistrationRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

/// A certification-owned inert binding. It is intentionally not installed by
/// the product application entrypoint and therefore cannot act as a hidden
/// production host default.
#[derive(Clone, Copy)]
pub(crate) struct UiCertificationBuilderHost;

impl WorthUiMeasurementHostAdapter for UiCertificationBuilderHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("the certification builder host admits no measurement capability")
    }
}

impl WorthUiHostMechanicsAdapter for UiCertificationBuilderHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        use worth_ui_host_contract::WorthUiHostCapability;

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
        _request: UiHostSurfaceRegistrationRequest,
    ) -> UiHostSurfaceRegistrationOutcome {
        UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    }

    fn perform_surface_deregistration(
        &self,
        request: UiHostSurfaceRegistrationRequest,
    ) -> UiHostSurfaceDeregistrationOutcome {
        UiHostSurfaceDeregistrationOutcome::Deregistered(
            UiHostSurfaceDeregistrationReceipt::from_runtime(
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
