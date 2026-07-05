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
        }
    }
}
