use super::closeout::PlanarBooleanLoopBlueprintCloseoutDenial;
use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::required_phase_2_operator_lanes::REQUIRED_PHASE_2_OPERATOR_LANES;
use super::required_phase_2_validator_lanes::REQUIRED_PHASE_2_VALIDATOR_LANES;
use super::validator_row::PlanarBooleanLoopValidatorRow;

pub(super) fn require_phase_2_operator_rows(
    operators: &[PlanarBooleanLoopOperatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    for (required_operator, _, _) in REQUIRED_PHASE_2_OPERATOR_LANES {
        if !operators
            .iter()
            .any(|operator| operator.operator_name() == *required_operator)
        {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::MissingRequiredOperator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_2_validator_rows(
    validators: &[PlanarBooleanLoopValidatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    for (required_validator, _, _) in REQUIRED_PHASE_2_VALIDATOR_LANES {
        if !validators
            .iter()
            .any(|validator| validator.validator_name() == *required_validator)
        {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::MissingRequiredValidator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_2_operator_lanes(
    operators: &[PlanarBooleanLoopOperatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    for (required_operator, classification, query_surface) in REQUIRED_PHASE_2_OPERATOR_LANES {
        let Some(operator) = operators
            .iter()
            .find(|operator| operator.operator_name() == *required_operator)
        else {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::MissingRequiredOperator);
        };
        if operator.classification() != *classification
            || operator.required_query_surface() != *query_surface
        {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::RequiredOperatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn require_phase_2_validator_lanes(
    validators: &[PlanarBooleanLoopValidatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    for (required_validator, runtime_lane, governs_topology_legality) in
        REQUIRED_PHASE_2_VALIDATOR_LANES
    {
        let Some(validator) = validators
            .iter()
            .find(|validator| validator.validator_name() == *required_validator)
        else {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::MissingRequiredValidator);
        };
        if validator.runtime_lane() != *runtime_lane
            || validator.governs_topology_legality() != *governs_topology_legality
        {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::RequiredValidatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn required_phase_2_operator_row_count() -> usize {
    REQUIRED_PHASE_2_OPERATOR_LANES.len()
}

pub(super) fn required_phase_2_validator_row_count() -> usize {
    REQUIRED_PHASE_2_VALIDATOR_LANES.len()
}
