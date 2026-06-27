use worth_spatial::facade::undo_family_catalog::{
    SpatialUndoFamilyDeclaration, SpatialUndoFamilyIdentity, SpatialUndoFamilyLocalityPosture,
    SpatialUndoFamilyPriorProofPosture, SpatialUndoFamilyScopeProductPosture,
    SpatialUndoFamilyStageIndexPosture, SpatialUndoFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = SpatialUndoFamilyDeclaration {
        identity: SpatialUndoFamilyIdentity::BooleanEventLedgerRollback,
        locality_posture: SpatialUndoFamilyLocalityPosture::RequiresSpatialTouchAuthority,
        prior_proof_posture: SpatialUndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
        stage_index_posture: SpatialUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture:
            SpatialUndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload,
        scope_product_posture: SpatialUndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct,
    };
}
