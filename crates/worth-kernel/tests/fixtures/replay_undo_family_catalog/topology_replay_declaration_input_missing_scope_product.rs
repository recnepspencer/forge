use topology::facade::{
    admit_topology_replay_family_declaration, admit_topology_replay_family_identity,
    TopologyReplayFamilyDeclarationInput, TopologyReplayFamilyIdentityAuthority,
    TopologyReplayFamilyLocalityPosture, TopologyReplayFamilyPriorProofPosture,
    TopologyReplayFamilyStageIndexPosture, TopologyReplayFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = admit_topology_replay_family_declaration(TopologyReplayFamilyDeclarationInput {
        identity: admit_topology_replay_family_identity(
            TopologyReplayFamilyIdentityAuthority::traversal_views(),
        ),
        locality_posture: TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture:
            TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt,
        stage_index_posture: TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly,
    });
}
