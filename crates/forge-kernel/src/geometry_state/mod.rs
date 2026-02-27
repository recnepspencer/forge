//! DOMAIN: Side-car geometry storage for topology entities.
//!
//! Stores per-face planes and per-vertex positions alongside the
//! `TopologyArena`. The topology layer stores structure only (Architecture
//! Rule 2.3); this module bridges topology handles to geometric data.
//!
//! INVARIANTS:
//! - Every face in the topology should have a corresponding plane entry
//! - Every vertex in the topology should have a corresponding position entry
//! - Implements `GeometrySource` (from `forge-math`) for use by `forge-geom` solvers
//!
//! DEPENDENCIES: `forge-math` (GeometrySource), `forge-core` (KernelError),
//!               `forge-topo` (handles), `forge-geom` (Plane)

#[cfg(test)]
mod adversarial_tests;
pub mod coalescence;
mod eval;
#[cfg(test)]
mod patch_tests;
pub(crate) mod schema;
pub mod split_propagation;
#[cfg(test)]
mod tests;

pub mod patch;

pub use coalescence::{snap_or_coalesce_vertex, CoalescenceResult};
pub use eval::build_position_lookup;
pub use patch::GeometryPatch;
pub use schema::ExactPosition;
pub use schema::GeometryState;
pub use split_propagation::propagate_curve_on_split;

use forge_core::ToleranceProvider;
use crate::geom_facade::Plane;
use forge_math::GeometrySource;
use forge_topo::handles::{FaceId, VertexId};

/// Read-only abstraction over geometry state to support both
/// immutable snapshots and mid-transaction patches.
pub trait GeometryView: ToleranceProvider + GeometrySource {
    /// Retrieve the plane for a face.
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane>;

    /// Retrieve the cached f64 position for a vertex.
    fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]>;
}

impl GeometryView for GeometryState {
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.get_face_plane(face)
    }

    fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        self.get_vertex_position(vertex)
    }
}

impl GeometryView for GeometryPatch {
    fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.get_face_plane(face)
    }

    fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        self.get_vertex_position(vertex)
    }
}
