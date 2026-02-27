//! # forge-geom
//!
//! Analytic surfaces, NURBS, and curve representations
//! for the Forge geometry kernel.
//!
//! Geometry is a binding layer — it may be approximate, but it carries
//! bounded error metrics and never corrupts topology (Doctrine D3).

#![forbid(unsafe_code)]

pub mod prelude;
pub mod traits;

pub mod algorithms;
pub mod coedge;
pub mod curve;
pub mod primitives;
pub mod spatial;
pub mod surface;

// Re-exports for cleaner API (optional, but requested "public re-exports only")
pub use algorithms::chord::{clip_line_to_face_polygon, compute_intersection_line};
pub use primitives::aabb::Aabb;
pub use primitives::implicit_vertex::Vertex;
pub use primitives::plane::Plane;
pub use primitives::ray::{
    compute_ray_plane_intersection, dominant_projection_axes, resolve_zero_edge,
    scanline_edge_crossing, EdgeTieBreaker,
};
pub use primitives::vertex_geom::{VertexGeom, VertexProvenance};
pub use spatial::bsp::PlaneSet; // Exposed for tests mostly?
pub use spatial::bsp::{BspOp, BspSolid};
pub use spatial::bvh::BvhNode;
pub use spatial::edge_match::{fuzzy_match_edges, FuzzyMatchMode};
pub use spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};
pub use traits::EvaluateNormal;

// Phase 4 geometry types
pub use coedge::{Coedge, ParametricCurve2D};
pub use curve::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex};
pub use surface::classify_surface_pair;
pub use surface::{ParameterDomain, SurfaceData, SurfaceKind, SurfaceRelation};

/// Standard grid scale for spatial hashing (1 unit = 1e6 integers).
pub const GRID_SCALE: f64 = 1e6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
