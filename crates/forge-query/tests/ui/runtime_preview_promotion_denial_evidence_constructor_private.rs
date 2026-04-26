use forge_query::facade::{
    ForgeQueryEffectPolicy, ForgeQueryPreviewPromotionDenialEvidence,
    ForgeQueryPreviewPromotionDenialKind,
};

fn main() {
    let _forged = ForgeQueryPreviewPromotionDenialEvidence {
        label: "preview".to_string(),
        kind: ForgeQueryPreviewPromotionDenialKind::StaleBasis,
        effect_policy: ForgeQueryEffectPolicy::DeriveOnly,
        basis_evidence: Vec::new(),
        basis_snapshot_token: "a".to_string(),
        promotion_snapshot_token: "b".to_string(),
        staged_preview_write_count: 1,
        promoted_write_count: 0,
        failed_write_sequence: None,
        preview_binding_count: 0,
        reason: "forged".to_string(),
        denial_digest: String::new(),
    };
}
