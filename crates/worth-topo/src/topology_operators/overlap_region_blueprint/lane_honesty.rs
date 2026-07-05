use super::classification::{
    PlanarBooleanOverlapOperatorClassification, PlanarBooleanOverlapOperatorTruthAuthority,
    PlanarBooleanOverlapRequiredQuerySurface,
};
use super::closeout::PlanarBooleanOverlapBlueprintCloseoutDenial;
use super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::validator_row::PlanarBooleanOverlapValidatorRow;

pub(super) fn require_operator_lane_is_honest(
    operator: &PlanarBooleanOverlapOperatorRow,
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    match operator.classification() {
        PlanarBooleanOverlapOperatorClassification::PreparedSpatialOnly
            if operator.truth_authority()
                != PlanarBooleanOverlapOperatorTruthAuthority::WorthSpatialPrepared
                || operator.required_query_surface()
                    != PlanarBooleanOverlapRequiredQuerySurface::None =>
        {
            Err(
                PlanarBooleanOverlapBlueprintCloseoutDenial::PreparedSpatialOperatorClaimsTopologyAuthority,
            )
        }
        PlanarBooleanOverlapOperatorClassification::TopologyDeclarationFamily
        | PlanarBooleanOverlapOperatorClassification::TopologyGroupedDeclarationFamily
        | PlanarBooleanOverlapOperatorClassification::TopologyContributionWorkflow
            if !operator.classification().requires_query_surface()
                || operator.required_query_surface()
                    == PlanarBooleanOverlapRequiredQuerySurface::None =>
        {
            Err(
                PlanarBooleanOverlapBlueprintCloseoutDenial::AuthoritativeTopologyOperatorMissingQuerySurface,
            )
        }
        PlanarBooleanOverlapOperatorClassification::QueryGraphCompositionProgram
            if operator.required_query_surface()
                != PlanarBooleanOverlapRequiredQuerySurface::QueryGraphComposition
                && operator.required_query_surface()
                    != PlanarBooleanOverlapRequiredQuerySurface::QueryInvariantRegistration =>
        {
            Err(
                PlanarBooleanOverlapBlueprintCloseoutDenial::GraphCompositionOperatorMissingGraphSurface,
            )
        }
        _ => Ok(()),
    }
}

pub(super) fn require_validator_lane_is_honest(
    validator: &PlanarBooleanOverlapValidatorRow,
) -> Result<(), PlanarBooleanOverlapBlueprintCloseoutDenial> {
    if validator.governs_topology_legality() && !validator.runtime_lane().is_runtime_facing() {
        return Err(
            PlanarBooleanOverlapBlueprintCloseoutDenial::TopologyLegalityValidatorMissingRuntimeLane,
        );
    }
    Ok(())
}
