use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryPreviewCloseoutKind,
    WorthQueryPreviewOutcomeInspection,
};

fn main() {
    let _worthd = WorthQueryPreviewOutcomeInspection {
        label: String::new(),
        closeout_kind: WorthQueryPreviewCloseoutKind::Discarded,
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        promoted: false,
        discarded: true,
        write_count: 0,
        preview_binding_count: 0,
        live_binding_count: 0,
        computed_binding_count: 0,
        effect_binding_count: 0,
        subscription_residue_count: 0,
        derived_runtime_residue_count: 0,
        effect_delivery_residue_count: 0,
        pending_write_intent_residue_count: 0,
        preview_write_staging_count: 0,
        promoted_write_count: 0,
        authoritative_residue_count: 0,
        source_lane: WorthQueryAuthorityLane::PreviewTruth,
        target_lane: WorthQueryAuthorityLane::PreviewTruth,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        closeout_digest: String::new(),
        residue_digest: String::new(),
        inspection_digest: String::new(),
    };
}
