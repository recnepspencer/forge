//! Public façade for forge-geom.
//!
//! All external crates should import from this facade, not deep internal
//! module paths. This ensures internal reorganisation does not break
//! consumers.
//!
//! DOMAIN STANDARDS: Each component exposes a single public façade file.
//! Internal complexity remains hidden.

// ── Primitives ───────────────────────────────────────────────────────────
pub use super::primitives::aabb::Aabb;
pub use super::primitives::implicit_vertex::Vertex;
pub use super::primitives::plane::{intersect_three_planes_exact, signed_distance, Plane};
pub use super::primitives::point::is_same_point_within;
pub use super::primitives::polygon::{compute_polygon_area, compute_polygon_centroid};
pub use super::primitives::ray::{
    compute_ray_plane_intersection, dominant_projection_axes, resolve_zero_edge,
    scanline_edge_crossing, EdgeTieBreaker,
};
pub use super::primitives::vertex_geom::{VertexGeom, VertexProvenance};

// ── Shape generators ────────────────────────────────────────────────────
pub use super::primitives::shapes::{
    block, cube, dodecahedron, prism, pyramid, tetrahedron, wedge,
};

// ── Spatial / BSP ───────────────────────────────────────────────────────
pub use super::spatial::bsp::{
    build_convex_polyhedron, BspConfig, BspOp, BspSolid, ConvexCell, PlaneSet,
};
pub use super::spatial::bvh::BvhNode;
pub use super::spatial::edge_match::{fuzzy_match_edges, FuzzyMatchMode};
pub use super::spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};

// ── Algorithms ──────────────────────────────────────────────────────────
pub use super::algorithms::chord::{clip_line_to_face_polygon, compute_intersection_line};
pub use super::algorithms::measurement::area::{dihedral_sine, tangent_frame, triangle_area_3d};
pub use super::algorithms::measurement::centroid::polyhedron_centroid;
pub use super::algorithms::measurement::distance::{distance, distance_squared};
pub use super::algorithms::measurement::volume::{polyhedron_volume, signed_tetra_volume_6x};
pub use super::algorithms::polygon::segment::point_on_segment;

// ── Curves & Surfaces (Phase 4) ─────────────────────────────────────────
pub use super::coedge::{Coedge, ParametricCurve2D};
pub use super::curve::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex};
pub use super::surface::classify_surface_pair;
pub use super::surface::{ParameterDomain, SurfaceData, SurfaceKind, SurfaceRelation};

// ── Traits ──────────────────────────────────────────────────────────────
pub use super::traits::EvaluateNormal;
