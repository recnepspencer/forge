//! Bounds and proximity queries for topology entities.
//!
//! DOMAIN: AABB aggregation over faces, shells, regions, lumps, and bodies.
//!         Vertex proximity and coincidence detection for mesh construction.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-geom (Aabb, point predicates).
//! INVARIANTS: No topology mutation. Returns Ok(None) when no vertex positions present.

pub mod distance;
pub mod face;
pub mod proximity;
pub mod solid;
