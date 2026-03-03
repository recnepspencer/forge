//! RAII conditioning guard for pipeline coordinate restoration.
//!
//! DOMAIN: Ensures `OperationSpace::restore_store` runs on the output
//! geometry even if the pipeline encounters a panic during execution.
//!
//! Today, owned inputs are consumed/destroyed on error (safe), and
//! outputs only exist on success. But if partial-failure recovery is
//! ever added (e.g., `catch_unwind`), this guard prevents geometry
//! from being returned in local-space coordinates.
//!
//! INVARIANT: `restore_store()` is called exactly once — either
//! explicitly via `defuse()` or automatically on `Drop`.

use crate::engine::operation_space::operation_space::OperationSpace;
use crate::geometry::facade::GeometryStore;

/// RAII guard that ensures world-coordinate restoration runs.
///
/// Created by the pipeline after `execute_typed` succeeds. Holds a
/// reference to the `OperationSpace` and a mutable reference to the
/// output geometry. On `Drop`, calls `restore_store` if not already
/// restored via `defuse()`.
///
/// # Usage
///
/// ```ignore
/// let mut guard = ConditioningGuard::new(&op_space, envelope.geometry_mut());
/// // ... do post-execution work (hashing, finalization, invariants) ...
/// guard.defuse(); // explicit restore — preferred path
/// // if we panic before defuse(), Drop handles it
/// ```
pub struct ConditioningGuard<'a> {
    op_space: &'a OperationSpace,
    geometry: &'a mut GeometryStore,
    defused: bool,
}

impl<'a> ConditioningGuard<'a> {
    /// Create a guard that will restore world coordinates on drop.
    ///
    /// Only creates an active guard if the `OperationSpace` is active
    /// (i.e., a non-identity transform was applied). Returns `None` if
    /// no restoration is needed.
    pub fn new(op_space: &'a OperationSpace, geometry: &'a mut GeometryStore) -> Option<Self> {
        if op_space.is_active() {
            Some(Self {
                op_space,
                geometry,
                defused: false,
            })
        } else {
            None
        }
    }

    /// Explicitly restore world coordinates and mark the guard as defused.
    ///
    /// This is the preferred path — explicit > implicit. The `Drop` impl
    /// is a safety net, not the intended mechanism.
    pub fn defuse(mut self) {
        self.op_space.restore_store(self.geometry);
        self.defused = true;
    }
}

impl<'a> Drop for ConditioningGuard<'a> {
    fn drop(&mut self) {
        if !self.defused {
            self.op_space.restore_store(self.geometry);
        }
    }
}
