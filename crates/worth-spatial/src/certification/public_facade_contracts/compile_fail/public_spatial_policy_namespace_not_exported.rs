use worth_spatial::facade::policy::{
    SpatialArbitrationConflict, SpatialArbitrationPolicyProfile, SpatialPreviewRichness,
};

fn main() {
    let _ = SpatialArbitrationConflict::analyze_with_capabilities_and_profile;
    let _ = SpatialArbitrationPolicyProfile::bim_host_friendly();
    let _ = SpatialPreviewRichness::HighFidelity;
}
