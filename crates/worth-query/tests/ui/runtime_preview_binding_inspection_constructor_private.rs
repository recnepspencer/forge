use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryPreviewBindingInspection, WorthQueryPreviewHandleBindingFamily};

fn main() {
    let _worthd = WorthQueryPreviewBindingInspection {
        label: String::new(),
        handle_name: String::new(),
        family: WorthQueryPreviewHandleBindingFamily::LiveView,
        source_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        preview_lane: WorthQueryAuthorityLane::PreviewTruth,
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        effect_disposition: Some(String::new()),
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        effect_delivery_admitted: false,
        pending_write_intent_admitted: false,
        authoritative_side_effect_admitted: false,
        admission_digest: String::new(),
        inspection_digest: String::new(),
    };
}
