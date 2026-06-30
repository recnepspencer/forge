use super::family_declaration::{
    TopologyReplayFamilyDeclaration, TopologyReplayFamilyLocalityPosture,
    TopologyReplayFamilyPriorProofPosture, TopologyReplayFamilyScopeProductPosture,
    TopologyReplayFamilyStageIndexPosture, TopologyReplayFamilyWorkloadDependencyPosture,
};
use super::family_identity::TopologyReplayFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyReplayFamilyDeclarationInput {
    pub identity: TopologyReplayFamilyIdentity,
    pub locality_posture: TopologyReplayFamilyLocalityPosture,
    pub prior_proof_posture: TopologyReplayFamilyPriorProofPosture,
    pub stage_index_posture: TopologyReplayFamilyStageIndexPosture,
    pub workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture,
    pub scope_product_posture: TopologyReplayFamilyScopeProductPosture,
}

pub fn admit_topology_replay_family_declaration(
    input: TopologyReplayFamilyDeclarationInput,
) -> TopologyReplayFamilyDeclaration {
    TopologyReplayFamilyDeclaration::new(
        input.identity,
        input.locality_posture,
        input.prior_proof_posture,
        input.stage_index_posture,
        input.workload_dependency_posture,
        input.scope_product_posture,
    )
}
