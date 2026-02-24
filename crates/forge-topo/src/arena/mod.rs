//! Arena-based entity storage for topology.
//!
//! DOMAIN: Entity allocation and retrieval with generational handles.
//!
//! INVARIANTS:
//! - Handles encode a generation counter to detect stale references
//! - Slots are reusable after deletion (generation is bumped)
//! - All accessors validate generation before returning data
//!
//! DEPENDENCIES: `handles` (typed IDs), `lineage` (inline provenance)
//!
//! SUBMODULES:
//! - `schema`: Data shapes (`FaceData`, `HalfEdgeData`, `VertexData`, `LoopData`, `Slot`)
//! - `eval`: `TopologyArena` struct and all arena operations

pub(crate) mod schema;
mod eval;

#[cfg(test)]
mod tests;

pub use schema::{FaceData, HalfEdgeData, VertexData, LoopData, ShellData, BodyData, LumpData, RegionData, EdgeData, ShellOrientation, ShellKind};
pub use eval::TopologyArena;
