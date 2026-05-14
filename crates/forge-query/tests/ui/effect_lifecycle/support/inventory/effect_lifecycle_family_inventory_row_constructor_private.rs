use forge_query::facade::{
    BasisFamily, EffectAuthorityOwner, EffectLifecycleFamilyInventoryRow,
    EffectLifecycleFamilyKey,
    EffectLoweredArtifactKind, EffectReceiptArtifactKind, EffectSupportPosture,
};

fn main() {
    let _ = EffectLifecycleFamilyInventoryRow {
        family_key: EffectLifecycleFamilyKey::Mutation,
        authority_owner: EffectAuthorityOwner::ForgeRelational,
        admitted_basis_families: vec![BasisFamily::CurrentHead],
        lowered_artifact_kind: EffectLoweredArtifactKind::LoweredMutationIntentDeclaration,
        receipt_artifact_kind: EffectReceiptArtifactKind::ForgeQueryIntentExecution,
        denial_posture: EffectSupportPosture::Denied,
        deferred_posture: EffectSupportPosture::Unsupported,
        row_digest: String::new(),
    };
}
