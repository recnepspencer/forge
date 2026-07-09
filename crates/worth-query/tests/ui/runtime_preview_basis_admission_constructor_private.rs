use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryPreviewBasisAdmission,
};

fn main() {
    let _ = WorthQueryPreviewBasisAdmission {
        label: String::new(),
        effect_policy: WorthQueryEffectPolicy::DeriveOnly,
        authority_lane: WorthQueryAuthorityLane::PreviewTruth,
        evidence: Vec::new(),
    };
}
