use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewBindingInspection,
    ForgeQueryPreviewHandleBindingFamily,
};

fn main() {
    let _forged = ForgeQueryPreviewBindingInspection {
        label: String::new(),
        handle_name: String::new(),
        family: ForgeQueryPreviewHandleBindingFamily::LiveView,
        source_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
        effect_policy: ForgeQueryEffectPolicy::DeriveOnly,
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
