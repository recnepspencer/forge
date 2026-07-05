use std::collections::BTreeSet;

use super::classification::PlanarBooleanOverlapOperatorClassification;
use super::lane_honesty::{require_operator_lane_is_honest, require_validator_lane_is_honest};
use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::required_phase_2_rows::{
    require_phase_2_operator_lanes, require_phase_2_operator_rows, require_phase_2_validator_lanes,
    require_phase_2_validator_rows,
};
use super::validator_row::PlanarBooleanOverlapValidatorRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapBlueprintCloseoutDenial {
    DuplicateOperatorName,
    DuplicateValidatorName,
    MissingRequiredOperator,
    MissingRequiredValidator,
    UnexpectedOperatorName,
    UnexpectedValidatorName,
    RequiredOperatorLaneMismatch,
    RequiredValidatorLaneMismatch,
    PreparedSpatialOperatorClaimsTopologyAuthority,
    AuthoritativeTopologyOperatorMissingQuerySurface,
    GraphCompositionOperatorMissingGraphSurface,
    TopologyLegalityValidatorMissingRuntimeLane,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapBlueprintCloseout {
    prepared_spatial_operators: usize,
    topology_contribution_workflows: usize,
    query_graph_composition_programs: usize,
    validator_count: usize,
    runtime_facing_validator_count: usize,
}

impl PlanarBooleanOverlapBlueprintCloseout {
    pub fn certify(
        operators: &[PlanarBooleanOverlapOperatorRow],
        validators: &[PlanarBooleanOverlapValidatorRow],
    ) -> Result<Self, PlanarBooleanOverlapBlueprintCloseoutDenial> {
        require_unique_operators(operators)?;
        require_unique_validators(validators)?;
        for operator in operators {
            require_operator_lane_is_honest(operator)?;
        }
        for validator in validators {
            require_validator_lane_is_honest(validator)?;
        }
        require_phase_2_operator_rows(operators)?;
        require_phase_2_validator_rows(validators)?;
        require_phase_2_operator_lanes(operators)?;
        require_phase_2_validator_lanes(validators)?;

        Ok(Self {
            prepared_spatial_operators: count_operators(
                operators,
                PlanarBooleanOverlapOperatorClassification::PreparedSpatialOnly,
            ),
            topology_contribution_workflows: count_operators(
                operators,
                PlanarBooleanOverlapOperatorClassification::TopologyContributionWorkflow,
            ),
            query_graph_composition_programs: count_operators(
                operators,
                PlanarBooleanOverlapOperatorClassification::QueryGraphCompositionProgram,
            ),
            validator_count: validators.len(),
            runtime_facing_validator_count: validators
                .iter()
                .filter(|validator| validator.runtime_lane().is_runtime_facing())
                .count(),
        })
    }

    pub fn prepared_spatial_operators(&self) -> usize {
        self.prepared_spatial_operators
    }

    pub fn query_graph_composition_programs(&self) -> usize {
        self.query_graph_composition_programs
    }

    pub fn runtime_facing_validator_count(&self) -> usize {
        self.runtime_facing_validator_count
    }
}

fn require_unique_operators(
    operators: &[PlanarBooleanOverlapOperatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for operator in operators {
        if !names.insert(operator.operator_name()) {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::DuplicateOperatorName);
        }
    }
    Ok(())
}

fn require_unique_validators(
    validators: &[PlanarBooleanOverlapValidatorRow],
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for validator in validators {
        if !names.insert(validator.validator_name()) {
            return Err(PlanarBooleanOverlapBlueprintCloseoutDenial::DuplicateValidatorName);
        }
    }
    Ok(())
}

fn count_operators(
    operators: &[PlanarBooleanOverlapOperatorRow],
    classification: PlanarBooleanOverlapOperatorClassification,
) -> usize {
    operators
        .iter()
        .filter(|operator| operator.classification() == classification)
        .count()
}
