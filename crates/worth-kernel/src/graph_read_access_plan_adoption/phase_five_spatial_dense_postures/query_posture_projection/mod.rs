mod admitted_posture_projection;
mod denied_posture_projection;
mod posture_projection;
mod required_posture_projection;

pub(crate) use posture_projection::project_spatial_dense_postures;
pub use posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};
