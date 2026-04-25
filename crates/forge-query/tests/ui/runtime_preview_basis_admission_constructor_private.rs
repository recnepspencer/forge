use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission,
};

fn main() {
    let _ = ForgeQueryPreviewBasisAdmission {
        label: String::new(),
        effect_policy: ForgeQueryEffectPolicy::DeriveOnly,
        authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
        evidence: Vec::new(),
    };
}
