use super::declaration_admission::{
    admit_spatial_undo_family_declaration, SpatialUndoFamilyDeclarationInput,
};
use super::family_declaration::{
    SpatialUndoFamilyCatalog, SpatialUndoFamilyLocalityPosture, SpatialUndoFamilyPriorProofPosture,
    SpatialUndoFamilyScopeProductPosture, SpatialUndoFamilyStageIndexPosture,
    SpatialUndoFamilyWorkloadDependencyPosture,
};
use super::family_identity::{
    admit_spatial_undo_family_identity, SpatialUndoFamilyIdentityAuthority,
};

pub fn current_spatial_undo_family_catalog() -> SpatialUndoFamilyCatalog {
    SpatialUndoFamilyCatalog::new(vec![
        admit_spatial_undo_family_declaration(SpatialUndoFamilyDeclarationInput {
            identity: admit_spatial_undo_family_identity(
                SpatialUndoFamilyIdentityAuthority::boolean_event_ledger(),
            ),
            locality_posture: SpatialUndoFamilyLocalityPosture::RequiresSpatialTouchAuthority,
            prior_proof_posture:
                SpatialUndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
            stage_index_posture: SpatialUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture:
                SpatialUndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload,
            scope_product_posture:
                SpatialUndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct,
        }),
        admit_spatial_undo_family_declaration(SpatialUndoFamilyDeclarationInput {
            identity: admit_spatial_undo_family_identity(
                SpatialUndoFamilyIdentityAuthority::projection_receipt(),
            ),
            locality_posture: SpatialUndoFamilyLocalityPosture::RequiresSpatialTouchAuthority,
            prior_proof_posture:
                SpatialUndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
            stage_index_posture: SpatialUndoFamilyStageIndexPosture::RequiresStageIndexIdentity,
            workload_dependency_posture:
                SpatialUndoFamilyWorkloadDependencyPosture::LookupReceiptOnly,
            scope_product_posture:
                SpatialUndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct,
        }),
    ])
}
