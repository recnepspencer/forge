//! EMBER BSP Merge Pipeline.
//!
//! DOMAIN: Exact boolean operations on planar solids via BSP tree merge
//! (Bernstein/Naylor algorithm). This is a fully independent pipeline —
//! it never delegates to the legacy split-classify-stitch path.
//!
//! PIPELINE:
//!   1. Convert each input solid → `BspSolid` (convex planes → BSP tree)
//!   2. Merge BSP trees (exact rational arithmetic, zero f64 comparisons)
//!   3. Extract boundary ConvexCells from merged tree
//!   4. Build halfedge mesh from ConvexCells
//!
//! DEPENDENCIES: forge-geom (BspSolid, merge_bsp, convex_to_bsp,
//!   extract_boundary_cells), mesh_builder (build_halfedge_mesh)
//!
//! INVARIANTS:
//!   - All merge decisions use exact Rational::sign() (zero tolerance)
//!   - Vertices are 3-plane intersections until mesh extraction
//!   - Never delegates to legacy split-classify-stitch pipeline

pub mod schema;
pub mod quantize;
pub mod eval;
pub mod classify;
pub mod checkpoint;
pub mod mesh;
#[cfg(test)]
mod tests;

pub use schema::QuantizedSpace;
pub use eval::{execute_ember_boolean, execute_boolean_adaptive, EmberError};
