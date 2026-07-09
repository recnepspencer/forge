use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryPreviewEffectBindingDisposition,
    WorthQueryPreviewHandleBindingEvidence, WorthQueryPreviewHandleBindingFamily,
};

fn main() {
    let _worthd = WorthQueryPreviewHandleBindingEvidence {
        label: "preview".to_string(),
        handle_name: "ui.effect".to_string(),
        family: WorthQueryPreviewHandleBindingFamily::Effect,
        source_lane: WorthQueryAuthorityLane::EffectDeliveryState,
        preview_lane: WorthQueryAuthorityLane::PreviewTruth,
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        effect_disposition: Some(WorthQueryPreviewEffectBindingDisposition::Muted),
        basis_evidence: Vec::new(),
        effect_delivery_admitted: false,
        pending_write_intent_admitted: false,
        authoritative_side_effect_admitted: false,
    };
}
