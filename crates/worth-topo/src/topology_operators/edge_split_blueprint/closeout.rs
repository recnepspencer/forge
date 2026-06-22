use std::collections::BTreeSet;

use super::classification::EdgeSplitOperatorClassification;
use super::lane_honesty::{require_operator_lane_is_honest, require_validator_lane_is_honest};
use super::operator_row::EdgeSplitOperatorRow;
use super::required_phase_1_rows::{
    require_phase_1_operator_lanes, require_phase_1_operator_rows, require_phase_1_validator_lanes,
    require_phase_1_validator_rows, required_phase_1_operator_row_count,
    required_phase_1_validator_row_count,
};
use super::validator_row::EdgeSplitValidatorRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeSplitBlueprintCloseoutDenial {
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

impl EdgeSplitBlueprintCloseoutDenial {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::DuplicateOperatorName => "edge split blueprint has duplicate operator names",
            Self::DuplicateValidatorName => "edge split blueprint has duplicate validator names",
            Self::MissingRequiredOperator => {
                "edge split blueprint is missing a required phase 1 operator row"
            }
            Self::MissingRequiredValidator => {
                "edge split blueprint is missing a required phase 1 validator row"
            }
            Self::RequiredOperatorLaneMismatch => {
                "edge split blueprint required operator row uses the wrong authority lane"
            }
            Self::RequiredValidatorLaneMismatch => {
                "edge split blueprint required validator row uses the wrong runtime lane"
            }
            Self::PreparedSpatialOperatorClaimsTopologyAuthority => {
                "prepared spatial split operators must not claim topology mutation authority"
            }
            Self::AuthoritativeTopologyOperatorMissingQuerySurface => {
                "authoritative topology split operators must declare a Query topology surface"
            }
            Self::GraphCompositionOperatorMissingGraphSurface => {
                "edge split graph composition operators must require the Query graph composition surface"
            }
            Self::SupportGatedOperatorClaimsAdmittedTopologyMutation => {
                "support-gated future split operators must not claim admitted topology mutation in 7.3"
            }
            Self::TopologyLegalityValidatorMissingRuntimeLane => {
                "topology legality validators must register through Query invariants or topology declaration review"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeSplitBlueprintCloseout {
    prepared_spatial_operators: usize,
    topology_declaration_operators: usize,
    topology_grouped_declaration_operators: usize,
    topology_contribution_workflows: usize,
    query_graph_composition_programs: usize,
    support_gated_future_operators: usize,
    required_phase_1_operator_rows: usize,
    required_phase_1_validator_rows: usize,
    validator_count: usize,
    runtime_facing_validator_count: usize,
}

impl EdgeSplitBlueprintCloseout {
    pub(crate) fn certify(
        operators: &[EdgeSplitOperatorRow],
        validators: &[EdgeSplitValidatorRow],
    ) -> Result<Self, EdgeSplitBlueprintCloseoutDenial> {
        require_unique_operators(operators)?;
        require_unique_validators(validators)?;
        for operator in operators {
            require_operator_lane_is_honest(operator)?;
        }
        for validator in validators {
            require_validator_lane_is_honest(validator)?;
        }
        require_phase_1_operator_rows(operators)?;
        require_phase_1_validator_rows(validators)?;
        require_phase_1_operator_lanes(operators)?;
        require_phase_1_validator_lanes(validators)?;
        Ok(Self {
            prepared_spatial_operators: count_operators(
                operators,
                EdgeSplitOperatorClassification::PreparedSpatialOnly,
            ),
            topology_declaration_operators: count_operators(
                operators,
                EdgeSplitOperatorClassification::TopologyDeclarationFamily,
            ),
            topology_grouped_declaration_operators: count_operators(
                operators,
                EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily,
            ),
            topology_contribution_workflows: count_operators(
                operators,
                EdgeSplitOperatorClassification::TopologyContributionWorkflow,
            ),
            query_graph_composition_programs: count_operators(
                operators,
                EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
            ),
            support_gated_future_operators: count_operators(
                operators,
                EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation,
            ),
            required_phase_1_operator_rows: required_phase_1_operator_row_count(),
            required_phase_1_validator_rows: required_phase_1_validator_row_count(),
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

    pub fn required_phase_1_operator_rows(&self) -> usize {
        self.required_phase_1_operator_rows
    }

    pub fn required_phase_1_validator_rows(&self) -> usize {
        self.required_phase_1_validator_rows
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

    pub fn certified_phase_1_required_rows_present(&self) -> bool {
        self.required_phase_1_operator_rows == required_phase_1_operator_row_count()
            && self.required_phase_1_validator_rows == required_phase_1_validator_row_count()
    }
}

fn require_unique_operators(
    operators: &[EdgeSplitOperatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for operator in operators {
        if !names.insert(operator.operator_name()) {
            return Err(EdgeSplitBlueprintCloseoutDenial::DuplicateOperatorName);
        }
    }
    Ok(())
}

fn require_unique_validators(
    validators: &[EdgeSplitValidatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    let mut names = BTreeSet::new();
    for validator in validators {
        if !names.insert(validator.validator_name()) {
            return Err(EdgeSplitBlueprintCloseoutDenial::DuplicateValidatorName);
        }
    }
    Ok(())
}

fn count_operators(
    operators: &[EdgeSplitOperatorRow],
    classification: EdgeSplitOperatorClassification,
) -> usize {
    operators
        .iter()
        .filter(|operator| operator.classification() == classification)
        .count()
}
