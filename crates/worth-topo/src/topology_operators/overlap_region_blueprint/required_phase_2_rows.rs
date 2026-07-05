use super::closeout::PlanarBooleanOverlapBlueprintCloseoutDenial;
use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::required_phase_2_operator_lanes::REQUIRED_PHASE_2_OPERATOR_LANES;
use super::required_phase_2_validator_lanes::REQUIRED_PHASE_2_VALIDATOR_LANES;
use super::validator_row::PlanarBooleanOverlapValidatorRow;

pub(super) fn require_phase_2_operator_rows(
    operators: &[PlanarBooleanOverlapOperatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    for (required_operator, _, _) in REQUIRED_PHASE_2_OPERATOR_LANES {
        if !operators
            .iter()
            .any(|operator| operator.operator_name() == *required_operator)
        {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::MissingRequiredOperator);
        }
    }
    if operators.len() != REQUIRED_PHASE_2_OPERATOR_LANES.len() {
        return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::UnexpectedOperatorName);
    }
    Ok(())
}

pub(super) fn require_phase_2_validator_rows(
    validators: &[PlanarBooleanOverlapValidatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    for (required_validator, _, _) in REQUIRED_PHASE_2_VALIDATOR_LANES {
        if !validators
            .iter()
            .any(|validator| validator.validator_name() == *required_validator)
        {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::MissingRequiredValidator);
        }
    }
    if validators.len() != REQUIRED_PHASE_2_VALIDATOR_LANES.len() {
        return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::UnexpectedValidatorName);
    }
    Ok(())
}

pub(super) fn require_phase_2_operator_lanes(
    operators: &[PlanarBooleanOverlapOperatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    for (required_operator, classification, query_surface) in REQUIRED_PHASE_2_OPERATOR_LANES {
        let Some(operator) = operators
            .iter()
            .find(|operator| operator.operator_name() == *required_operator)
        else {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::MissingRequiredOperator);
        };
        if operator.classification() != *classification
            || operator.required_query_surface() != *query_surface
        {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::RequiredOperatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn require_phase_2_validator_lanes(
    validators: &[PlanarBooleanOverlapValidatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    for (required_validator, runtime_lane, governs_topology_legality) in
        REQUIRED_PHASE_2_VALIDATOR_LANES
    {
        let Some(validator) = validators
            .iter()
            .find(|validator| validator.validator_name() == *required_validator)
        else {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::MissingRequiredValidator);
        };
        if validator.runtime_lane() != *runtime_lane
            || validator.governs_topology_legality() != *governs_topology_legality
        {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::RequiredValidatorLaneMismatch);
        }
    }
    Ok(())
}
