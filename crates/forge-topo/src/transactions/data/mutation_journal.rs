//! Per-operation mutation journal for automatic lineage tracking.
//!
//! DOMAIN: Records every entity created and destroyed during a single
//! operator execution. The runner reads the journal after `op.execute()`
//! to auto-stamp deletions and build precise `LineageDelta` values.
//!
//! INVARIANTS:
//! - Every `remove_*` call on `MutableDraft` records a `destroyed` entry
//! - Every `insert_*` call on `MutableDraft` records a `created` entry
//! - The journal is reset at the start of each `execute()` call
//!
//! DEPENDENCIES: `forge_core::EntityRef`

use forge_core::EntityRef;

/// Tracks entity mutations during a single operator execution.
///
/// The [`MutableDraft`](crate::transactions::MutableDraft) proxy layer
/// populates this automatically — operators never interact with it directly.
/// After `op.execute()` returns, the runner reads `destroyed` to auto-stamp
/// deletion lineage, eliminating the need for manual `stamp_deletions()` calls.
#[derive(Debug, Clone)]
pub struct MutationJournal {
    /// Entities created during this operation (populated by `insert_*` proxies).
    created: Vec<EntityRef>,
    /// Entities destroyed during this operation (populated by `remove_*` proxies).
    destroyed: Vec<EntityRef>,
}

impl MutationJournal {
    /// Create an empty journal.
    pub fn new() -> Self {
        Self {
            created: Vec::new(),
            destroyed: Vec::new(),
        }
    }

    /// Record an entity creation.
    #[inline]
    pub fn record_creation(&mut self, entity: EntityRef) {
        self.created.push(entity);
    }

    /// Record an entity destruction.
    ///
    /// Must be called *before* the arena slot generation is bumped,
    /// so the `EntityRef` carries the pre-removal identity.
    #[inline]
    pub fn record_destruction(&mut self, entity: EntityRef) {
        self.destroyed.push(entity);
    }

    /// Reset the journal for a new operation.
    pub fn reset(&mut self) {
        self.created.clear();
        self.destroyed.clear();
    }

    /// Entities created during the current operation.
    pub fn created(&self) -> &[EntityRef] {
        &self.created
    }

    /// Entities destroyed during the current operation.
    pub fn destroyed(&self) -> &[EntityRef] {
        &self.destroyed
    }

    /// Drain all destroyed entities, leaving the list empty.
    pub fn drain_destroyed(&mut self) -> Vec<EntityRef> {
        std::mem::take(&mut self.destroyed)
    }

    /// Drain all created entities, leaving the list empty.
    pub fn drain_created(&mut self) -> Vec<EntityRef> {
        std::mem::take(&mut self.created)
    }

    /// Total number of entities created.
    pub fn creation_count(&self) -> usize {
        self.created.len()
    }

    /// Total number of entities destroyed.
    pub fn destruction_count(&self) -> usize {
        self.destroyed.len()
    }

    /// Count created entities grouped by `EntityKind`.
    pub fn count_created(&self) -> EntityKindCounts {
        EntityKindCounts::from_refs(&self.created)
    }

    /// Count destroyed entities grouped by `EntityKind`.
    pub fn count_destroyed(&self) -> EntityKindCounts {
        EntityKindCounts::from_refs(&self.destroyed)
    }
}

/// Per-`EntityKind` tallies derived from a journal's entity lists.
#[derive(Debug, Clone, Default)]
pub struct EntityKindCounts {
    pub faces: u32,
    pub half_edges: u32,
    pub vertices: u32,
    pub loops: u32,
    pub edges: u32,
    pub shells: u32,
    pub bodies: u32,
    pub lumps: u32,
    pub regions: u32,
}

impl EntityKindCounts {
    /// Build counts by iterating a slice of `EntityRef`.
    pub fn from_refs(refs: &[EntityRef]) -> Self {
        use forge_core::EntityKind;
        let mut c = Self::default();
        for r in refs {
            match r.kind() {
                EntityKind::Face     => c.faces += 1,
                EntityKind::HalfEdge => c.half_edges += 1,
                EntityKind::Vertex   => c.vertices += 1,
                EntityKind::Loop     => c.loops += 1,
                EntityKind::Edge     => c.edges += 1,
                EntityKind::Shell    => c.shells += 1,
                EntityKind::Body     => c.bodies += 1,
                EntityKind::Lump     => c.lumps += 1,
                EntityKind::Region   => c.regions += 1,
            }
        }
        c
    }

    /// Sum of all entity counts.
    pub fn total(&self) -> u32 {
        self.faces + self.half_edges + self.vertices + self.loops
            + self.edges + self.shells + self.bodies + self.lumps + self.regions
    }
}

impl Default for MutationJournal {
    fn default() -> Self {
        Self::new()
    }
}
