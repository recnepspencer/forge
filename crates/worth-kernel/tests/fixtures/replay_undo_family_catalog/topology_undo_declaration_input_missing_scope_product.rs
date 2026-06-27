use topology::facade::{
    admit_topology_undo_family_declaration, admit_topology_undo_family_identity,
    TopologyUndoFamilyDeclarationInput, TopologyUndoFamilyIdentityAuthority,
    TopologyUndoFamilyLocalityPosture, TopologyUndoFamilyPriorProofPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = admit_topology_undo_family_declaration(TopologyUndoFamilyDeclarationInput {
        identity: admit_topology_undo_family_identity(
            TopologyUndoFamilyIdentityAuthority::traversal_views(),
        ),
        locality_posture: TopologyUndoFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture: TopologyUndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt,
        stage_index_posture: TopologyUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture::TopologyOnly,
    });
}
