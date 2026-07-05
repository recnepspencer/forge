#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementRequestFamily {
    TextIntrinsicSize,
    TextBaselineMetrics,
    FontMetrics,
    NativeControlIntrinsicSize,
    ViewportExtent,
    DpiScaleFactor,
    PortalAnchorRect,
    ScrollContainerViewport,
}

impl UiMeasurementRequestFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextIntrinsicSize => "text-intrinsic-size",
            Self::TextBaselineMetrics => "text-baseline-metrics",
            Self::FontMetrics => "font-metrics",
            Self::NativeControlIntrinsicSize => "native-control-intrinsic-size",
            Self::ViewportExtent => "viewport-extent",
            Self::DpiScaleFactor => "dpi-scale-factor",
            Self::PortalAnchorRect => "portal-anchor-rect",
            Self::ScrollContainerViewport => "scroll-container-viewport",
        }
    }
}
