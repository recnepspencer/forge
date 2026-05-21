mod frame_admission;
mod resolution;
mod witness_support;

pub use frame_admission::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameBasis, SpatialFrameError,
};
pub use resolution::{
    resolve_spatial_direction_witness, resolve_spatial_direction_witness_with_catalog,
    resolve_spatial_point_witness, resolve_spatial_point_witness_with_catalog,
    ResolvedSpatialDirectionWitness, ResolvedSpatialPointWitness,
};
pub use witness_support::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};
