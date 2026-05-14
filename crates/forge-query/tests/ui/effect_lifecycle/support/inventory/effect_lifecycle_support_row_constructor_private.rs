use forge_query::facade::{
    BasisFamily, EffectAuthorityOwner, EffectFamily, EffectLifecycleSupportRow,
    EffectLoweredArtifactKind, EffectReceiptArtifactKind, EffectSupportCause, EffectSupportPosture,
};

fn main() {
    let _ = EffectLifecycleSupportRow {
        basis_family: BasisFamily::CurrentHead,
        effect_family: EffectFamily::Mutation,
        authority_owner: EffectAuthorityOwner::ForgeRelational,
        lowered_artifact_kind: EffectLoweredArtifactKind::LoweredMutationIntentDeclaration,
        receipt_artifact_kind: EffectReceiptArtifactKind::ForgeQueryIntentExecution,
        posture: EffectSupportPosture::Admitted,
        cause: EffectSupportCause::Supported,
        row_digest: String::new(),
    };
}
