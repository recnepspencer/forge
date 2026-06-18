use super::classification::{
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorTruthAuthority,
    PlanarBooleanLoopRequiredQuerySurface,
};
use super::closeout::PlanarBooleanLoopBlueprintCloseoutDenial;
use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::validator_row::PlanarBooleanLoopValidatorRow;

pub(super) fn require_operator_lane_is_honest(
    operator: &PlanarBooleanLoopOperatorRow,
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    match operator.classification() {
        PlanarBooleanLoopOperatorClassification::PreparedSpatialOnly
            if operator.truth_authority()
                != PlanarBooleanLoopOperatorTruthAuthority::WorthSpatialPrepared
                || operator.required_query_surface() != PlanarBooleanLoopRequiredQuerySurface::None =>
        {
            Err(PlanarBooleanLoopBlueprintCloseoutDenial::PreparedSpatialOperatorClaimsTopologyAuthority)
        }
        PlanarBooleanLoopOperatorClassification::TopologyDeclarationFamily
        | PlanarBooleanLoopOperatorClassification::TopologyGroupedDeclarationFamily
        | PlanarBooleanLoopOperatorClassification::TopologyContributionWorkflow
            if !operator.classification().requires_query_surface()
                || operator.required_query_surface() == PlanarBooleanLoopRequiredQuerySurface::None =>
        {
            Err(PlanarBooleanLoopBlueprintCloseoutDenial::AuthoritativeTopologyOperatorMissingQuerySurface)
        }
        PlanarBooleanLoopOperatorClassification::QueryGraphCompositionProgram
            if operator.required_query_surface()
                != PlanarBooleanLoopRequiredQuerySurface::QueryGraphComposition
                && operator.required_query_surface()
                    != PlanarBooleanLoopRequiredQuerySurface::QueryInvariantRegistration =>
        {
            Err(PlanarBooleanLoopBlueprintCloseoutDenial::GraphCompositionOperatorMissingGraphSurface)
        }
        PlanarBooleanLoopOperatorClassification::SupportGatedFutureTopologyMutation
            if operator.truth_authority()
                != PlanarBooleanLoopOperatorTruthAuthority::FutureSupportGated =>
        {
            Err(PlanarBooleanLoopBlueprintCloseoutDenial::SupportGatedOperatorClaimsAdmittedTopologyMutation)
        }
        _ => Ok(()),
    }
}

pub(super) fn require_validator_lane_is_honest(
    validator: &PlanarBooleanLoopValidatorRow,
) -> Result<(), PlanarBooleanLoopBlueprintCloseoutDenial> {
    if validator.governs_topology_legality() && !validator.runtime_lane().is_runtime_facing() {
        return Err(
            PlanarBooleanLoopBlueprintCloseoutDenial::TopologyLegalityValidatorMissingRuntimeLane,
        );
    }
    Ok(())
}
