mod evaluation;
mod outcomes;

pub use evaluation::{
    assess_spatial_identity_continuity_from_analysis,
    assess_spatial_identity_continuity_from_resolution,
};
pub use outcomes::{
    SpatialIdentityContinuityAssessment, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass,
};

#[cfg(test)]
mod tests;
