use super::{
    UiDpiScaleFactorRequest, UiFontMetricsRequest, UiNativeControlIntrinsicSizeRequest,
    UiPortalAnchorRectRequest, UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest,
    UiTextIntrinsicSizeRequest, UiViewportExtentRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum UiMeasurementRequestPayload {
    TextIntrinsicSize(UiTextIntrinsicSizeRequest),
    TextBaselineMetrics(UiTextBaselineMetricsRequest),
    FontMetrics(UiFontMetricsRequest),
    NativeControlIntrinsicSize(UiNativeControlIntrinsicSizeRequest),
    ViewportExtent(UiViewportExtentRequest),
    DpiScaleFactor(UiDpiScaleFactorRequest),
    PortalAnchorRect(UiPortalAnchorRectRequest),
    ScrollContainerViewport(UiScrollContainerViewportRequest),
}
