//! DOMAIN: BSP-Based Convex Polyhedron Construction
//!
//! Builds bounded convex polyhedra from sets of half-space planes.
//! Each plane divides space; the intersection of all negative half-spaces
//! produces the convex cell.
//!
//! INVARIANTS:
//! - All vertices are exact 3-plane intersections
//! - Face polygons are convex and planar by construction
//! - Classification uses `signed_distance` for geometric clipping
//!
//! DEPENDENCIES: `plane` (Plane, classify_point, intersect_three_planes)

mod schema;
mod eval;
#[cfg(test)]
mod tests;

pub use schema::{ConvexCell, CellFace, CellVertex, BspTree, BspNode, PlaneSet};
pub use eval::{build_convex_polyhedron, clip_cell_by_plane, BspConfig};
