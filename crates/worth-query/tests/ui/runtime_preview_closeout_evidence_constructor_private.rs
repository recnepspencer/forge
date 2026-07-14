use worth_query::facade::runtime::{WorthQueryEffectPolicy, WorthQueryPreviewCloseoutEvidence, WorthQueryPreviewCloseoutKind};

fn main() {
    let _worthd = WorthQueryPreviewCloseoutEvidence {
        label: "preview".to_string(),
        kind: WorthQueryPreviewCloseoutKind::Discarded,
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        basis_evidence: Vec::new(),
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
        closeout_digest: String::new(),
    };
}
