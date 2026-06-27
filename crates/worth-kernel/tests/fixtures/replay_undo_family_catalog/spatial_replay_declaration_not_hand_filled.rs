use worth_spatial::facade::replay_family_catalog::{
    SpatialReplayFamilyDeclaration, SpatialReplayFamilyIdentity, SpatialReplayFamilyLocalityPosture,
    SpatialReplayFamilyPriorProofPosture, SpatialReplayFamilyScopeProductPosture,
    SpatialReplayFamilyStageIndexPosture, SpatialReplayFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = SpatialReplayFamilyDeclaration {
        identity: SpatialReplayFamilyIdentity::BooleanEventLedgerReplay,
        locality_posture: SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority,
        prior_proof_posture: SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
        stage_index_posture: SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture:
            SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay,
        scope_product_posture: SpatialReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct,
    };
}
