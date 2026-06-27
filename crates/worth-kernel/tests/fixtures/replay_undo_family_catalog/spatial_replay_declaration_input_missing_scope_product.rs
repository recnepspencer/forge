use worth_spatial::facade::replay_family_catalog::{
    admit_spatial_replay_family_declaration, admit_spatial_replay_family_identity,
    SpatialReplayFamilyDeclarationInput, SpatialReplayFamilyIdentityAuthority,
    SpatialReplayFamilyLocalityPosture, SpatialReplayFamilyPriorProofPosture,
    SpatialReplayFamilyStageIndexPosture, SpatialReplayFamilyWorkloadDependencyPosture,
};

fn main() {
    let _ = admit_spatial_replay_family_declaration(SpatialReplayFamilyDeclarationInput {
        identity: admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        locality_posture: SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority,
        prior_proof_posture: SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt,
        stage_index_posture: SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity,
        workload_dependency_posture:
            SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay,
    });
}
