//! Unified read interface for geometry data.
//!
//! DOMAIN: `GeometryView` is the polymorphic read trait that consumers use
//! to access geometry regardless of whether they're reading from a resting
//! `GeometryStore` or a mid-transaction `GeometryDraft`.
//!
//! All logic-layer functions that only READ geometry should accept
//! `view: &impl GeometryView` instead of `&GeometryStore`.

use worth_geom::facade::Plane;
use worth_math::arithmetic::Rational;
use forge_topo::handles::{FaceId, VertexId};

use super::super::data::position::ExactPosition;

/// Unified read interface for geometry data.
///
/// Implemented by both `GeometryStore` (resting) and `GeometryDraft`
/// (transactional). Consumers that only read geometry accept
/// `&impl GeometryView`, which makes them work identically on either.
pub trait GeometryView {
    /// Get the plane equation for a face.
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane>;

    /// Get the exact position data for a vertex.
    fn get_vertex_exact(&self, vertex: VertexId) -> Option<&ExactPosition>;

    /// Get the approximate f64 position for a vertex.
    fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        self.get_vertex_exact(vertex).map(|ep| ep.approx())
    }

    /// Get the exact rational position for a vertex.
    fn get_vertex_position_exact(&self, vertex: VertexId) -> Option<&[Rational; 3]> {
        self.get_vertex_exact(vertex).map(|ep| ep.exact())
    }

    /// Number of face→plane bindings.
    fn face_plane_count(&self) -> usize;

    /// Number of vertex→position bindings.
    fn vertex_position_count(&self) -> usize;

    /// Whether there are any parametric surfaces (false in Phase 1).
    fn has_parametric_surfaces(&self) -> bool;

    /// Whether a face has a plane binding.
    fn has_face_plane(&self, face: FaceId) -> bool;

    /// Whether a vertex has a position binding.
    fn has_vertex_position(&self, vertex: VertexId) -> bool;

    /// Iterate over all approximate vertex positions (for model-scale, AABB, etc.)
    ///
    /// Returns boxed iterator to avoid leaking concrete types. The overhead
    /// is negligible — this is used for bulk operations, not hot paths.
    fn vertex_positions_approx(&self) -> Box<dyn Iterator<Item = &[f64; 3]> + '_>;
}

// ── Implementations ──────────────────────────────────────────────────────

use super::super::data::draft::GeometryDraft;
use super::super::data::store::GeometryStore;

impl GeometryView for GeometryStore {
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.planes.get(face)
    }

    fn get_vertex_exact(&self, vertex: VertexId) -> Option<&ExactPosition> {
        self.positions.get(vertex)
    }

    fn face_plane_count(&self) -> usize {
        self.planes.len()
    }

    fn vertex_position_count(&self) -> usize {
        self.positions.len()
    }

    fn has_parametric_surfaces(&self) -> bool {
        !self.surfaces.is_empty()
    }

    fn has_face_plane(&self, face: FaceId) -> bool {
        self.planes.contains(face)
    }

    fn has_vertex_position(&self, vertex: VertexId) -> bool {
        self.positions.contains(vertex)
    }

    fn vertex_positions_approx(&self) -> Box<dyn Iterator<Item = &[f64; 3]> + '_> {
        Box::new(self.positions.values().map(|ep| ep.approx()))
    }
}

impl GeometryView for GeometryDraft {
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.planes.get(face)
    }

    fn get_vertex_exact(&self, vertex: VertexId) -> Option<&ExactPosition> {
        self.positions.get(vertex)
    }

    fn face_plane_count(&self) -> usize {
        self.planes.len()
    }

    fn vertex_position_count(&self) -> usize {
        self.positions.len()
    }

    fn has_parametric_surfaces(&self) -> bool {
        !self.surfaces.is_empty()
    }

    fn has_face_plane(&self, face: FaceId) -> bool {
        self.planes.contains(face)
    }

    fn has_vertex_position(&self, vertex: VertexId) -> bool {
        self.positions.contains(vertex)
    }

    fn vertex_positions_approx(&self) -> Box<dyn Iterator<Item = &[f64; 3]> + '_> {
        Box::new(self.positions.values().map(|ep| ep.approx()))
    }
}
