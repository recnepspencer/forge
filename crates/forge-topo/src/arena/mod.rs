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
//! - `entity`: EntityKind enum
//! - `crud_macro`: Macro-generated CRUD for all entity types
//! - `specializations`: Non-generic methods (insert_radial_pair, etc.)
//! - `indexes`: O(1) reverse indexes and reassignment methods
//! - `mesh_schema`: Data shapes for Face, HalfEdge, Vertex, Loop, Edge
//! - `containment_schema`: Data shapes for Body, Lump, Region, Shell

pub(crate) mod slot;
pub(crate) mod core;
pub mod entity;
pub(crate) mod mesh_schema;
pub(crate) mod containment_schema;
mod crud_macro;
mod specializations;
mod indexes;

#[cfg(test)]
mod tests;

pub use core::TopologyArena;
pub use entity::EntityKind;
pub use mesh_schema::{
    FaceData, HalfEdgeData, VertexData, LoopData, EdgeData,
};
pub use containment_schema::{
    BodyData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation,
};
