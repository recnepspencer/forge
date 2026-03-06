//! Public façade for spatial operations.
//!
//! DOMAIN: Re-exports the key spatial query functions.
//! External components depend ONLY on this façade.

// Bounds
pub use super::bounds::distance::{
    combined_solid_scale, compute_solid_centroid, compute_solid_ray_extent,
};
pub use super::bounds::face::{all_face_bounds, face_bounds, face_vertex_positions};
pub use super::bounds::proximity::{find_coincident_vertex, ProximityResult};
pub use super::bounds::solid::{lump_bounds, region_bounds, shell_bounds, solid_bounds};

// Classification
pub use super::classify::face_sampling::face_interior_samples;
pub use super::classify::normal_orientation::classify_face_normal_orientation;
pub use super::classify::point_in_solid::{
    classify_point_in_solid, classify_point_with_perturbation,
};
pub use super::classify::point_on_face::{classify_point_on_face, FacePointClassification};
pub use super::classify::schema::{NormalClassification, PointClassification, SpatialAccelerator};

// Continuity
pub use super::continuity::{
    edge_dihedral_angle, face_normal_from_outer_loop, find_g1_chain, is_edge_g1_continuous,
};

// Centroid
pub use super::centroid::face_centroid;

// Volume
pub use super::volume::{collect_face_positions, compute_shell_signed_volume};

// Healing
pub use super::healing::{heal_shell_orientation, HealingResult};

// Simplify
pub use super::simplify::{consolidate_one_collinear_vertex, find_collinear_vertex_candidate};
