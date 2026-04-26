use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewEffectBindingDisposition,
    ForgeQueryPreviewHandleBindingEvidence, ForgeQueryPreviewHandleBindingFamily,
};

fn main() {
    let _forged = ForgeQueryPreviewHandleBindingEvidence {
        label: "preview".to_string(),
        handle_name: "ui.effect".to_string(),
        family: ForgeQueryPreviewHandleBindingFamily::Effect,
        source_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
        preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
        effect_policy: ForgeQueryEffectPolicy::DeriveOnly,
        effect_disposition: Some(ForgeQueryPreviewEffectBindingDisposition::Muted),
        basis_evidence: Vec::new(),
        effect_delivery_admitted: false,
        pending_write_intent_admitted: false,
        authoritative_side_effect_admitted: false,
    };
}
