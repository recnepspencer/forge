//! Crate-neutral topology entity reference.
//!
//! DOMAIN: `EntityKind` and `EntityRef` live in `forge-core` so both
//! `forge-topo` and `forge-kernel` can reference topology entities
//! without upward dependencies.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Entity kind discriminant for typed topology references.
///
/// Each variant maps to a fixed u8 tag for bit-packing into `EntityRef`.
/// This enum lives in `forge-core` so both `forge-topo` and `forge-kernel`
/// can reference it without upward dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EntityKind {
    Face = 0,
    HalfEdge = 1,
    Vertex = 2,
    Loop = 3,
    Body = 4,
    Shell = 5,
    Edge = 6,
    Lump = 7,
    Region = 8,
}

impl EntityKind {
    /// The canonical string name for this kind.
    pub fn as_str(self) -> &'static str {
        match self {
            EntityKind::Face => "Face",
            EntityKind::HalfEdge => "HalfEdge",
            EntityKind::Vertex => "Vertex",
            EntityKind::Loop => "Loop",
            EntityKind::Body => "Body",
            EntityKind::Shell => "Shell",
            EntityKind::Edge => "Edge",
            EntityKind::Lump => "Lump",
            EntityKind::Region => "Region",
        }
    }

    /// Parse a string into an EntityKind.
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "Face" => Some(EntityKind::Face),
            "HalfEdge" => Some(EntityKind::HalfEdge),
            "Vertex" => Some(EntityKind::Vertex),
            "Loop" => Some(EntityKind::Loop),
            "Body" => Some(EntityKind::Body),
            "Shell" => Some(EntityKind::Shell),
            "Edge" => Some(EntityKind::Edge),
            "Lump" => Some(EntityKind::Lump),
            "Region" => Some(EntityKind::Region),
            _ => None,
        }
    }

    /// Reconstruct from the raw u8 tag.
    fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(EntityKind::Face),
            1 => Some(EntityKind::HalfEdge),
            2 => Some(EntityKind::Vertex),
            3 => Some(EntityKind::Loop),
            4 => Some(EntityKind::Body),
            5 => Some(EntityKind::Shell),
            6 => Some(EntityKind::Edge),
            7 => Some(EntityKind::Lump),
            8 => Some(EntityKind::Region),
            _ => None,
        }
    }
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Crate-neutral reference to a topological entity.
///
/// Packed as a single `u64`: `[8-bit EntityKind tag | 24-bit generation | 32-bit index]`.
/// This eliminates heap allocation (no `String`) and enables O(1)
/// hashing/comparison.
///
/// - **Index**: 32-bit slot address (4 billion entities per kind)
/// - **Generation**: 24-bit reuse counter (16 million reuses per slot)
/// - **Kind**: 8-bit entity type discriminant
///
/// Used in `TracedDecision`, `LineageStore`, and `DecisionContext` to
/// scope decisions to specific entities without importing typed handles
/// from `forge-topo`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityRef(u64);

impl EntityRef {
    /// Create a new entity reference from kind, index, and generation.
    ///
    /// This is the preferred constructor — it preserves the full generational
    /// identity so that stale handles produce `None` from `LineageStore`
    /// lookups rather than silently returning wrong data.
    pub fn new(kind: EntityKind, index: u32, generation: u32) -> Self {
        let tag = kind as u64;
        let gen = (generation as u64) & 0x00FF_FFFF; // 24-bit generation
        Self((tag << 56) | (gen << 32) | (index as u64))
    }

    /// The entity kind.
    pub fn kind(self) -> EntityKind {
        let tag = (self.0 >> 56) as u8;
        EntityKind::from_tag(tag).expect("invalid EntityRef tag")
    }

    /// The arena index (slot position).
    pub fn index(self) -> u32 {
        (self.0 & 0xFFFF_FFFF) as u32
    }

    /// The generation counter for stale-handle detection.
    ///
    /// Truncated to 24 bits when packed. Returns 0 for legacy refs
    /// that were created before generational packing was introduced.
    pub fn generation(self) -> u32 {
        ((self.0 >> 32) & 0x00FF_FFFF) as u32
    }
}

impl fmt::Debug for EntityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntityRef({}#{}:gen{})", self.kind().as_str(), self.index(), self.generation())
    }
}

impl fmt::Display for EntityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}:gen{}", self.kind().as_str(), self.index(), self.generation())
    }
}

