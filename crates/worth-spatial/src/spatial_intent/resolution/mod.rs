mod point_resolution;
mod profiles;
mod resolution;

pub use point_resolution::{
    resolve_spatial_point_witness, resolve_spatial_point_witness_with_catalog,
    ResolvedSpatialPointWitness,
};
pub use profiles::{
    SpatialArbitrationPosture, SpatialIntentPolicyProfile, SpatialIntentPolicyProfileOverride,
    SpatialPreviewRichness, SpatialThresholdPosture,
};
pub use resolution::{
    resolve_spatial_direction_witness, resolve_spatial_direction_witness_with_catalog,
    ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};
