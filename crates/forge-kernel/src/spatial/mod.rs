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

pub use forge_spatial::bounds::face::{all_face_bounds, face_bounds};
pub use forge_spatial::bounds::solid::{lump_bounds, region_bounds, shell_bounds, solid_bounds};

pub use forge_spatial::integrity::validate_geometric_invariants;
