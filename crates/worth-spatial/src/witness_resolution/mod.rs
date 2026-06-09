mod admitted_witness_requests;
mod frame_admission;
mod resolution;
mod witness_helper_entry;
mod witness_support;

pub(crate) use frame_admission::admit_spatial_frame;
pub use frame_admission::{AdmittedSpatialFrameRef, SpatialFrameBasis, SpatialFrameError};
pub use resolution::ResolvedSpatialDirectionWitness;
#[cfg(test)]
pub use resolution::ResolvedSpatialPointWitness;
pub use witness_support::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};

pub(crate) mod witness_resolution {
    #[cfg(test)]
    pub(crate) use super::witness_helper_entry::resolve_spatial_direction_witness;
    #[cfg(test)]
    pub(crate) use super::witness_helper_entry::resolve_spatial_point_witness;
    pub(crate) use super::witness_helper_entry::{
        resolve_spatial_direction_witness_with_catalog, resolve_spatial_point_witness_with_catalog,
    };
}
