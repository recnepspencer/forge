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
//! - `slot`: Generational slot wrapper and validation helpers
//! - `core`: TopologyArena struct definition and constructor
//! - `mesh_schema`: Data shapes for Face, HalfEdge, Vertex, Loop, Edge
//! - `containment_schema`: Data shapes for Body, Lump, Region, Shell
//! - `mesh_crud`: CRUD operations for mesh entities
//! - `containment_crud`: CRUD operations for containment entities

pub(crate) mod slot;
pub(crate) mod core;
pub(crate) mod mesh_schema;
pub(crate) mod containment_schema;
mod mesh_crud;
mod containment_crud;
mod indexes;

#[cfg(test)]
mod tests;

pub use core::TopologyArena;
pub use mesh_schema::{
    FaceData, HalfEdgeData, VertexData, LoopData, EdgeData,
};
pub use containment_schema::{
    BodyData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation,
};
