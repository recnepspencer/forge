//! Public API for forge-geom.
//!
//! Downstream crates should import from this facade, not deep internal
//! module paths. This ensures internal reorganisation does not break
//! consumers.

// ── Primitives ───────────────────────────────────────────────────────────
pub use super::primitives::aabb::Aabb;
pub use super::primitives::implicit_vertex::Vertex;
pub use super::primitives::plane::Plane;
pub use super::primitives::ray::{
    compute_ray_plane_intersection, dominant_projection_axes, resolve_zero_edge,
    scanline_edge_crossing, EdgeTieBreaker,
};
pub use super::primitives::vertex_geom::{VertexGeom, VertexProvenance};

// ── Spatial ──────────────────────────────────────────────────────────────
pub use super::spatial::bsp::{BspOp, BspSolid, PlaneSet};
pub use super::spatial::bvh::BvhNode;
pub use super::spatial::edge_match::{fuzzy_match_edges, FuzzyMatchMode};
pub use super::spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};

// ── Algorithms ───────────────────────────────────────────────────────────
pub use super::algorithms::chord::{clip_line_to_face_polygon, compute_intersection_line};
pub use super::algorithms::polygon::segment::point_on_segment;

// ── Curves & Surfaces (Phase 4) ──────────────────────────────────────────
pub use super::coedge::{Coedge, ParametricCurve2D};
pub use super::curve::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex};
pub use super::surface::classify_surface_pair;
pub use super::surface::{ParameterDomain, SurfaceData, SurfaceKind, SurfaceRelation};

// ── Traits ───────────────────────────────────────────────────────────────
pub use super::traits::EvaluateNormal;
