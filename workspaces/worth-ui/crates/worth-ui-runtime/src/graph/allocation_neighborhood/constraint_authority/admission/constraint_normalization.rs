use crate::evidence::{
    MeasurementEvidenceInput, UiConstraintNormalizationPosture, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiMeasurementBasis, UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

pub(super) fn admit_downward_normalization_posture(
    measurement_basis: &UiMeasurementBasis,
    neighborhood_identity_digest: u64,
    contract_identity_digest: u64,
) -> Result<UiConstraintNormalizationPosture, UiConstraintPropagationDenial> {
    let mut explicit_posture = None;

    for input in measurement_basis.evidence_inputs() {
        let MeasurementEvidenceInput::HostMeasurementResult(result) = input else {
            continue;
        };

        let next = (
            result.unit_posture(),
            result.coordinate_space(),
            result.rounding_posture(),
        );
        if let Some(current) = explicit_posture {
            if current != next {
                return Err(UiConstraintPropagationDenial::new(
                    UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture,
                    neighborhood_identity_digest,
                    contract_identity_digest,
                    None,
                    posture_digest(next.0, next.1, next.2),
                ));
            }
        } else {
            explicit_posture = Some(next);
        }
    }

    Ok(match explicit_posture {
        Some((unit_posture, coordinate_space, rounding_posture)) => {
            UiConstraintNormalizationPosture::explicit(
                unit_posture,
                coordinate_space,
                rounding_posture,
            )
        }
        None => UiConstraintNormalizationPosture::deferred(),
    })
}

fn posture_digest(
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
) -> u64 {
    UiConstraintNormalizationPosture::explicit(unit_posture, coordinate_space, rounding_posture)
        .identity_digest()
}
