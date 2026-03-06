//! MutableDraft proxy methods for entity insert/remove.
//!
//! DOMAIN: Delegates insert/remove calls from MutableDraft to TopologyArena.
//! Each proxy method also records into the `MutationJournal` so that
//! the runner can auto-stamp deletions without operator involvement.

use forge_core::{EntityRef, KernelError};

/// Generate MutableDraft proxy methods with journal hooks.
macro_rules! define_draft_proxies {
    (@standard $m:ident, $id:ty, $data:ty) => {
        paste::paste! {
            impl crate::transactions::MutableDraft {
                #[doc = concat!("Insert a new ", stringify!($m), ".")]
                pub fn [<insert_ $m>](&mut self, data: $data) -> $id {
                    let id = self.arena.[<insert_ $m>](data);
                    self.mutation_journal.record_creation(EntityRef::from(id));
                    id
                }

                #[doc = concat!("Remove a ", stringify!($m), ".")]
                pub fn [<remove_ $m>](&mut self, id: $id) -> Result<$data, KernelError> {
                    // Capture EntityRef BEFORE the arena bumps the slot generation.
                    self.mutation_journal.record_destruction(EntityRef::from(id));
                    self.arena.[<remove_ $m>](id)
                }
            }
        }
    };
}

// ── MutableDraft Proxy Methods ─────────────────────────────────────

define_draft_proxies!(@standard face,      FaceId,     FaceData);
define_draft_proxies!(@standard half_edge, HalfEdgeId, HalfEdgeData);
define_draft_proxies!(@standard vertex,    VertexId,   VertexData);
define_draft_proxies!(@standard edge,      EdgeId,     EdgeData);
define_draft_proxies!(@standard shell,     ShellId,    ShellData);
define_draft_proxies!(@standard region,    RegionId,   RegionData);
define_draft_proxies!(@standard lump,      LumpId,     LumpData);
define_draft_proxies!(@standard body,      BodyId,     BodyData);

// Loop — keyword-safe draft proxies
impl crate::transactions::MutableDraft {
    /// Insert a new loop.
    pub fn insert_loop(&mut self, data: LoopData) -> LoopId {
        let id = self.arena.insert_loop(data);
        self.mutation_journal.record_creation(EntityRef::from(id));
        id
    }

    /// Remove a loop.
    pub fn remove_loop(&mut self, id: LoopId) -> Result<LoopData, KernelError> {
        // Capture EntityRef BEFORE the arena bumps the slot generation.
        self.mutation_journal
            .record_destruction(EntityRef::from(id));
        self.arena.remove_loop(id)
    }
}

// Imports needed by the macro-generated code
use crate::b_rep::data::containment::{BodyData, LumpData, RegionData, ShellData};
use crate::b_rep::data::mesh::{EdgeData, FaceData, HalfEdgeData, LoopData, VertexData};
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
