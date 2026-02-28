//! Public API for the Boundary Representation (B-Rep) subsystem.
//!
//! External components must import from here, never from `data/` or `logic/` directly.
//! Internal restructuring of subdirectories does not affect downstream consumers.

// ── Topological Entities (The Mesh) ───────────────────────────────────
pub use super::data::mesh::face::FaceData;
pub use super::data::mesh::half_edge::HalfEdgeData;
pub use super::data::mesh::vertex::VertexData;
pub use super::data::mesh::boundary_loop::LoopData;
pub use super::data::mesh::edge::EdgeData;

// ── Containment Entities (The Volumetric Hierarchy) ───────────────────
pub use super::data::containment::body::BodyData;
pub use super::data::containment::lump::LumpData;
pub use super::data::containment::region::RegionData;
pub use super::data::containment::shell::{ShellData, ShellKind, ShellOrientation};

// ── Core Storage ──────────────────────────────────────────────────────
pub use super::data::storage::arena::TopologyArena;
pub use super::data::entity_kind::EntityKind;
