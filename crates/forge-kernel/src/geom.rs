//! Geometry facade — single crossing point for forge-geom inside forge-kernel.
//!
//! DOMAIN: Re-exports all geometry types and functions consumed by the kernel.
//! All internal kernel code must import geometry from this module, never
//! directly from `forge_geom::*`. If forge-geom is reorganized, only this
//! file changes.
//!
//! INVARIANTS: No kernel business logic lives here — only re-exports.

// ── Plane ────────────────────────────────────────────────────────────────────
pub use forge_geom::primitives::plane::Plane;
pub use forge_geom::primitives::plane::exact_eq as plane_exact_eq;
pub use forge_geom::primitives::plane::normals_aligned_exact;
pub use forge_geom::primitives::plane::intersect_three_planes_exact;
pub use forge_geom::primitives::plane::intersect_edge_plane;
pub use forge_geom::primitives::plane::are_parallel_exact as planes_are_parallel;
pub use forge_geom::primitives::plane::classify_point_exact;
pub use forge_geom::primitives::plane::signed_distance as plane_signed_distance;

// ── BSP ──────────────────────────────────────────────────────────────────────
pub use forge_geom::spatial::bsp::{
    BspConfig, BspNode, BspOp, BspSolid, ConvexCell, PlaneSet,
    build_convex_polyhedron, merge_bsp, extract_boundary_cells,
};

// ── BVH ──────────────────────────────────────────────────────────────────────
pub use forge_geom::spatial::bvh::{BvhNode, query_overlapping_pairs};

// ── Bounds ───────────────────────────────────────────────────────────────────
pub use forge_geom::spatial::bounds::compute_characteristic_scale;

// ── Edge matching ────────────────────────────────────────────────────────────
pub use forge_geom::spatial::edge_match::{
    fuzzy_match_edges, DirectedEdge, FuzzyMatchMode, select_best_radial_match,
};

// ── Epsilon welding ──────────────────────────────────────────────────────────
pub use forge_geom::spatial::epsilon_weld::EpsilonWelder;

// ── Local coordinate space ───────────────────────────────────────────────────
pub use forge_geom::spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};

// ── Coincidence ──────────────────────────────────────────────────────────────
pub use forge_geom::spatial::coincidence::{CoincidenceGraph, CoincidenceKind};

// ── Point utilities ──────────────────────────────────────────────────────────
pub use forge_geom::primitives::point::is_same_point_within;

// ── Implicit vertex ──────────────────────────────────────────────────────────
pub use forge_geom::primitives::implicit_vertex::{orient3d_symbolic, PlaneRef, Vertex};

// ── Shapes ───────────────────────────────────────────────────────────────────
pub mod shapes {
    pub use forge_geom::primitives::shapes::{cube, tetrahedron, dodecahedron};
}

// ── AABB ─────────────────────────────────────────────────────────────────────
pub use forge_geom::primitives::aabb::Aabb;

// ── Ray ──────────────────────────────────────────────────────────────────────
pub use forge_geom::primitives::ray::{
    compute_ray_plane_intersection, dominant_projection_axes, resolve_zero_edge,
    scanline_edge_crossing, EdgeTieBreaker,
};

// ── Surface ──────────────────────────────────────────────────────────────────
pub use forge_geom::surface::{
    classify_surface_pair, ParameterDomain, SurfaceData, SurfaceKind, SurfaceRelation,
};

// ── Curve ────────────────────────────────────────────────────────────────────
pub use forge_geom::curve::{
    CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex,
};
pub use forge_geom::coedge::{Coedge, ParametricCurve2D};

// ── Vertex geometry ──────────────────────────────────────────────────────────
pub use forge_geom::primitives::vertex_geom::{VertexGeom, VertexProvenance};

// ── Boundary certification ───────────────────────────────────────────────────
pub mod cert {
    pub use forge_geom::algorithms::boundary_cert::eval::{certify_boundary, project_boundary_to_2d};
    pub use forge_geom::algorithms::boundary_cert::schema::{
        WeakSimpleCertificate, BoundaryRejectReason,
    };
}

// ── Polygon algorithms ───────────────────────────────────────────────────────
pub mod polygon {
    pub use forge_geom::algorithms::polygon::bridge_polygon_hole;
}

// ── Clipping ─────────────────────────────────────────────────────────────────
pub mod clipping {
    pub use forge_geom::algorithms::clipping::{clip_line_to_polygon, clip_line_to_face_polygon};
    pub use forge_geom::algorithms::chord::compute_intersection_line;
}

// ── Traits ───────────────────────────────────────────────────────────────────
pub use forge_geom::traits::EvaluateNormal;

// ── Constants ────────────────────────────────────────────────────────────────
pub use forge_geom::GRID_SCALE;
