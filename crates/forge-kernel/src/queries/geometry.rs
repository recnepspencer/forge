//! Geometry state query traits.
//!
//! DOMAIN: Named interfaces for looking up geometric data bound to topology
//! handles. Used by operations that need face planes, vertex positions,
//! or curve/surface data without depending on `GeometryState` directly.

use forge_geom::Plane;
use forge_topo::handles::{FaceId, VertexId};

use crate::geometry_state::GeometryState;

/// Face-level geometry access.
///
/// Used by: boolean classify (face orientation), split (face plane lookup),
/// postprocess (coplanar detection), mesh builder (normal extraction).
pub trait FaceGeometryQuery {
    /// Get the analytic plane for a face, if bound.
    fn face_plane(&self, face: FaceId) -> Option<&Plane>;
}

impl FaceGeometryQuery for GeometryState {
    fn face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.get_face_plane(face)
    }
}

/// Vertex-level geometry access.
///
/// Used by: boolean split (vertex position lookup), mesh builder (coordinate extraction),
/// assembly stitch (weld candidate positions).
pub trait VertexGeometryQuery {
    /// Get the f64 position for a vertex, if bound.
    fn vertex_position(&self, vertex: VertexId) -> Option<[f64; 3]>;
}

impl VertexGeometryQuery for GeometryState {
    fn vertex_position(&self, vertex: VertexId) -> Option<[f64; 3]> {
        self.get_vertex_position(vertex).copied()
    }
}
