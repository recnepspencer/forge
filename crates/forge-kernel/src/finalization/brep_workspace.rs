//! Lifecycle wrapper for single-solid kernel operations.
//!
//! DOMAIN: Manages the create → work → commit lifecycle of topology mutation.
//!
//! DEPENDENCIES: `forge-topo` (TopologyState, MutableDraft), GeometryState,
//! ResolvedConfig.
//!
//! INVARIANTS: Functions destructure via `as_parts_mut()` and pass individual
//! borrows to leaf functions — BRepWorkspace is NOT a parameter bag.

use forge_topo::transactions::MutableDraft;

use crate::brep::patch::BrepPatch;
use crate::configuration::facade::ResolvedConfig;
use crate::geometry_state::GeometryPatch;

use super::{kernel_draft::KernelDraft, kernel_state::KernelState};

/// Lifecycle wrapper for kernel operations that need draft + geometry + context.
///
/// Destructure via `as_parts_mut()` and pass individual borrows to leaf
/// functions. This avoids the borrow-checker conflict where bundling
/// everything prevents simultaneous `arena()` reads and `draft()` writes.
pub struct BRepWorkspace {
    draft: KernelDraft,
    config: ResolvedConfig,
}

impl BRepWorkspace {
    /// Create a workspace from an existing `KernelState`.
    pub fn new(state: KernelState, config: ResolvedConfig) -> Self {
        Self {
            draft: KernelDraft::new(state),
            config,
        }
    }

    /// Destructure for use — pass individual borrows to leaf functions.
    pub fn as_parts_mut(
        &mut self,
    ) -> (
        &mut MutableDraft,
        &mut GeometryPatch,
        &mut BrepPatch,
        &ResolvedConfig,
    ) {
        let (draft, geom, brep) = self.draft.as_parts_mut();
        (draft, geom, brep, &self.config)
    }

    /// Mut access to the draft.
    pub fn get_draft(&mut self) -> &mut MutableDraft {
        self.draft.draft_mut()
    }

    /// Read-only access to geometry.
    pub fn get_geometry(&self) -> &GeometryPatch {
        self.draft.geometry()
    }

    /// Read-only access to the resolved config.
    pub fn get_config(&self) -> &ResolvedConfig {
        &self.config
    }

    /// Read-only access to B-Rep data.
    pub fn get_brep(&self) -> &BrepPatch {
        self.draft.brep()
    }

    /// Finish: commit topology and return everything.
    pub fn commit(self) -> Result<(KernelState, ResolvedConfig), forge_core::KernelError> {
        let state = self.draft.commit()?;
        Ok((state, self.config))
    }
}
