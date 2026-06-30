use super::declaration_admission::{
    admit_topology_replay_family_declaration, TopologyReplayFamilyDeclarationInput,
};
use super::family_declaration::{
    TopologyReplayFamilyCatalog, TopologyReplayFamilyLocalityPosture,
    TopologyReplayFamilyPriorProofPosture, TopologyReplayFamilyScopeProductPosture,
    TopologyReplayFamilyStageIndexPosture, TopologyReplayFamilyWorkloadDependencyPosture,
};
use super::family_identity::{
    admit_topology_replay_family_identity, TopologyReplayFamilyIdentityAuthority,
};

pub fn current_topology_replay_family_catalog() -> TopologyReplayFamilyCatalog {
    TopologyReplayFamilyCatalog::new(vec![
        admit_topology_replay_family_declaration(TopologyReplayFamilyDeclarationInput {
            identity: admit_topology_replay_family_identity(
                TopologyReplayFamilyIdentityAuthority::traversal_views(),
            ),
            locality_posture: TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure,
            prior_proof_posture:
                TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt,
            stage_index_posture: TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly,
            scope_product_posture: TopologyReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct,
        }),
        admit_topology_replay_family_declaration(TopologyReplayFamilyDeclarationInput {
            identity: admit_topology_replay_family_identity(
                TopologyReplayFamilyIdentityAuthority::materialized_graph(),
            ),
            locality_posture: TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure,
            prior_proof_posture:
                TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt,
            stage_index_posture: TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly,
            scope_product_posture: TopologyReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct,
        }),
    ])
}
