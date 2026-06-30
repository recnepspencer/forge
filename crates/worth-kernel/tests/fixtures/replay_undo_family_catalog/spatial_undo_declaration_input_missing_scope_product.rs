use worth_spatial::facade::undo_family_catalog::{
    admit_spatial_undo_family_declaration, admit_spatial_undo_family_identity,
    SpatialUndoFamilyDeclarationInput, SpatialUndoFamilyIdentityAuthority,
    SpatialUndoFamilyLocalityPosture, SpatialUndoFamilyPriorProofPosture,
    SpatialUndoFamilyStageIndexPosture, SpatialUndoFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = admit_spatial_undo_family_declaration(SpatialUndoFamilyDeclarationInput {
        identity: admit_spatial_undo_family_identity(
            SpatialUndoFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        locality_posture: SpatialUndoFamilyLocalityPosture::RequiresSpatialTouchAuthority,
        prior_proof_posture: SpatialUndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
        stage_index_posture: SpatialUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture:
            SpatialUndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload,
    });
}
