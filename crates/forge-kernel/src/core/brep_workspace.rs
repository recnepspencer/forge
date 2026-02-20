//! Lifecycle wrapper for single-solid kernel operations.
//!
//! DOMAIN: Manages the create → work → commit lifecycle of topology mutation.
//!
//! DEPENDENCIES: `forge-topo` (TopologyState, MutableDraft), GeometryStore,
//! ModelingContext.
//!
//! INVARIANTS: Functions destructure via `as_parts_mut()` and pass individual
//! borrows to leaf functions — BRepWorkspace is NOT a parameter bag.

use forge_core::KernelError;
use forge_topo::state::{TopologyState, MutableDraft};

use crate::geometry_store::GeometryStore;
use super::ModelingContext;

/// Lifecycle wrapper for kernel operations that need draft + geometry + context.
///
/// Destructure via `as_parts_mut()` and pass individual borrows to leaf
/// functions. This avoids the borrow-checker conflict where bundling
/// everything prevents simultaneous `arena()` reads and `draft()` writes.
pub struct BRepWorkspace {
    draft: MutableDraft,
    geometry: GeometryStore,
    ctx: ModelingContext,
}

impl BRepWorkspace {
    /// Create a workspace from a committed topology state.
    pub fn new(topo: TopologyState, geometry: GeometryStore, ctx: ModelingContext) -> Self {
        Self { draft: topo.into_mutation(), geometry, ctx }
    }

    /// Destructure for use — pass individual borrows to leaf functions.
    pub fn as_parts_mut(&mut self) -> (&mut MutableDraft, &mut GeometryStore, &mut ModelingContext) {
        (&mut self.draft, &mut self.geometry, &mut self.ctx)
    }

    /// Read-only access to the draft.
    pub fn get_draft(&self) -> &MutableDraft {
        &self.draft
    }

    /// Read-only access to geometry.
    pub fn get_geometry(&self) -> &GeometryStore {
        &self.geometry
    }

    /// Read-only access to the modeling context.
    pub fn get_ctx(&self) -> &ModelingContext {
        &self.ctx
    }

    /// Finish: commit topology and return everything.
    pub fn commit(self) -> Result<(TopologyState, GeometryStore, ModelingContext), KernelError> {
        let topo = self.draft.commit()?;
        Ok((topo, self.geometry, self.ctx))
    }
}
