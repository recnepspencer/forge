//! Transactional mutation handle for topology + geometry.
//!
//! DOMAIN: Opens a transaction over a `KernelState`, providing mutable
//! access to topology and geometry patches. Supports commit
//! (finalize mutations) and rollback (restore original state).
//!
//! INVARIANTS:
//! - The original `TopologyState` is stored internally so `rollback()`
//!   cannot be paired with a mismatched snapshot.
//! - Drop without commit = rollback.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::transactions::{MutableDraft, TopologyState};

use crate::geometry::facade::{GeometryDraft, GeometryStore};

use super::super::data::state::KernelState;

/// Transactional mutation handle for topology + geometry.
///
/// Extends the `BRepWorkspace` pattern without `ModelingContext`
/// (context is passed as `&mut` to phase functions, not owned).
///
/// The original `TopologyState` is stored internally so that `rollback()`
/// cannot be paired with a mismatched snapshot. Drop without commit = rollback.
pub struct KernelDraft {
    draft: MutableDraft,
    geom: GeometryDraft,
    /// The pre-mutation topology snapshot, kept for guaranteed-safe rollback.
    original_topo: TopologyState,
}

impl KernelDraft {
    /// Create a new `KernelDraft` from an existing `KernelState`.
    ///
    /// Stores the original topology internally so `rollback()` always pairs
    /// geometry with the correct topological snapshot.
    pub fn new(state: KernelState) -> Self {
        let (topo, geom) = state.into_parts();
        let original_topo = topo.clone();
        Self {
            draft: topo.into_mutation(),
            geom: GeometryDraft::new(geom),
            original_topo,
        }
    }

    /// Read-only access to the topology arena.
    pub fn arena(&self) -> &TopologyArena {
        self.draft.arena()
    }

    /// Read-only access to the topology snapshot before mutations.
    pub fn original_topology(&self) -> &TopologyState {
        &self.original_topo
    }

    /// Mutable access to the topology draft.
    pub fn draft_mut(&mut self) -> &mut MutableDraft {
        &mut self.draft
    }

    /// Read-only access to the geometry draft.
    pub fn geometry(&self) -> &GeometryDraft {
        &self.geom
    }

    /// Mutable access to the geometry draft.
    pub fn geometry_mut(&mut self) -> &mut GeometryDraft {
        &mut self.geom
    }

    /// Destructure the draft into mutable borrows for leaf function calls.
    pub fn as_parts_mut(&mut self) -> (&mut MutableDraft, &mut GeometryDraft) {
        (&mut self.draft, &mut self.geom)
    }

    /// Discard all pending mutations and restore the pre-draft `KernelState`.
    ///
    /// Uses the original topology stored at construction time, guaranteeing
    /// that geometry and topology are always paired correctly.
    pub fn rollback(self) -> KernelState {
        KernelState::new(
            self.original_topo,
            self.geom.rollback(),
        )
    }

    /// Commit the transaction, finalizing all topology and geometry mutations.
    pub fn commit(self) -> Result<KernelState, KernelError> {
        let topo = self.draft.commit()?;
        Ok(KernelState::new(
            topo,
            self.geom.commit(),
        ))
    }
}
