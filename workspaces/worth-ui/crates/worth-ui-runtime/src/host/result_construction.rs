//! Host-lane-only measurement result construction. Other modules must use the host transition chain.

use worth_ui_host_contract::{UiHostObservation, UiHostObservationValue};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{
    UiMeasurementCoordinateSpace, UiMeasurementEvidenceCategory, UiMeasurementResult,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture, UiMeasurementValue,
};

use super::UiHostMeasurementAssumptionProfile;

pub(super) fn construct_measurement_result_from_host_observation(
    observation: UiHostObservation,
    evidence_generation: UiEvidenceAuthorityGeneration,
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiMeasurementResult {
    let request_identity = observation.request_identity();
    let evidence_category =
        UiMeasurementEvidenceCategory::from_request_family(observation.family());
    let value = measurement_value_from_host_observation(observation.value().clone());

    UiMeasurementResult::new_from_host_lane(
        request_identity,
        evidence_category,
        evidence_generation,
        unit_posture,
        coordinate_space,
        rounding_posture,
        assumption_profile,
        value,
    )
}

fn measurement_value_from_host_observation(value: UiHostObservationValue) -> UiMeasurementValue {
    match value {
        UiHostObservationValue::TextIntrinsicSize(value) => {
            UiMeasurementValue::TextIntrinsicSize(value)
        }
        UiHostObservationValue::TextBaselineMetrics(value) => {
            UiMeasurementValue::TextBaselineMetrics(value)
        }
        UiHostObservationValue::FontMetrics(value) => UiMeasurementValue::FontMetrics(value),
        UiHostObservationValue::NativeControlIntrinsicSize(value) => {
            UiMeasurementValue::NativeControlIntrinsicSize(value)
        }
        UiHostObservationValue::ViewportExtent(value) => UiMeasurementValue::ViewportExtent(value),
        UiHostObservationValue::DpiScaleFactor(value) => UiMeasurementValue::DpiScaleFactor(value),
        UiHostObservationValue::PortalAnchorRect(value) => {
            UiMeasurementValue::PortalAnchorRect(value)
        }
        UiHostObservationValue::ScrollContainerViewport(value) => {
            UiMeasurementValue::ScrollContainerViewport(value)
        }
    }
}
