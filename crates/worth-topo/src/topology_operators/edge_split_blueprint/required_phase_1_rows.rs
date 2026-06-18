use super::closeout::EdgeSplitBlueprintCloseoutDenial;
use super::operator_row::EdgeSplitOperatorRow;
use super::required_phase_1_operator_lanes::REQUIRED_PHASE_1_OPERATOR_LANES;
use super::required_phase_1_validator_lanes::REQUIRED_PHASE_1_VALIDATOR_LANES;
use super::validator_row::EdgeSplitValidatorRow;

pub(super) fn require_phase_1_operator_rows(
    operators: &[EdgeSplitOperatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_operator, _, _) in REQUIRED_PHASE_1_OPERATOR_LANES {
        if !operators
            .iter()
            .any(|operator| operator.operator_name() == *required_operator)
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredOperator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_validator_rows(
    validators: &[EdgeSplitValidatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_validator, _, _) in REQUIRED_PHASE_1_VALIDATOR_LANES {
        if !validators
            .iter()
            .any(|validator| validator.validator_name() == *required_validator)
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredValidator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_operator_lanes(
    operators: &[EdgeSplitOperatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_operator, classification, query_surface) in REQUIRED_PHASE_1_OPERATOR_LANES {
        let Some(operator) = operators
            .iter()
            .find(|operator| operator.operator_name() == *required_operator)
        else {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredOperator);
        };
        if operator.classification() != *classification
            || operator.required_query_surface() != *query_surface
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::RequiredOperatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_validator_lanes(
    validators: &[EdgeSplitValidatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_validator, runtime_lane, governs_topology_legality) in
        REQUIRED_PHASE_1_VALIDATOR_LANES
    {
        let Some(validator) = validators
            .iter()
            .find(|validator| validator.validator_name() == *required_validator)
        else {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredValidator);
        };
        if validator.runtime_lane() != *runtime_lane
            || validator.governs_topology_legality() != *governs_topology_legality
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::RequiredValidatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn required_phase_1_operator_row_count() -> usize {
    REQUIRED_PHASE_1_OPERATOR_LANES.len()
}

pub(super) fn required_phase_1_validator_row_count() -> usize {
    REQUIRED_PHASE_1_VALIDATOR_LANES.len()
}
