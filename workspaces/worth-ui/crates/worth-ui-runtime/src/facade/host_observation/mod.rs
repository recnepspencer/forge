//! Host observation intake — measurement requests and host contract surfaces.

pub use crate::host::{
    admit_current_host_measurement_evidence, collect_host_measurement_evidence,
    freeze_measurement_request, UiHostMeasurementAssumptionProfile,
    UiHostMeasurementEvidenceDenial, UiHostMeasurementExecutionDenial,
    UiHostMeasurementFreshnessWitness, UiHostMeasurementInvalidationReason, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, UiHostMeasurementNormalizationDenial,
};
pub use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiDpiScaleFactorRequest, UiFontMeasurementKey,
    UiFontMetricsObservation, UiFontMetricsRequest, UiForbiddenHostAuthorityAsk, UiHostObservation,
    UiHostObservationContractDenial, UiHostObservationValue, UiMeasurementCapabilityPosture,
    UiMeasurementEvidenceFamily, UiMeasurementRequest, UiMeasurementRequestDenial,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeObservation, UiNativeControlIntrinsicSizeRequest,
    UiNativeControlKind, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiScrollContainerViewportObservation, UiScrollContainerViewportRequest,
    UiTextBaselineMetricsObservation, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeObservation,
    UiTextIntrinsicSizeRequest, UiViewportExtentObservation, UiViewportExtentRequest,
    WorthUiHostAdapter, WorthUiHostCapability, WorthUiHostCapabilityPosture,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};