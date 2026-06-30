use worth_kernel::replay_undo_family_catalog::{
    ReplayFamilyDeclaration, ReplayFamilyDomain, ReplayFamilyIdentity, ReplayFamilyLocalityPosture,
    ReplayFamilyPriorProofPosture, ReplayFamilyScopeProductPosture,
    ReplayFamilyStageIndexPosture, ReplayFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = ReplayFamilyDeclaration {
        identity: ReplayFamilyIdentity::TopologyTraversalViewsReplay,
        domain: ReplayFamilyDomain::Topology,
        locality_posture: ReplayFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture:
            ReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt,
        stage_index_posture: ReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: ReplayFamilyWorkloadDependencyPosture::TopologyOnly,
        scope_product_posture: ReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct,
    };
}
