//! TopologyEntity marker trait and EntityKind discriminant.
//!
//! DOMAIN: Defines the zero-cost trait that associates each topology
//! entity type with its Id, Data, and kind discriminant. The
//! `define_topology_entities!` macro in `crud_macro.rs` implements
//! this trait and generates all CRUD methods.

/// Discriminant for topology entity types.
///
/// Used in `EntityRef`, lineage recording, tracing, and diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Vertex,
    HalfEdge,
    Edge,
    Loop,
    Face,
    Shell,
    Region,
    Lump,
    Body,
}

impl std::fmt::Display for EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertex => write!(f, "Vertex"),
            Self::HalfEdge => write!(f, "HalfEdge"),
            Self::Edge => write!(f, "Edge"),
            Self::Loop => write!(f, "Loop"),
            Self::Face => write!(f, "Face"),
            Self::Shell => write!(f, "Shell"),
            Self::Region => write!(f, "Region"),
            Self::Lump => write!(f, "Lump"),
            Self::Body => write!(f, "Body"),
        }
    }
}
