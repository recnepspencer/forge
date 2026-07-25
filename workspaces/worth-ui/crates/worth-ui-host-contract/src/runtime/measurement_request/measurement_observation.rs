use super::{
    UiHostMeasurementRequest, UiMeasurementEvidenceFamily, UiMeasurementRequestFamily,
    UiMeasurementRequestIdentity,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiTextIntrinsicSizeObservation {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiTextBaselineMetricsObservation {
    pub ascent: f32,
    pub descent: f32,
    pub baseline: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiFontMetricsObservation {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiNativeControlIntrinsicSizeObservation {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiViewportExtentObservation {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiDpiScaleFactorObservation {
    pub scale_factor: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiPortalAnchorRectObservation {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiScrollContainerViewportObservation {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiHostMeasurementObservationValue {
    TextIntrinsicSize(UiTextIntrinsicSizeObservation),
    TextBaselineMetrics(UiTextBaselineMetricsObservation),
    FontMetrics(UiFontMetricsObservation),
    NativeControlIntrinsicSize(UiNativeControlIntrinsicSizeObservation),
    ViewportExtent(UiViewportExtentObservation),
    DpiScaleFactor(UiDpiScaleFactorObservation),
    PortalAnchorRect(UiPortalAnchorRectObservation),
    ScrollContainerViewport(UiScrollContainerViewportObservation),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementObservationContractDenial {
    FamilyMismatch {
        requested: UiMeasurementRequestFamily,
        observed: UiMeasurementRequestFamily,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiHostMeasurementObservation {
    request: UiHostMeasurementRequest,
    value: UiHostMeasurementObservationValue,
}

impl UiHostMeasurementObservationValue {
    pub fn family(&self) -> UiMeasurementRequestFamily {
        match self {
            Self::TextIntrinsicSize(_) => UiMeasurementRequestFamily::TextIntrinsicSize,
            Self::TextBaselineMetrics(_) => UiMeasurementRequestFamily::TextBaselineMetrics,
            Self::FontMetrics(_) => UiMeasurementRequestFamily::FontMetrics,
            Self::NativeControlIntrinsicSize(_) => {
                UiMeasurementRequestFamily::NativeControlIntrinsicSize
            }
            Self::ViewportExtent(_) => UiMeasurementRequestFamily::ViewportExtent,
            Self::DpiScaleFactor(_) => UiMeasurementRequestFamily::DpiScaleFactor,
            Self::PortalAnchorRect(_) => UiMeasurementRequestFamily::PortalAnchorRect,
            Self::ScrollContainerViewport(_) => UiMeasurementRequestFamily::ScrollContainerViewport,
        }
    }
}

impl UiHostMeasurementObservation {
    pub fn from_request(
        request: &UiHostMeasurementRequest,
        value: UiHostMeasurementObservationValue,
    ) -> Result<Self, UiHostMeasurementObservationContractDenial> {
        let observed = value.family();
        if request.family() != observed {
            return Err(UiHostMeasurementObservationContractDenial::FamilyMismatch {
                requested: request.family(),
                observed,
            });
        }

        Ok(Self {
            request: request.clone(),
            value,
        })
    }

    pub fn request_identity(&self) -> UiMeasurementRequestIdentity {
        self.request.identity()
    }

    pub fn family(&self) -> UiMeasurementRequestFamily {
        self.request.family()
    }

    pub fn evidence_family(&self) -> UiMeasurementEvidenceFamily {
        self.request.evidence_family()
    }

    pub fn request(&self) -> &UiHostMeasurementRequest {
        &self.request
    }

    pub fn value(&self) -> &UiHostMeasurementObservationValue {
        &self.value
    }
}
