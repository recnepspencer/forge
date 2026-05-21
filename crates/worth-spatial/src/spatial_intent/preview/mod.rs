mod simulation;

pub use simulation::{
    prepare_spatial_intent_preview, prepare_spatial_intent_preview_with_capabilities,
    prepare_spatial_intent_preview_with_capabilities_and_profile,
    prepare_spatial_intent_preview_with_profile, SpatialIntentPreview,
    SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning,
};

#[cfg(test)]
mod tests;
