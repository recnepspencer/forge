//! Spatial queries bridging forge-topo topology and forge-geom geometry.
//!
//! DOMAIN: Point-in-solid classification, AABB bounds aggregation, and
//!         geometric invariant validation — functions that require both
//!         topology traversal (via forge-topo) and geometric types
//!         (via forge-geom) and therefore cannot live in either crate alone.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-geom (Aabb,
//!               BvhNode, polygon helpers), forge-math (predicates, linalg).
//!
//! INVARIANTS:
//! - Geometry is always received via caller-provided position callbacks.
//! - No topology mutation occurs in this crate.
//! - All public functions are deterministic for identical inputs.

pub mod bounds;
pub mod classify;
pub mod integrity;

pub use bounds::face::{all_face_bounds, face_bounds};
pub use bounds::solid::{lump_bounds, region_bounds, shell_bounds, solid_bounds};
pub use classify::point_in_solid::{classify_point_in_solid, classify_point_with_perturbation};
pub use classify::point_on_face::{classify_point_on_face, FacePointClassification};
/// Crate-level re-exports for the most commonly used spatial query types.
pub use classify::schema::{PointClassification, SpatialAccelerator};
pub use integrity::validate_geometric_invariants;
