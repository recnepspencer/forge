use worth_ui_host_contract::UiHostObservation;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiMeasurementEvidenceCategory, UiMeasurementResult};

use super::{UiHostMeasurementNormalizationContext, UiHostMeasurementNormalizationDenial};

pub(crate) fn normalize_host_measurement_evidence(
    observation: UiHostObservation,
    evidence_generation: UiEvidenceAuthorityGeneration,
    normalization_context: UiHostMeasurementNormalizationContext,
) -> Result<UiMeasurementResult, UiHostMeasurementNormalizationDenial> {
    let observed_category =
        UiMeasurementEvidenceCategory::from_request_family(observation.family());
    if observed_category != normalization_context.evidence_category() {
        return Err(UiHostMeasurementNormalizationDenial::CategoryMismatch {
            observed: observed_category,
            normalized: normalization_context.evidence_category(),
        });
    }

    Ok(super::result_construction::construct_measurement_result_from_host_observation(
        observation,
        evidence_generation,
        normalization_context.unit_posture(),
        normalization_context.coordinate_space(),
        normalization_context.rounding_posture(),
        normalization_context.assumption_profile(),
    ))
}
