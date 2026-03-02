//! Public façade for spatial operations.
//!
//! DOMAIN: Re-exports the key spatial query functions.
//! External components depend ONLY on this façade.

// Bounds
pub use super::bounds::distance::{combined_solid_scale, compute_solid_centroid, compute_solid_ray_extent};
pub use super::bounds::face::{all_face_bounds, face_bounds, face_vertex_positions};
pub use super::bounds::solid::{lump_bounds, region_bounds, shell_bounds, solid_bounds};
pub use super::bounds::proximity::{find_coincident_vertex, ProximityResult};

// Classification
pub use super::classify::face_sampling::face_interior_samples;
pub use super::classify::point_in_solid::{classify_point_in_solid, classify_point_with_perturbation};
pub use super::classify::point_on_face::{classify_point_on_face, FacePointClassification};
pub use super::classify::schema::{PointClassification, SpatialAccelerator};
