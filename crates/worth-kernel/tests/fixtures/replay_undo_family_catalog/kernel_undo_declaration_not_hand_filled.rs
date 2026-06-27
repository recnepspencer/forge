use worth_kernel::replay_undo_family_catalog::{
    UndoFamilyDeclaration, UndoFamilyDomain, UndoFamilyIdentity, UndoFamilyLocalityPosture,
    UndoFamilyPriorProofPosture, UndoFamilyScopeProductPosture, UndoFamilyStageIndexPosture,
    UndoFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = UndoFamilyDeclaration {
        identity: UndoFamilyIdentity::TopologyTraversalViewsRollback,
        domain: UndoFamilyDomain::Topology,
        locality_posture: UndoFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture: UndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt,
        stage_index_posture: UndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: UndoFamilyWorkloadDependencyPosture::TopologyOnly,
        scope_product_posture: UndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct,
    };
}
