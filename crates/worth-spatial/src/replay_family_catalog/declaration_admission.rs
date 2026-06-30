use super::family_declaration::{
    SpatialReplayFamilyCoveredLookupIdentity, SpatialReplayFamilyDeclaration,
    SpatialReplayFamilyLocalityPosture, SpatialReplayFamilyPriorProofPosture,
    SpatialReplayFamilyScopeProductPosture, SpatialReplayFamilyStageIndexPosture,
    SpatialReplayFamilyWorkloadDependencyPosture,
};
use super::family_identity::SpatialReplayFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialReplayFamilyDeclarationInput {
    pub identity: SpatialReplayFamilyIdentity,
    pub locality_posture: SpatialReplayFamilyLocalityPosture,
    pub prior_proof_posture: SpatialReplayFamilyPriorProofPosture,
    pub stage_index_posture: SpatialReplayFamilyStageIndexPosture,
    pub covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
    pub workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
    pub scope_product_posture: SpatialReplayFamilyScopeProductPosture,
}

pub fn admit_spatial_replay_family_declaration(
    input: SpatialReplayFamilyDeclarationInput,
) -> SpatialReplayFamilyDeclaration {
    SpatialReplayFamilyDeclaration::new(
        input.identity,
        input.locality_posture,
        input.prior_proof_posture,
        input.stage_index_posture,
        input.covered_lookup_identity,
        input.workload_dependency_posture,
        input.scope_product_posture,
    )
}
