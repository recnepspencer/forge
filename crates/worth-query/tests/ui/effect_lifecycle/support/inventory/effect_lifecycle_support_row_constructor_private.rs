use worth_query::facade::{
    BasisFamily, EffectAuthorityOwner, EffectFamily, EffectLifecycleSupportRow,
    EffectLoweredArtifactKind, EffectReceiptArtifactKind, EffectSupportCause, EffectSupportPosture,
};

fn main() {
    let _ = EffectLifecycleSupportRow {
        basis_family: BasisFamily::CurrentHead,
        effect_family: EffectFamily::Mutation,
        authority_owner: EffectAuthorityOwner::WorthRelational,
        lowered_artifact_kind: EffectLoweredArtifactKind::LoweredMutationIntentDeclaration,
        receipt_artifact_kind: EffectReceiptArtifactKind::WorthQueryIntentExecution,
        posture: EffectSupportPosture::Admitted,
        cause: EffectSupportCause::Supported,
        row_digest: String::new(),
    };
}
