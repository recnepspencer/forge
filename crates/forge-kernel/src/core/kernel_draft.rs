use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::state::{MutableDraft, TopologyState};

use crate::brep::patch::BrepPatch;
use crate::core::KernelState;
use crate::geometry_state::GeometryPatch;

/// Transactional mutation handle for topology + geometry.
///
/// Extends the `BRepWorkspace` pattern without `ModelingContext`
/// (context is passed as `&mut` to phase functions, not owned).
///
/// The original `TopologyState` is stored internally so that `rollback()`
/// cannot be paired with a mismatched snapshot. Drop without commit = rollback.
pub struct KernelDraft {
    draft: MutableDraft,
    geom_patch: GeometryPatch,
    brep_patch: BrepPatch,
    /// The pre-mutation topology snapshot, kept for guaranteed-safe rollback.
    original_topo: TopologyState,
}

impl KernelDraft {
    /// Create a new `KernelDraft` from an existing `KernelState`.
    ///
    /// Stores the original topology internally so `rollback()` always pairs
    /// geometry with the correct topological snapshot.
    pub fn new(state: KernelState) -> Self {
        let (topo, geom, brep) = state.into_parts();
        let original_topo = topo.clone();
        Self {
            draft: topo.into_mutation(),
            geom_patch: GeometryPatch::new(geom),
            brep_patch: BrepPatch::new(brep),
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

    /// Read-only access to the geometry patch.
    pub fn geometry(&self) -> &GeometryPatch {
        &self.geom_patch
    }

    /// Mutable access to the geometry patch.
    pub fn geometry_mut(&mut self) -> &mut GeometryPatch {
        &mut self.geom_patch
    }

    /// Read-only access to the B-Rep patch.
    pub fn brep(&self) -> &BrepPatch {
        &self.brep_patch
    }

    /// Mutable access to the B-Rep patch.
    pub fn brep_mut(&mut self) -> &mut BrepPatch {
        &mut self.brep_patch
    }

    /// Destructure the draft into mutable borrows for leaf function calls.
    pub fn as_parts_mut(&mut self) -> (&mut MutableDraft, &mut GeometryPatch, &mut BrepPatch) {
        (&mut self.draft, &mut self.geom_patch, &mut self.brep_patch)
    }

    /// Discard all pending mutations and restore the pre-draft `KernelState`.
    ///
    /// Uses the original topology stored at construction time, guaranteeing
    /// that geometry and topology are always paired correctly.
    pub fn rollback(self) -> KernelState {
        KernelState::new(
            self.original_topo,
            self.geom_patch.rollback(),
            self.brep_patch.rollback(),
        )
    }

    /// Commit the transaction, finalizing all topology and geometry mutations.
    pub fn commit(self) -> Result<KernelState, KernelError> {
        let topo = self.draft.commit()?;
        Ok(KernelState::new(
            topo,
            self.geom_patch.commit(),
            self.brep_patch.commit(),
        ))
    }
}
