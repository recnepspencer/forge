use super::declaration_admission::{
    admit_spatial_replay_family_declaration, SpatialReplayFamilyDeclarationInput,
};
use super::family_declaration::{
    SpatialReplayFamilyCatalog, SpatialReplayFamilyCoveredLookupIdentity,
    SpatialReplayFamilyLocalityPosture, SpatialReplayFamilyPriorProofPosture,
    SpatialReplayFamilyScopeProductPosture, SpatialReplayFamilyStageIndexPosture,
    SpatialReplayFamilyWorkloadDependencyPosture,
};
use super::family_identity::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentityAuthority,
};

pub fn current_spatial_replay_family_catalog() -> SpatialReplayFamilyCatalog {
    SpatialReplayFamilyCatalog::new(vec![
        admit_spatial_replay_family_declaration(SpatialReplayFamilyDeclarationInput {
            identity: admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
            ),
            locality_posture: SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority,
            prior_proof_posture:
                SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
            stage_index_posture: SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
            covered_lookup_identity:
                SpatialReplayFamilyCoveredLookupIdentity::BooleanEventLedgerEvidence,
            workload_dependency_posture:
                SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay,
            scope_product_posture: SpatialReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct,
        }),
        admit_spatial_replay_family_declaration(SpatialReplayFamilyDeclarationInput {
            identity: admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::projection_receipt(),
            ),
            locality_posture: SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority,
            prior_proof_posture:
                SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
            stage_index_posture: SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
            covered_lookup_identity:
                SpatialReplayFamilyCoveredLookupIdentity::ProjectionConsumptionEvidence,
            workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly,
            scope_product_posture: SpatialReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct,
        }),
    ])
}
