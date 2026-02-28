//! Arena-based entity storage for topology.
//!
//! DOMAIN: Entity allocation and retrieval with generational handles.
//!
//! INVARIANTS:
//! - Handles encode a generation counter to detect stale references
//! - Slots are reusable after deletion (generation is bumped)
//! - All accessors validate generation before returning data
//!
//! DEPENDENCIES: `handles` (typed IDs)
//!
//! SUBMODULES:
//! - `schema`: Data shapes (`FaceData`, `HalfEdgeData`, `VertexData`, `LoopData`, `Slot`)
//! - `eval`: `TopologyArena` struct and all arena operations

mod eval;
pub(crate) mod schema;

#[cfg(test)]
mod tests;

pub use eval::TopologyArena;
pub use schema::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation, VertexData,
};
