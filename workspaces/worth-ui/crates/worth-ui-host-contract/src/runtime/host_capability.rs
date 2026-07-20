#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiHostCapability {
    TextInput,
    Ime,
    Accessibility,
    FontMetrics,
    TextIntrinsicMeasurement,
    TextBaselineMeasurement,
    NativeControlIntrinsicMeasurement,
    ViewportObservation,
    DpiObservation,
    PortalAnchorObservation,
    ScrollContainerObservation,
    VisualCapture,
    CanvasSpatialDraw,
    CanvasSpatialHitTest,
    CanvasSpatialOverlay,
    CanvasSpatialToolState,
    CanvasSpatialRenderResource,
    RealtimeOverlayDraw,
    RealtimeOverlaySurface,
    RealtimeOverlayHook,
}

impl WorthUiHostCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextInput => "text-input",
            Self::Ime => "ime",
            Self::Accessibility => "accessibility",
            Self::FontMetrics => "font-metrics",
            Self::TextIntrinsicMeasurement => "text-intrinsic-measurement",
            Self::TextBaselineMeasurement => "text-baseline-measurement",
            Self::NativeControlIntrinsicMeasurement => "native-control-intrinsic-measurement",
            Self::ViewportObservation => "viewport-observation",
            Self::DpiObservation => "dpi-observation",
            Self::PortalAnchorObservation => "portal-anchor-observation",
            Self::ScrollContainerObservation => "scroll-container-observation",
            Self::VisualCapture => "visual-capture",
            Self::CanvasSpatialDraw => "canvas-spatial-draw",
            Self::CanvasSpatialHitTest => "canvas-spatial-hit-test",
            Self::CanvasSpatialOverlay => "canvas-spatial-overlay",
            Self::CanvasSpatialToolState => "canvas-spatial-tool-state",
            Self::CanvasSpatialRenderResource => "canvas-spatial-render-resource",
            Self::RealtimeOverlayDraw => "realtime-overlay-draw",
            Self::RealtimeOverlaySurface => "realtime-overlay-surface",
            Self::RealtimeOverlayHook => "realtime-overlay-hook",
        }
    }
}
