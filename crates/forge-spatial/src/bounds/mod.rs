//! Bounds queries for topology entities.
//!
//! DOMAIN: AABB aggregation over faces, shells, regions, lumps, and bodies.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-geom (Aabb).
//! INVARIANTS: No topology mutation. Returns Ok(None) when no vertex positions present.

pub mod face;
pub mod solid;
