use crate::declaration::stable_text_digest;
use crate::host::UiHostMeasurementAssumptionProfile;
use worth_ui_host_contract::WorthUiHostCapabilityReport;

use super::{
    UiMeasurementEvidenceCategory, UiMeasurementResult, UiMeasurementValue, UiProjectionFactReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementEvidenceInput {
    QueryProjectionFact(UiProjectionFactReceipt),
    HostMeasurementResult(UiMeasurementResult),
    HostCapabilityReport(WorthUiHostCapabilityReport),
}

impl MeasurementEvidenceInput {
    pub fn query_projection_fact(receipt: &UiProjectionFactReceipt) -> Self {
        Self::QueryProjectionFact(receipt.clone())
    }

    pub fn host_measurement_result(result: &UiMeasurementResult) -> Self {
        Self::HostMeasurementResult(result.clone())
    }

    pub fn host_capability_report(report: &WorthUiHostCapabilityReport) -> Self {
        Self::HostCapabilityReport(report.clone())
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::QueryProjectionFact(receipt) => {
                stable_text_digest("measurement-evidence-input:query-projection-fact")
                    ^ stable_text_digest(receipt.query_basis_digest()).rotate_left(7)
                    ^ stable_text_digest(receipt.projection_contract_digest()).rotate_left(13)
                    ^ stable_text_digest(receipt.projection_consumption_receipt_digest())
                        .rotate_left(19)
                    ^ stable_text_digest(receipt.projection_fact_set_digest()).rotate_left(23)
                    ^ receipt
                        .required_query_fact_family_set_digest()
                        .rotate_left(29)
                    ^ receipt.consumed_fact_family_set_digest().rotate_left(31)
            }
            Self::HostMeasurementResult(result) => {
                stable_text_digest("measurement-evidence-input:host-measurement-result")
                    ^ result.request_identity().as_u64().rotate_left(7)
                    ^ measurement_category_digest(result.evidence_category()).rotate_left(13)
                    ^ result
                        .assumption_profile()
                        .profile_identity_digest()
                        .rotate_left(19)
                    ^ measurement_shape_digest(result).rotate_left(23)
                    ^ measurement_value_digest(result.value()).rotate_left(29)
            }
            Self::HostCapabilityReport(report) => {
                stable_text_digest("measurement-evidence-input:host-capability-report")
                    ^ report.profile_identity_digest().rotate_left(7)
                    ^ report.observation_generation().as_u64().rotate_left(13)
            }
        }
    }

    pub(crate) fn as_query_projection_fact(&self) -> Option<&UiProjectionFactReceipt> {
        match self {
            Self::QueryProjectionFact(receipt) => Some(receipt),
            _ => None,
        }
    }

    pub(crate) fn as_host_measurement_result(&self) -> Option<&UiMeasurementResult> {
        match self {
            Self::HostMeasurementResult(result) => Some(result),
            _ => None,
        }
    }

    pub(crate) fn as_host_capability_report(&self) -> Option<&WorthUiHostCapabilityReport> {
        match self {
            Self::HostCapabilityReport(report) => Some(report),
            _ => None,
        }
    }
}

fn measurement_shape_digest(result: &UiMeasurementResult) -> u64 {
    stable_text_digest("measurement-result-shape")
        ^ result.evidence_generation().as_u64().rotate_left(7)
        ^ unit_posture_digest(result.assumption_profile()).rotate_left(13)
        ^ rounding_coordinate_digest(result).rotate_left(19)
}

fn unit_posture_digest(profile: UiHostMeasurementAssumptionProfile) -> u64 {
    profile.profile_identity_digest()
}

fn rounding_coordinate_digest(result: &UiMeasurementResult) -> u64 {
    stable_text_digest(result.unit_posture().as_str()).rotate_left(7)
        ^ stable_text_digest(result.coordinate_space().as_str()).rotate_left(13)
        ^ stable_text_digest(result.rounding_posture().as_str()).rotate_left(19)
}

fn measurement_category_digest(category: UiMeasurementEvidenceCategory) -> u64 {
    stable_text_digest(match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => "text-intrinsic-size",
        UiMeasurementEvidenceCategory::TextBaselineMetrics => "text-baseline-metrics",
        UiMeasurementEvidenceCategory::FontMetrics => "font-metrics",
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            "native-control-intrinsic-size"
        }
        UiMeasurementEvidenceCategory::ViewportExtent => "viewport-extent",
        UiMeasurementEvidenceCategory::DpiScaleFactor => "dpi-scale-factor",
        UiMeasurementEvidenceCategory::PortalAnchorRect => "portal-anchor-rect",
        UiMeasurementEvidenceCategory::ScrollContainerViewport => "scroll-container-viewport",
    })
}

fn measurement_value_digest(value: &UiMeasurementValue) -> u64 {
    match value {
        UiMeasurementValue::TextIntrinsicSize(value) => {
            stable_text_digest("text-intrinsic-size")
                ^ f32_digest(value.width).rotate_left(7)
                ^ f32_digest(value.height).rotate_left(13)
        }
        UiMeasurementValue::TextBaselineMetrics(value) => {
            stable_text_digest("text-baseline-metrics")
                ^ f32_digest(value.ascent).rotate_left(7)
                ^ f32_digest(value.descent).rotate_left(13)
                ^ f32_digest(value.baseline).rotate_left(19)
        }
        UiMeasurementValue::FontMetrics(value) => {
            stable_text_digest("font-metrics")
                ^ f32_digest(value.ascent).rotate_left(7)
                ^ f32_digest(value.descent).rotate_left(13)
                ^ f32_digest(value.line_gap).rotate_left(19)
        }
        UiMeasurementValue::NativeControlIntrinsicSize(value) => {
            stable_text_digest("native-control-intrinsic-size")
                ^ f32_digest(value.width).rotate_left(7)
                ^ f32_digest(value.height).rotate_left(13)
        }
        UiMeasurementValue::ViewportExtent(value) => {
            stable_text_digest("viewport-extent")
                ^ f32_digest(value.width).rotate_left(7)
                ^ f32_digest(value.height).rotate_left(13)
        }
        UiMeasurementValue::DpiScaleFactor(value) => {
            stable_text_digest("dpi-scale-factor") ^ f32_digest(value.scale_factor).rotate_left(7)
        }
        UiMeasurementValue::PortalAnchorRect(value) => {
            stable_text_digest("portal-anchor-rect")
                ^ f32_digest(value.x).rotate_left(7)
                ^ f32_digest(value.y).rotate_left(13)
                ^ f32_digest(value.width).rotate_left(19)
                ^ f32_digest(value.height).rotate_left(23)
        }
        UiMeasurementValue::ScrollContainerViewport(value) => {
            stable_text_digest("scroll-container-viewport")
                ^ f32_digest(value.width).rotate_left(7)
                ^ f32_digest(value.height).rotate_left(13)
        }
    }
}

const fn f32_digest(value: f32) -> u64 {
    value.to_bits() as u64
}
