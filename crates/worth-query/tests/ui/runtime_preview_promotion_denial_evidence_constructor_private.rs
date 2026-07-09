use worth_query::facade::{
    WorthQueryEffectPolicy, WorthQueryPreviewPromotionDenialEvidence,
    WorthQueryPreviewPromotionDenialKind,
};

fn main() {
    let _worthd = WorthQueryPreviewPromotionDenialEvidence {
        label: "preview".to_string(),
        kind: WorthQueryPreviewPromotionDenialKind::StaleBasis,
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        basis_evidence: Vec::new(),
        basis_snapshot_token: "a".to_string(),
        promotion_snapshot_token: "b".to_string(),
        staged_preview_write_count: 1,
        promoted_write_count: 0,
        failed_write_sequence: None,
        preview_binding_count: 0,
        reason: "Worthd".to_string(),
        denial_digest: String::new(),
    };
}
