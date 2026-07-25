//! Host-lane-only measurement result construction. Other modules must use the host transition chain.

use worth_ui_host_contract::{UiHostMeasurementObservation, UiHostMeasurementObservationValue};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{
    UiHostMeasurementResultInput, UiMeasurementEvidenceCategory, UiMeasurementResult,
    UiMeasurementValue,
};

pub(super) fn construct_measurement_result_from_host_observation(
    observation: UiHostMeasurementObservation,
    evidence_generation: UiEvidenceAuthorityGeneration,
    normalization: super::UiHostMeasurementNormalizationContext,
) -> UiMeasurementResult {
    let request_identity = observation.request_identity();
    let request_shape_digest =
        crate::evidence::host_measurement_request_shape_digest(observation.request());
    let evidence_category =
        UiMeasurementEvidenceCategory::from_request_family(observation.family());
    let value = measurement_value_from_host_observation(observation.value().clone());
    let portal_anchor_target_identity = observation
        .request()
        .portal_anchor_rect_input()
        .map(worth_ui_host_contract::UiPortalAnchorRectRequest::target_identity);

    UiMeasurementResult::new_from_host_lane(UiHostMeasurementResultInput {
        request_identity,
        request_shape_digest,
        evidence_category,
        evidence_generation,
        unit_posture: normalization.unit_posture(),
        coordinate_space: normalization.coordinate_space(),
        rounding_posture: normalization.rounding_posture(),
        assumption_profile: normalization.assumption_profile(),
        value,
        portal_anchor_target_identity,
    })
}

fn measurement_value_from_host_observation(
    value: UiHostMeasurementObservationValue,
) -> UiMeasurementValue {
    match value {
        UiHostMeasurementObservationValue::TextIntrinsicSize(value) => {
            UiMeasurementValue::TextIntrinsicSize(value)
        }
        UiHostMeasurementObservationValue::TextBaselineMetrics(value) => {
            UiMeasurementValue::TextBaselineMetrics(value)
        }
        UiHostMeasurementObservationValue::FontMetrics(value) => {
            UiMeasurementValue::FontMetrics(value)
        }
        UiHostMeasurementObservationValue::NativeControlIntrinsicSize(value) => {
            UiMeasurementValue::NativeControlIntrinsicSize(value)
        }
        UiHostMeasurementObservationValue::ViewportExtent(value) => {
            UiMeasurementValue::ViewportExtent(value)
        }
        UiHostMeasurementObservationValue::DpiScaleFactor(value) => {
            UiMeasurementValue::DpiScaleFactor(value)
        }
        UiHostMeasurementObservationValue::PortalAnchorRect(value) => {
            UiMeasurementValue::PortalAnchorRect(value)
        }
        UiHostMeasurementObservationValue::ScrollContainerViewport(value) => {
            UiMeasurementValue::ScrollContainerViewport(value)
        }
    }
}
