//! Spatial query adapter — single crossing point for forge-spatial inside forge-kernel.
//!
//! DOMAIN: Re-exports point classification, bounds queries, and geometric
//!         invariant validation from `forge-spatial`. All internal kernel code
//!         must import spatial primitives from this module, never directly from
//!         `forge_spatial::*`. If forge-spatial is reorganized, only this file changes.
//!
//! INVARIANTS: No kernel business logic lives here — only re-exports.

pub use forge_spatial::classify::schema::{PointClassification, SpatialAccelerator};

pub use forge_spatial::classify::point_on_face::{classify_point_on_face, FacePointClassification};

pub use forge_spatial::classify::point_in_solid::{
    classify_point_in_solid, classify_point_with_perturbation,
};

pub use forge_spatial::classify::face_sampling::face_interior_samples;

pub use forge_spatial::bounds::face::{all_face_bounds, face_bounds, face_vertex_positions};
pub use forge_spatial::bounds::solid::{lump_bounds, region_bounds, shell_bounds, solid_bounds};
pub use forge_spatial::bounds::distance::{combined_solid_scale, compute_solid_centroid, compute_solid_ray_extent};

pub use forge_spatial::integrity::validate_geometric_invariants;
