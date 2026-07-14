use worth_ui_host_contract::UiMeasurementRequestFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementEvidenceCategory {
    TextIntrinsicSize,
    TextBaselineMetrics,
    FontMetrics,
    NativeControlIntrinsicSize,
    ViewportExtent,
    DpiScaleFactor,
    PortalAnchorRect,
    ScrollContainerViewport,
}

impl UiMeasurementEvidenceCategory {
    pub const fn from_request_family(family: UiMeasurementRequestFamily) -> Self {
        match family {
            UiMeasurementRequestFamily::TextIntrinsicSize => Self::TextIntrinsicSize,
            UiMeasurementRequestFamily::TextBaselineMetrics => Self::TextBaselineMetrics,
            UiMeasurementRequestFamily::FontMetrics => Self::FontMetrics,
            UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
                Self::NativeControlIntrinsicSize
            }
            UiMeasurementRequestFamily::ViewportExtent => Self::ViewportExtent,
            UiMeasurementRequestFamily::DpiScaleFactor => Self::DpiScaleFactor,
            UiMeasurementRequestFamily::PortalAnchorRect => Self::PortalAnchorRect,
            UiMeasurementRequestFamily::ScrollContainerViewport => Self::ScrollContainerViewport,
        }
    }

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
