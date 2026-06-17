use super::classification::{
    EdgeSplitOperatorClassification, EdgeSplitOperatorTruthAuthority, EdgeSplitRequiredQuerySurface,
};
use super::closeout::EdgeSplitBlueprintCloseoutDenial;
use super::operator_row::EdgeSplitOperatorRow;
use super::validator_row::EdgeSplitValidatorRow;

pub(super) fn require_operator_lane_is_honest(
    operator: &EdgeSplitOperatorRow,
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    match operator.classification() {
        EdgeSplitOperatorClassification::PreparedSpatialOnly
            if operator.truth_authority() != EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared
                || operator.required_query_surface() != EdgeSplitRequiredQuerySurface::None =>
        {
            Err(EdgeSplitBlueprintCloseoutDenial::PreparedSpatialOperatorClaimsTopologyAuthority)
        }
        EdgeSplitOperatorClassification::TopologyDeclarationFamily
            | EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily
            | EdgeSplitOperatorClassification::TopologyContributionWorkflow
            if !operator.classification().requires_query_surface()
                || operator.required_query_surface() == EdgeSplitRequiredQuerySurface::None =>
        {
            Err(EdgeSplitBlueprintCloseoutDenial::AuthoritativeTopologyOperatorMissingQuerySurface)
        }
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram
            if operator.required_query_surface() != EdgeSplitRequiredQuerySurface::QueryGraphComposition
                && operator.required_query_surface()
                    != EdgeSplitRequiredQuerySurface::QueryInvariantRegistration =>
        {
            Err(EdgeSplitBlueprintCloseoutDenial::GraphCompositionOperatorMissingGraphSurface)
        }
        EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
            if operator.truth_authority() != EdgeSplitOperatorTruthAuthority::FutureSupportGated =>
        {
            Err(EdgeSplitBlueprintCloseoutDenial::SupportGatedOperatorClaimsAdmittedTopologyMutation)
        }
        _ => Ok(()),
    }
}

pub(super) fn require_validator_lane_is_honest(
    validator: &EdgeSplitValidatorRow,
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    if validator.governs_topology_legality() && !validator.runtime_lane().is_runtime_facing() {
        return Err(EdgeSplitBlueprintCloseoutDenial::TopologyLegalityValidatorMissingRuntimeLane);
    }
    Ok(())
}
