use worth_spatial::facade::{
    assess_spatial_identity_continuity_from_analysis,
    assess_spatial_identity_continuity_from_resolution, prepare_spatial_intent_preview,
    prepare_spatial_intent_preview_with_capabilities,
    prepare_spatial_intent_preview_with_capabilities_and_profile,
    prepare_spatial_intent_preview_with_profile, SpatialIntentPreview,
};

fn main() {
    let _ = assess_spatial_identity_continuity_from_analysis;
    let _ = assess_spatial_identity_continuity_from_resolution;
    let _ = prepare_spatial_intent_preview;
    let _ = prepare_spatial_intent_preview_with_capabilities;
    let _ = prepare_spatial_intent_preview_with_capabilities_and_profile;
    let _ = prepare_spatial_intent_preview_with_profile;
    let _: Option<SpatialIntentPreview> = None;
}
