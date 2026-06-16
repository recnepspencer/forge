use super::admitted_scope::PlanarBooleanCommonPlaneAdmittedOperandScope;
use super::denial::PlanarBooleanCommonPlaneScopeAdmissionError;
use crate::workload_composition::{
    PlanarBooleanCommonPlaneReductionRequest, WorkloadCatalogRecipeKind,
};

pub(super) fn admit_operand_scope(
    request: &PlanarBooleanCommonPlaneReductionRequest,
) -> Result<PlanarBooleanCommonPlaneAdmittedOperandScope, PlanarBooleanCommonPlaneScopeAdmissionError>
{
    match request.operand_pair_recipe().recipe() {
        WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair
        | WorkloadCatalogRecipeKind::BooleanEventCarrierCleanPlanarBodyPair
        | WorkloadCatalogRecipeKind::BooleanEventExtractionMetabossPair
        | WorkloadCatalogRecipeKind::BooleanMismatchedPosturePair => {
            Ok(PlanarBooleanCommonPlaneAdmittedOperandScope::ClosedPlanarBodyPair)
        }
        actual_recipe => Err(
            PlanarBooleanCommonPlaneScopeAdmissionError::UnsupportedOperandPairRecipe {
                actual_recipe,
                admitted_scope: PlanarBooleanCommonPlaneAdmittedOperandScope::ClosedPlanarBodyPair
                    .human_name(),
                request_identity: request.request_identity().to_string(),
                operand_pair_identity: request.operand_pair_identity().to_string(),
            },
        ),
    }
}
