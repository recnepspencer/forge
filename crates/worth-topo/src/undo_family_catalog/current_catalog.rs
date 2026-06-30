use super::declaration_admission::{
    admit_topology_undo_family_declaration, TopologyUndoFamilyDeclarationInput,
};
use super::family_declaration::{
    TopologyUndoFamilyCatalog, TopologyUndoFamilyLocalityPosture,
    TopologyUndoFamilyPriorProofPosture, TopologyUndoFamilyScopeProductPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};
use super::family_identity::{
    admit_topology_undo_family_identity, TopologyUndoFamilyIdentityAuthority,
};

pub fn current_topology_undo_family_catalog() -> TopologyUndoFamilyCatalog {
    TopologyUndoFamilyCatalog::new(vec![
        admit_topology_undo_family_declaration(TopologyUndoFamilyDeclarationInput {
            identity: admit_topology_undo_family_identity(
                TopologyUndoFamilyIdentityAuthority::traversal_views(),
            ),
            locality_posture: TopologyUndoFamilyLocalityPosture::RequiresTouchedClosure,
            prior_proof_posture:
                TopologyUndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt,
            stage_index_posture: TopologyUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture::TopologyOnly,
            scope_product_posture:
                TopologyUndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct,
        }),
        admit_topology_undo_family_declaration(TopologyUndoFamilyDeclarationInput {
            identity: admit_topology_undo_family_identity(
                TopologyUndoFamilyIdentityAuthority::materialized_graph(),
            ),
            locality_posture: TopologyUndoFamilyLocalityPosture::RequiresTouchedClosure,
            prior_proof_posture:
                TopologyUndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt,
            stage_index_posture: TopologyUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture::TopologyOnly,
            scope_product_posture:
                TopologyUndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct,
        }),
    ])
}
