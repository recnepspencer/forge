use super::super::classification::{
    PlanarBooleanOverlapOperatorClassification as Class,
    PlanarBooleanOverlapOperatorTruthAuthority as Authority,
};
use super::super::operator_row::PlanarBooleanOverlapOperatorRow;
use super::super::required_phase_2_operator_lanes::REQUIRED_PHASE_2_OPERATOR_LANES;

pub(crate) fn phase_2_operators() -> Vec<PlanarBooleanOverlapOperatorRow> {
    REQUIRED_PHASE_2_OPERATOR_LANES
        .iter()
        .map(|(name, class, surface)| {
            let authority = match class {
                Class::PreparedSpatialOnly => Authority::WorthSpatialPrepared,
                Class::TopologyDeclarationFamily
                | Class::TopologyGroupedDeclarationFamily
                | Class::TopologyContributionWorkflow => Authority::WorthTopoQueryDeclaration,
                Class::QueryGraphCompositionProgram => Authority::ForgeQueryGraphComposition,
            };
            PlanarBooleanOverlapOperatorRow::new(name, *class, authority, *surface)
        })
        .collect()
}
