use std::collections::BTreeSet;

use super::classification::PlanarBooleanLoopOperatorClassification;
use super::lane_honesty::{require_operator_lane_is_honest, require_validator_lane_is_honest};
use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::required_phase_2_rows::{
    require_phase_2_operator_lanes, require_phase_2_operator_rows, require_phase_2_validator_lanes,
    require_phase_2_validator_rows, required_phase_2_operator_row_count,
    required_phase_2_validator_row_count,
};
use super::validator_row::PlanarBooleanLoopValidatorRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopBlueprintCloseoutDenial {
    DuplicateOperatorName,
    DuplicateValidatorName,
    MissingRequiredOperator,
    MissingRequiredValidator,
    RequiredOperatorLaneMismatch,
    RequiredValidatorLaneMismatch,
    PreparedSpatialOperatorClaimsTopologyAuthority,
    AuthoritativeTopologyOperatorMissingQuerySurface,
    GraphCompositionOperatorMissingGraphSurface,
    SupportGatedOperatorClaimsAdmittedTopologyMutation,
    TopologyLegalityValidatorMissingRuntimeLane,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopBlueprintCloseout {
    prepared_spatial_operators: usize,
    topology_declaration_operators: usize,
    topology_grouped_declaration_operators: usize,
    topology_contribution_workflows: usize,
    query_graph_composition_programs: usize,
    support_gated_future_operators: usize,
    required_phase_2_operator_rows: usize,
    required_phase_2_validator_rows: usize,
    validator_count: usize,
    runtime_facing_validator_count: usize,
}

impl PlanarBooleanLoopBlueprintCloseout {
    pub fn certify(
        operators: &[PlanarBooleanLoopOperatorRow],
        validators: &[PlanarBooleanLoopValidatorRow],
    ) -> Result<Self, PlanarBooleanLoopBlueprintCloseoutDenial> {
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
                PlanarBooleanLoopOperatorClassification::PreparedSpatialOnly,
            ),
            topology_declaration_operators: count_operators(
                operators,
                PlanarBooleanLoopOperatorClassification::TopologyDeclarationFamily,
            ),
            topology_grouped_declaration_operators: count_operators(
                operators,
                PlanarBooleanLoopOperatorClassification::TopologyGroupedDeclarationFamily,
            ),
            topology_contribution_workflows: count_operators(
                operators,
                PlanarBooleanLoopOperatorClassification::TopologyContributionWorkflow,
            ),
            query_graph_composition_programs: count_operators(
                operators,
                PlanarBooleanLoopOperatorClassification::QueryGraphCompositionProgram,
            ),
            support_gated_future_operators: count_operators(
                operators,
                PlanarBooleanLoopOperatorClassification::SupportGatedFutureTopologyMutation,
            ),
            required_phase_2_operator_rows: required_phase_2_operator_row_count(),
            required_phase_2_validator_rows: required_phase_2_validator_row_count(),
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

    pub fn topology_declaration_operators(&self) -> usize {
        self.topology_declaration_operators
    }

    pub fn topology_grouped_declaration_operators(&self) -> usize {
        self.topology_grouped_declaration_operators
    }

    pub fn topology_contribution_workflows(&self) -> usize {
        self.topology_contribution_workflows
    }

    pub fn query_graph_composition_programs(&self) -> usize {
        self.query_graph_composition_programs
    }

    pub fn support_gated_future_operators(&self) -> usize {
        self.support_gated_future_operators
    }

    pub fn required_phase_2_operator_rows(&self) -> usize {
        self.required_phase_2_operator_rows
    }

    pub fn required_phase_2_validator_rows(&self) -> usize {
        self.required_phase_2_validator_rows
    }

    pub fn validator_count(&self) -> usize {
        self.validator_count
    }

    pub fn runtime_facing_validator_count(&self) -> usize {
        self.runtime_facing_validator_count
    }

    pub fn certified_authoritative_topology_mutations_have_query_entries(&self) -> bool {
        self.topology_declaration_operators
            + self.topology_grouped_declaration_operators
            + self.topology_contribution_workflows
            + self.query_graph_composition_programs
            > 0
    }

    pub fn certified_prepared_spatial_products_do_not_claim_topology_authority(&self) -> bool {
        self.prepared_spatial_operators > 0
    }

    pub fn certified_validators_use_runtime_visible_lanes(&self) -> bool {
        self.runtime_facing_validator_count > 0
    }

    pub fn certified_phase_2_required_rows_present(&self) -> bool {
        self.required_phase_2_operator_rows == required_phase_2_operator_row_count()
            && self.required_phase_2_validator_rows == required_phase_2_validator_row_count()
    }
}

fn require_unique_operators(
    operators: &[PlanarBooleanLoopOperatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for operator in operators {
        if !names.insert(operator.operator_name()) {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::DuplicateOperatorName);
        }
    }
    Ok(())
}

fn require_unique_validators(
    validators: &[PlanarBooleanLoopValidatorRow],
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for validator in validators {
        if !names.insert(validator.validator_name()) {
            return Err(PlanarBooleanLoopBlueprintCloseoutDenial::DuplicateValidatorName);
        }
    }
    Ok(())
}

fn count_operators(
    operators: &[PlanarBooleanLoopOperatorRow],
    classification: PlanarBooleanLoopOperatorClassification,
) -> usize {
    operators
        .iter()
        .filter(|operator| operator.classification() == classification)
        .count()
}
