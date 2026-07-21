use super::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter,
};

/// Operational host for applications that deliberately expose no native
/// measurement capabilities.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHeadlessHost;

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessHost {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
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

    fn consume_output(
        &self,
        _output: &super::WorthUiHostOutputEnvelope,
    ) -> super::WorthUiHostOutputDisposition {
        super::WorthUiHostOutputDisposition::Consumed
    }
}
