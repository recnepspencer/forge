use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiFontMetricsObservation, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeObservation, UiPortalAnchorRectObservation,
    UiScrollContainerViewportObservation, UiTextBaselineMetricsObservation,
    UiTextIntrinsicSizeObservation, UiViewportExtentObservation,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::{
    UiMeasurementCoordinateSpace, UiMeasurementEvidenceCategory, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};
use crate::host::UiHostMeasurementAssumptionProfile;

#[derive(Clone, Debug, PartialEq)]
pub enum UiMeasurementValue {
    TextIntrinsicSize(UiTextIntrinsicSizeObservation),
    TextBaselineMetrics(UiTextBaselineMetricsObservation),
    FontMetrics(UiFontMetricsObservation),
    NativeControlIntrinsicSize(UiNativeControlIntrinsicSizeObservation),
    ViewportExtent(UiViewportExtentObservation),
    DpiScaleFactor(UiDpiScaleFactorObservation),
    PortalAnchorRect(UiPortalAnchorRectObservation),
    ScrollContainerViewport(UiScrollContainerViewportObservation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMeasurementResult {
    request_identity: UiMeasurementRequestIdentity,
    evidence_category: UiMeasurementEvidenceCategory,
    evidence_generation: UiEvidenceAuthorityGeneration,
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
    assumption_profile: UiHostMeasurementAssumptionProfile,
    value: UiMeasurementValue,
}

#[derive(Debug, PartialEq)]
pub struct UiCurrentMeasurementResult<'a> {
    result: &'a UiMeasurementResult,
}

impl UiMeasurementValue {
    pub fn category(&self) -> UiMeasurementEvidenceCategory {
        match self {
            Self::TextIntrinsicSize(_) => UiMeasurementEvidenceCategory::TextIntrinsicSize,
            Self::TextBaselineMetrics(_) => UiMeasurementEvidenceCategory::TextBaselineMetrics,
            Self::FontMetrics(_) => UiMeasurementEvidenceCategory::FontMetrics,
            Self::NativeControlIntrinsicSize(_) => {
                UiMeasurementEvidenceCategory::NativeControlIntrinsicSize
            }
            Self::ViewportExtent(_) => UiMeasurementEvidenceCategory::ViewportExtent,
            Self::DpiScaleFactor(_) => UiMeasurementEvidenceCategory::DpiScaleFactor,
            Self::PortalAnchorRect(_) => UiMeasurementEvidenceCategory::PortalAnchorRect,
            Self::ScrollContainerViewport(_) => {
                UiMeasurementEvidenceCategory::ScrollContainerViewport
            }
        }
    }
}

impl UiMeasurementResult {
    pub(crate) fn new_from_host_lane(
        request_identity: UiMeasurementRequestIdentity,
        evidence_category: UiMeasurementEvidenceCategory,
        evidence_generation: UiEvidenceAuthorityGeneration,
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
        assumption_profile: UiHostMeasurementAssumptionProfile,
        value: UiMeasurementValue,
    ) -> Self {
        Self {
            request_identity,
            evidence_category,
            evidence_generation,
            unit_posture,
            coordinate_space,
            rounding_posture,
            assumption_profile,
            value,
        }
    }

    pub(crate) fn request_identity(&self) -> UiMeasurementRequestIdentity {
        self.request_identity
    }

    pub(crate) fn evidence_category(&self) -> UiMeasurementEvidenceCategory {
        self.evidence_category
    }

    pub(crate) fn evidence_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.evidence_generation
    }

    pub(crate) fn unit_posture(&self) -> UiMeasurementUnitPosture {
        self.unit_posture
    }

    pub(crate) fn coordinate_space(&self) -> UiMeasurementCoordinateSpace {
        self.coordinate_space
    }

    pub(crate) fn rounding_posture(&self) -> UiMeasurementRoundingPosture {
        self.rounding_posture
    }

    pub(crate) fn assumption_profile(&self) -> UiHostMeasurementAssumptionProfile {
        self.assumption_profile
    }

    pub(crate) fn value(&self) -> &UiMeasurementValue {
        &self.value
    }
}

impl<'a> UiCurrentMeasurementResult<'a> {
    pub(crate) fn new(result: &'a UiMeasurementResult) -> Self {
        Self { result }
    }

    pub fn request_identity(&self) -> UiMeasurementRequestIdentity {
        self.result.request_identity()
    }

    pub fn evidence_category(&self) -> UiMeasurementEvidenceCategory {
        self.result.evidence_category()
    }

    pub fn evidence_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.result.evidence_generation()
    }

    pub fn unit_posture(&self) -> UiMeasurementUnitPosture {
        self.result.unit_posture()
    }

    pub fn coordinate_space(&self) -> UiMeasurementCoordinateSpace {
        self.result.coordinate_space()
    }

    pub fn rounding_posture(&self) -> UiMeasurementRoundingPosture {
        self.result.rounding_posture()
    }

    pub fn assumption_profile(&self) -> UiHostMeasurementAssumptionProfile {
        self.result.assumption_profile()
    }

    pub fn value(&self) -> &UiMeasurementValue {
        self.result.value()
    }
}
