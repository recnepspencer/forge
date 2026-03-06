//! Lifecycle wrapper for single-solid kernel operations.
//!
//! DOMAIN: Manages the create → work → commit lifecycle of topology mutation.
//!
//! DEPENDENCIES: `forge-topo` (TopologyState, MutableDraft), GeometryStore,
//! ResolvedConfig.
//!
//! INVARIANTS: Functions destructure via `as_parts_mut()` and pass individual
//! borrows to leaf functions — BRepWorkspace is NOT a parameter bag.

use forge_topo::transactions::MutableDraft;

use crate::configuration::facade::ResolvedConfig;
use crate::geometry::facade::GeometryDraft;

use super::super::data::state::KernelState;
use super::draft::KernelDraft;

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
    pub fn as_parts_mut(&mut self) -> (&mut MutableDraft, &mut GeometryDraft, &ResolvedConfig) {
        let (draft, geom) = self.draft.as_parts_mut();
        (draft, geom, &self.config)
    }

    /// Mut access to the draft.
    pub fn get_draft(&mut self) -> &mut MutableDraft {
        self.draft.draft_mut()
    }

    /// Read-only access to geometry.
    pub fn get_geometry(&self) -> &GeometryDraft {
        self.draft.geometry()
    }

    /// Read-only access to the resolved config.
    pub fn get_config(&self) -> &ResolvedConfig {
        &self.config
    }

    /// Finish: commit topology and return everything.
    pub fn commit(self) -> Result<(KernelState, ResolvedConfig), forge_core::KernelError> {
        let state = self.draft.commit()?;
        Ok((state, self.config))
    }
}
