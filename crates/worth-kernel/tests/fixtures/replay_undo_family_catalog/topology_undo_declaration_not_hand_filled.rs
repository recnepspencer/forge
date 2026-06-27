use topology::facade::{
    TopologyUndoFamilyDeclaration, TopologyUndoFamilyIdentity, TopologyUndoFamilyLocalityPosture,
    TopologyUndoFamilyPriorProofPosture, TopologyUndoFamilyScopeProductPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = TopologyUndoFamilyDeclaration {
        identity: TopologyUndoFamilyIdentity::TraversalViewsRollback,
        locality_posture: TopologyUndoFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture: TopologyUndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt,
        stage_index_posture: TopologyUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture::TopologyOnly,
        scope_product_posture: TopologyUndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct,
    };
}
