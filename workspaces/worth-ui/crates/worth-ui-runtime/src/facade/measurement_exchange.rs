//! Solicited host measurement request/response exchange.

pub use crate::host::adapter::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseIndeterminate, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, WorthUiHostAdapter, WorthUiOperationalHostAdapter,
};
pub use crate::host_exchange::measurement_admission::{
    UiHostMeasurementCompletion, UiHostMeasurementDenial, UiHostMeasurementIngressDenial,
    UiHostMeasurementIntent, UiHostMeasurementOutcome, UiRequestedHostMeasurement,
    UiSolicitedHostMeasurementResult, WorthUiHostMeasurementIngress,
};
pub use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiDpiScaleFactorRequest, UiFontMeasurementKey,
    UiFontMetricsObservation, UiFontMetricsRequest, UiForbiddenHostAuthorityAsk,
    UiHostMeasurementDeadline, UiHostMeasurementObservation,
    UiHostMeasurementObservationContractDenial, UiHostMeasurementObservationValue,
    UiHostMeasurementRequest, UiHostMeasurementRequestIntent, UiMeasurementCapabilityPosture,
    UiMeasurementEvidenceFamily, UiMeasurementRequestDenial, UiMeasurementRequestFamily,
    UiMeasurementRequestIdentity, UiNativeControlIntrinsicSizeObservation,
    UiNativeControlIntrinsicSizeRequest, UiNativeControlKind, UiPortalAnchorRectObservation,
    UiPortalAnchorRectRequest, UiPortalAnchorTargetIdentity, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsObservation,
    UiTextBaselineMetricsRequest, UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest,
    UiViewportExtentObservation, UiViewportExtentRequest, WorthUiHostCapability,
    WorthUiHostCapabilityPosture, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostKind, WorthUiMeasurementHostAdapter,
};

/// Host-integrator access to the solicited measurement exchange owned by an
/// active application session.
pub trait WorthUiHostMeasurementSessionExt {
    fn host_measurement_ingress(&self) -> WorthUiHostMeasurementIngress;

    fn begin_host_measurement(
        &mut self,
        intent: UiHostMeasurementIntent,
        now: u64,
    ) -> UiHostMeasurementOutcome;

    fn complete_host_measurement(
        &mut self,
        observation: UiHostMeasurementObservation,
        now: u64,
    ) -> UiHostMeasurementOutcome;

    fn cancel_host_measurement(
        &mut self,
        identity: UiMeasurementRequestIdentity,
    ) -> UiHostMeasurementOutcome;

    fn expire_host_measurements(&mut self, now: u64) -> Box<[UiHostMeasurementOutcome]>;

    fn pending_host_measurement_count(&self) -> usize;

    fn pending_host_measurement_bytes(&self) -> usize;

    fn complete_enqueued_host_measurements(&mut self) -> Box<[UiHostMeasurementOutcome]>;
}

impl WorthUiHostMeasurementSessionExt for crate::facade::WorthUiActiveApplicationSession {
    fn host_measurement_ingress(&self) -> WorthUiHostMeasurementIngress {
        crate::facade::WorthUiActiveApplicationSession::host_measurement_ingress(self)
    }

    fn begin_host_measurement(
        &mut self,
        intent: UiHostMeasurementIntent,
        now: u64,
    ) -> UiHostMeasurementOutcome {
        crate::facade::WorthUiActiveApplicationSession::begin_host_measurement(self, intent, now)
    }

    fn complete_host_measurement(
        &mut self,
        observation: UiHostMeasurementObservation,
        now: u64,
    ) -> UiHostMeasurementOutcome {
        crate::facade::WorthUiActiveApplicationSession::complete_host_measurement(
            self,
            observation,
            now,
        )
    }

    fn cancel_host_measurement(
        &mut self,
        identity: UiMeasurementRequestIdentity,
    ) -> UiHostMeasurementOutcome {
        crate::facade::WorthUiActiveApplicationSession::cancel_host_measurement(self, identity)
    }

    fn expire_host_measurements(&mut self, now: u64) -> Box<[UiHostMeasurementOutcome]> {
        crate::facade::WorthUiActiveApplicationSession::expire_host_measurements(self, now)
    }

    fn pending_host_measurement_count(&self) -> usize {
        crate::facade::WorthUiActiveApplicationSession::pending_host_measurement_count(self)
    }

    fn pending_host_measurement_bytes(&self) -> usize {
        crate::facade::WorthUiActiveApplicationSession::pending_host_measurement_bytes(self)
    }

    fn complete_enqueued_host_measurements(&mut self) -> Box<[UiHostMeasurementOutcome]> {
        crate::facade::WorthUiActiveApplicationSession::complete_enqueued_host_measurements(self)
    }
}
