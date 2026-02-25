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

mod schema;
mod eval;
pub mod coalescence;
pub mod split_propagation;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod adversarial_tests;
#[cfg(test)]
mod patch_tests;

pub mod patch;

pub use schema::GeometryState;
pub use patch::GeometryPatch;
pub use schema::ExactPosition;
pub use eval::build_position_lookup;
pub use coalescence::{snap_or_coalesce_vertex, CoalescenceResult};
pub use split_propagation::propagate_curve_on_split;

use forge_topo::handles::{FaceId, VertexId};
use forge_geom::Plane;
use forge_core::ToleranceProvider;
use forge_math::GeometrySource;

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
