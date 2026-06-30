use topology::facade::{
    TopologyReplayFamilyDeclaration, TopologyReplayFamilyIdentity, TopologyReplayFamilyLocalityPosture,
    TopologyReplayFamilyPriorProofPosture, TopologyReplayFamilyScopeProductPosture,
    TopologyReplayFamilyStageIndexPosture, TopologyReplayFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = TopologyReplayFamilyDeclaration {
        identity: TopologyReplayFamilyIdentity::TraversalViewsReplay,
        locality_posture: TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure,
        prior_proof_posture:
            TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt,
        stage_index_posture: TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly,
        scope_product_posture: TopologyReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct,
    };
}
