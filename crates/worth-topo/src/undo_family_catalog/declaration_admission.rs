use super::family_declaration::{
    TopologyUndoFamilyDeclaration, TopologyUndoFamilyLocalityPosture,
    TopologyUndoFamilyPriorProofPosture, TopologyUndoFamilyScopeProductPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};
use super::family_identity::TopologyUndoFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyUndoFamilyDeclarationInput {
    pub identity: TopologyUndoFamilyIdentity,
    pub locality_posture: TopologyUndoFamilyLocalityPosture,
    pub prior_proof_posture: TopologyUndoFamilyPriorProofPosture,
    pub stage_index_posture: TopologyUndoFamilyStageIndexPosture,
    pub workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture,
    pub scope_product_posture: TopologyUndoFamilyScopeProductPosture,
}

pub fn admit_topology_undo_family_declaration(
    input: TopologyUndoFamilyDeclarationInput,
) -> TopologyUndoFamilyDeclaration {
    TopologyUndoFamilyDeclaration::new(
        input.identity,
        input.locality_posture,
        input.prior_proof_posture,
        input.stage_index_posture,
        input.workload_dependency_posture,
        input.scope_product_posture,
    )
}
