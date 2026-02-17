//! DOMAIN: Plane Primitive
//! INVARIANTS:
//! - Plane normals are always non-zero (validated at construction)
//! - Point classification uses `orient3d` → `CertifiedTriSign` (D3)
//! - No raw f64 comparisons for topology decisions (D0)
//!
//! DEPENDENCIES: `forge-math` (predicates, sign, error)

mod schema;
pub(crate) mod eval;
#[cfg(test)]
mod tests;

pub use schema::{Plane, PlaneRelation};
pub use eval::{classify_point, signed_distance, intersect_three_planes, to_plane_relation, is_coplanar};
