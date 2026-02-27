//! Lifecycle wrapper for single-solid kernel operations.
//!
//! DOMAIN: Manages the create → work → commit lifecycle of topology mutation.
//!
//! DEPENDENCIES: `forge-topo` (TopologyState, MutableDraft), GeometryState,
//! ModelingContext.
//!
//! INVARIANTS: Functions destructure via `as_parts_mut()` and pass individual
//! borrows to leaf functions — BRepWorkspace is NOT a parameter bag.

use forge_topo::state::{MutableDraft, TopologyState};

use super::ModelingContext;
use crate::brep::patch::BrepPatch;
use crate::core::{KernelDraft, KernelState};
use crate::geometry_state::{GeometryPatch, GeometryState};

/// Lifecycle wrapper for kernel operations that need draft + geometry + context.
///
/// Destructure via `as_parts_mut()` and pass individual borrows to leaf
/// functions. This avoids the borrow-checker conflict where bundling
/// everything prevents simultaneous `arena()` reads and `draft()` writes.
pub struct BRepWorkspace {
    draft: KernelDraft,
    ctx: ModelingContext,
}

impl BRepWorkspace {
    /// Create a workspace from an existing `KernelState`.
    pub fn new(state: KernelState, ctx: ModelingContext) -> Self {
        Self {
            draft: KernelDraft::new(state),
            ctx,
        }
    }

    /// Destructure for use — pass individual borrows to leaf functions.
    pub fn as_parts_mut(
        &mut self,
    ) -> (
        &mut MutableDraft,
        &mut GeometryPatch,
        &mut BrepPatch,
        &mut ModelingContext,
    ) {
        let (draft, geom, brep) = self.draft.as_parts_mut();
        (draft, geom, brep, &mut self.ctx)
    }

    /// Mut access to the draft.
    pub fn get_draft(&mut self) -> &mut MutableDraft {
        self.draft.draft_mut()
    }

    /// Read-only access to geometry.
    pub fn get_geometry(&self) -> &GeometryPatch {
        self.draft.geometry()
    }

    /// Read-only access to the modeling context.
    pub fn get_ctx(&self) -> &ModelingContext {
        &self.ctx
    }

    /// Read-only access to B-Rep data.
    pub fn get_brep(&self) -> &BrepPatch {
        self.draft.brep()
    }

    /// Finish: commit topology and return everything.
    pub fn commit(self) -> Result<(KernelState, ModelingContext), forge_core::KernelError> {
        let state = self.draft.commit()?;
        Ok((state, self.ctx))
    }
}
