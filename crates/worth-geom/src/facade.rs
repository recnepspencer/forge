//! Public facade for worth-geom.
//!
//! All external crates should import from this facade, not deep internal
//! module paths. This keeps internal reorganization from leaking into
//! downstream Worth crates.

// Primitives
pub use super::primitives::aabb::Aabb;
pub use super::primitives::implicit_vertex::{
    orient3d_symbolic, resolve_position, select_best_triple, PlaneRef, Vertex,
};
pub use super::primitives::parameter_space::ParameterSpacePoint;
pub use super::primitives::plane::exact_eq as plane_exact_eq;
pub use super::primitives::plane::signed_distance as plane_signed_distance;
pub use super::primitives::plane::{
    are_parallel_exact, classify_point, classify_point_exact, coplanar_eq, exact_eq,
    intersect_edge_plane, intersect_three_planes_exact, normals_aligned_exact, signed_distance,
    to_plane_relation, Plane, PlaneRelation,
};
pub use super::primitives::point::is_same_point_within;
pub use super::primitives::polygon::{
    compute_largest_triangle_centroid, compute_polygon_area, compute_polygon_centroid,
};
pub use super::primitives::ray::{
    compute_ray_plane_intersection, dominant_projection_axes, resolve_zero_edge,
    scanline_edge_crossing, EdgeTieBreaker,
};
pub use super::primitives::shape_realization::{
    build_direct_realization_report, primitive_realization_exhaustion_witness_rows,
    realize_block_support, realize_prism_support, realize_pyramid_support,
    realize_tetrahedron_support, realize_tetrahedron_support_with_altitude_component,
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationError,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionReport,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationExhaustionWitnessRow,
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass, PrimitiveSupportRealization,
};
pub use super::primitives::shapes::{
    block, cube, dodecahedron, prism, pyramid, tetrahedron, wedge,
};
pub use super::primitives::vertex_geom::{VertexGeom, VertexProvenance};

// Surface and curve surface parameter types
pub use super::surface::eval::classify_surface_pair;
pub use super::surface::parameter_admission::{
    CanonicalParameterPoint, DomainParameterPoint, ParameterAxis, ParameterDomainError,
    PolygonalTrimmedParameterPoint, PolygonalTrimmedParameterRegion,
};
pub use super::surface::schema::{
    ParameterDomain, SurfaceData, SurfaceKind, SurfaceRelation, TriaxialEllipsoidDefinitionError,
};
pub use super::surface::traits::EvaluateSurface;
pub use super::surface::trim_ops::{TrimCurveOps, TrimOverlapResult};

// Spatial
pub use super::spatial::acceleration::bsp::{
    build_convex_polyhedron, clip_cell_by_plane, convex_to_bsp, extract_boundary_cells, merge_bsp,
    BspConfig, BspNode, BspOp, BspSolid, CellFace, CellVertex, ConvexCell, PlaneSet,
};
pub use super::spatial::acceleration::bvh::{query_overlapping_pairs, BvhNode};
pub use super::spatial::coordinate::bounds::compute_characteristic_scale;
pub use super::spatial::coordinate::local_space::{LocalCoordinateSpace, ScaleAnalysis};
pub use super::spatial::matching::coincidence::{CoincidenceGraph, CoincidenceKind};
pub use super::spatial::matching::edge_match::{
    fuzzy_match_edges, select_best_radial_match, DirectedEdge, EdgeMatch, FuzzyMatchMode,
};
pub use super::spatial::matching::epsilon_weld::EpsilonWelder;

// Algorithms
pub use super::algorithms::boundary_cert::eval::{
    build_projection_frame, certify_boundary, project_boundary_to_2d, project_point,
};
pub use super::algorithms::boundary_cert::schema::{
    BoundaryArrangement, BoundaryCertError, BoundaryRejectReason, ProjectedBoundary2D,
    ProjectionFrame2D, Segment2D, WeakSimpleCertificate,
};
pub use super::algorithms::boundary_cert::split::{
    ArrangementVertex, ArrangementVertexId, AtomicSegment2D,
};
pub use super::algorithms::intersection::chord::{
    chord_overlap_segment, clip_line_to_face_polygon, compute_intersection_line,
    project_interval_onto_direction,
};
pub use super::algorithms::intersection::clipping::clip_line_to_polygon;
pub use super::algorithms::intersection::overlap::polygons_overlap_3d;
pub use super::algorithms::intersection::polygon_overlap::{
    point_strictly_inside_polygon, polygons_overlap_2d, segments_properly_cross,
};
pub use super::algorithms::measurement::area::{dihedral_sine, tangent_frame, triangle_area_3d};
pub use super::algorithms::measurement::centroid::polyhedron_centroid;
pub use super::algorithms::measurement::distance::{distance, distance_squared};
pub use super::algorithms::measurement::volume::{polyhedron_volume, signed_tetra_volume_6x};
pub use super::algorithms::polygon::polygon::{bridge_polygon_hole, bridge_polygon_holes};
pub use super::algorithms::polygon::segment::point_on_segment;
pub use super::algorithms::sorting::angular_sort::sort_edges_radially;
pub use super::algorithms::triangulation::cdt::{
    triangulate_face_with_cut, triangulate_polygon_2d, CdtResult,
};

// Curves and authored geometry
pub use super::coedge::{Coedge, ParametricCurve2D};
pub use super::curve::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex};

// Traits
pub use super::traits::EvaluateNormal;
