use super::family_declaration::{
    SpatialUndoFamilyDeclaration, SpatialUndoFamilyLocalityPosture,
    SpatialUndoFamilyPriorProofPosture, SpatialUndoFamilyScopeProductPosture,
    SpatialUndoFamilyStageIndexPosture, SpatialUndoFamilyWorkloadDependencyPosture,
};
use super::family_identity::SpatialUndoFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialUndoFamilyDeclarationInput {
    pub identity: SpatialUndoFamilyIdentity,
    pub locality_posture: SpatialUndoFamilyLocalityPosture,
    pub prior_proof_posture: SpatialUndoFamilyPriorProofPosture,
    pub stage_index_posture: SpatialUndoFamilyStageIndexPosture,
    pub workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
    pub scope_product_posture: SpatialUndoFamilyScopeProductPosture,
}

pub fn admit_spatial_undo_family_declaration(
    input: SpatialUndoFamilyDeclarationInput,
) -> SpatialUndoFamilyDeclaration {
    SpatialUndoFamilyDeclaration::new(
        input.identity,
        input.locality_posture,
        input.prior_proof_posture,
        input.stage_index_posture,
        input.workload_dependency_posture,
        input.scope_product_posture,
    )
}
