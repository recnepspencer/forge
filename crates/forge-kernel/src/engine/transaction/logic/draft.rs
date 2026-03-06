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
    ///
    /// Runs topology validation (via `MutableDraft::commit()`) then geometry
    /// validation (via `validate_all_spatial_invariants` through the dispatch
    /// system). If either fails, the `KernelState` is never constructed.
    pub fn commit(self) -> Result<KernelState, KernelError> {
        let topo = self.draft.commit()?;
        let geom = self.geom.commit();

        // ── Geometry validation ─────────────────────────────────────────
        // Mirrors topology validation in MutableDraft::commit(). Runs all
        // geometry-dependent invariants through the spatial dispatch system.
        let default_tol = forge_core::FlatToleranceProvider::new(1e-10);
        let ctx = forge_spatial::GeometryContext {
            position_fn: &|v| geom.positions.get(v).map(|p| *p.approx()),
            plane_fn: &|f| geom.planes.get(f).cloned(),
            is_planar: &|f| geom.surfaces.get(f).is_some(),
            curve_fn: &|e| geom.curves.get(e).map(|c| c.kind.clone()),
            tolerance_provider: &default_tol,
        };
        forge_spatial::validate_all_spatial_invariants(topo.arena(), &ctx)?;

        Ok(KernelState::new(topo, geom))
    }
}
