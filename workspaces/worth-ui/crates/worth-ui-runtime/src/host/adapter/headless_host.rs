use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};

use super::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
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

impl WorthUiOperationalHostAdapter for WorthUiHeadlessHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
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

    fn register_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::Registered(
            request.confirm_known_empty(),
        )
    }

    fn deregister_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
        )
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
