//! Host observation intake — measurement requests and host contract surfaces.

pub use crate::host::{
    admit_current_host_measurement_evidence, freeze_measurement_request, UiAdmittedHostMeasurement,
    UiHostMeasurementAssumptionProfile, UiHostMeasurementEvidenceDenial,
    UiHostMeasurementExecutionDenial, UiHostMeasurementInvalidationReason, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, UiHostMeasurementNormalizationDenial,
    UiHostPreviewDiscardReason, UiHostPreviewPaintContext, UiHostPreviewPaintDenial,
    UiHostPreviewPaintDenialReport, UiHostPreviewPaintDiscardReport, UiHostPreviewPaintDisposition,
    UiHostPreviewPaintGeometry, UiHostPreviewPaintInput, UiHostPreviewPaintReceipt,
    UiPortalAnchorCoordinateSpacePosture, WorthUiPreviewPaintHost,
};
pub use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiDpiScaleFactorRequest, UiFontMeasurementKey,
    UiFontMetricsObservation, UiFontMetricsRequest, UiForbiddenHostAuthorityAsk, UiHostObservation,
    UiHostObservationContractDenial, UiHostObservationValue, UiMeasurementCapabilityPosture,
    UiMeasurementEvidenceFamily, UiMeasurementRequest, UiMeasurementRequestDenial,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeObservation, UiNativeControlIntrinsicSizeRequest,
    UiNativeControlKind, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiPortalAnchorTargetIdentity, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsObservation,
    UiTextBaselineMetricsRequest, UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest,
    UiViewportExtentObservation, UiViewportExtentRequest, WorthUiCanvasSpatialHostOutput,
    WorthUiCanvasSpatialHostOutputTarget, WorthUiHeadlessHost, WorthUiHostAdapter,
    WorthUiHostCapability, WorthUiHostCapabilityPosture, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostKind, WorthUiHostOutputDisposition, WorthUiHostOutputEnvelope,
    WorthUiHostOutputGeneration, WorthUiHostOutputGenerationDenial,
    WorthUiHostOutputGenerationDenialReason, WorthUiHostOutputLane, WorthUiHostOutputPayload,
    WorthUiHostOutputReceiptReference, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter, WorthUiOrdinaryHostOutput, WorthUiOrdinaryHostOutputTarget,
    WorthUiRealtimeHostOutput, WorthUiVirtualizedDataHostOutput,
};
