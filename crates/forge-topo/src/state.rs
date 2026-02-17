//! Epoch-versioned topology state with transactional mutation.
//!
//! # Architecture (Doctrine D6)
//!
//! `TopologyState` is immutable. The ONLY way to mutate topology is through
//! `MutableDraft`, which auto-rolls back if dropped without committing.
//!
//! The public API is functional:
//! ```ignore
//! fn my_operation(state: &TopologyState) -> Result<TopologyState, KernelError> {
//!     let mut draft = state.begin_mutation();
//!     // ... apply operations ...
//!     Ok(draft.commit()?)
//! }
//! ```
//!
//! This enables undo/redo in Phase 9: keep `Arc` references to old states.
//! "Undo" = point back to the previous state. No cloning needed.

use std::sync::Arc;

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::hashing::compute_arena_topology_hash;
use crate::lineage::{LineageEvent, OpSignature};
use crate::replay::{ReplayLog, ReplayEntry};
use crate::validate;

/// Immutable topology state with epoch versioning.
///
/// Every operation produces a NEW `TopologyState` — the old one is never
/// modified. This is the foundation for:
/// - **Undo/redo**: keep previous states as `Arc` references
/// - **Determinism** (D1): same input state + same ops = same output state
/// - **Transactionality** (D6): if an op fails, the old state is untouched
#[derive(Debug, Clone)]
pub struct TopologyState {
    /// Monotonically increasing epoch counter
    epoch: u64,
    /// Topology version (changes when connectivity changes)
    topology_version: u64,
    /// Geometry version (changes when positions change without topology change)
    geometry_version: u64,
    /// Structural hash of all topology (Merkle-style aggregate)
    topology_hash: u128,
    /// The topology arena holding all entity data (Milestone 0.5.1).
    /// Wrapped in `Arc` for cheap cloning and structural sharing.
    arena: Arc<TopologyArena>,
}

impl TopologyState {
    /// Create an empty topology state (the initial state before any geometry).
    pub fn empty() -> Self {
        Self {
            epoch: 0,
            topology_version: 0,
            geometry_version: 0,
            topology_hash: 0,
            arena: Arc::new(TopologyArena::new()),
        }
    }

    /// The current epoch (monotonically increasing version counter).
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// The topology version (bumped only when connectivity changes).
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// The geometry version (bumped when positions change).
    pub fn geometry_version(&self) -> u64 {
        self.geometry_version
    }

    /// Structural hash of the topology (for change detection and D1 verification).
    pub fn topology_hash(&self) -> u128 {
        self.topology_hash
    }

    /// Read-only access to the topology arena.
    pub fn arena(&self) -> &TopologyArena {
        &self.arena
    }

    /// The `Arc` reference to the arena (for snapshot / structural sharing).
    pub fn arena_arc(&self) -> &Arc<TopologyArena> {
        &self.arena
    }

    /// Begin a transactional mutation.
    ///
    /// Returns a `MutableDraft` — the ONLY way to change topology.
    /// If the draft is dropped without calling `.commit()`, all changes
    /// are discarded (Doctrine D6: atomic transactionality).
    ///
    /// # Example
    /// ```
    /// use forge_topo::state::TopologyState;
    ///
    /// let state = TopologyState::empty();
    /// let draft = state.begin_mutation();
    /// // ... apply Euler operators via apply_op() ...
    /// // draft.commit() to finalize, or just drop to rollback
    /// ```
    pub fn begin_mutation(&self) -> MutableDraft {
        MutableDraft {
            base_epoch: self.epoch,
            next_epoch: self.epoch + 1,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            lineage_log: Vec::new(),
            op_counter: 0,
            committed: false,
            replay_log: ReplayLog::new(),
            topology_hash: self.topology_hash,
            arena: (*self.arena).clone(),
        }
    }
}

/// A mutable draft for topology changes (Doctrine D6).
///
/// # Transactional Safety
///
/// - **Commit**: Call `.commit()` to finalize changes → returns new `TopologyState`
/// - **Rollback**: Drop without committing → all changes are silently discarded
/// - **Auto-validation**: `.commit()` runs topology validation before returning
///
/// This mirrors your Angular `createOptimisticMutation` pattern:
/// try the operation, rollback on failure, commit on success.
pub struct MutableDraft {
    /// The epoch of the state we forked from
    base_epoch: u64,
    /// The epoch this draft will produce if committed
    next_epoch: u64,
    /// Current topology version (may be bumped during mutations)
    topology_version: u64,
    /// Current geometry version (may be bumped during mutations)
    geometry_version: u64,
    /// Lineage events recorded during this draft
    lineage_log: Vec<LineageEvent>,
    /// Counter for assigning unique operation IDs within this draft
    op_counter: u64,
    /// Whether commit() was called
    committed: bool,
    /// Replay log for this draft (Milestone 0.4)
    replay_log: ReplayLog,
    /// Current topology hash (for replay entry pre-hashes)
    topology_hash: u128,
    /// The mutable arena — cloned from the source state's Arc on begin_mutation
    arena: TopologyArena,
}

impl MutableDraft {
    /// Record a lineage event during mutation.
    pub fn log_lineage_event(&mut self, event: LineageEvent) {
        self.lineage_log.push(event);
    }

    /// Get the next unique operation ID for this draft.
    pub fn next_op_id(&mut self) -> u64 {
        self.op_counter += 1;
        self.op_counter
    }

    /// Log the start of an operation (called by `apply_op` runner).
    pub fn log_operation_start(&mut self, signature: &OpSignature) {
        let entry = ReplayEntry::new(
            signature.clone(),
            String::new(),
            0,
            self.topology_hash,
        );
        self.replay_log.record(entry);
    }

    /// Apply lineage tracking for the completed operation (called by `apply_op` runner).
    ///
    /// Currently a placeholder — expanded in Milestone 1.2 (Euler Lineage Tracking).
    pub fn apply_lineage(&mut self, _signature: &OpSignature) {
    }

    /// Bump the topology version (call after connectivity changes).
    pub fn bump_topology_version(&mut self) {
        self.topology_version += 1;
    }

    /// Bump the geometry version (call after position-only changes).
    pub fn bump_geometry_version(&mut self) {
        self.geometry_version += 1;
    }

    /// The lineage events recorded during this draft.
    pub fn lineage_log(&self) -> &[LineageEvent] {
        &self.lineage_log
    }

    /// The replay log recorded during this draft (Milestone 0.4).
    pub fn replay_log(&self) -> &ReplayLog {
        &self.replay_log
    }

    /// Read-only access to the draft's arena.
    pub fn arena(&self) -> &TopologyArena {
        &self.arena
    }

    /// Mutable access to the draft's arena (for Euler operators).
    pub fn arena_mut(&mut self) -> &mut TopologyArena {
        &mut self.arena
    }

    /// Finalize the mutation, producing a new `TopologyState`.
    ///
    /// This automatically runs topology validation (D4, D6).
    /// If validation fails, returns `KernelError::TopologyViolation`.
    ///
    /// # Errors
    ///
    /// Returns `KernelError::TopologyViolation` if the resulting topology
    /// violates any invariant (Euler formula, twin consistency, etc.).
    pub fn commit(mut self) -> Result<TopologyState, KernelError> {
        self.committed = true;

        validate::validate_topology(&self.arena)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        Ok(TopologyState {
            epoch: self.next_epoch,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            topology_hash,
            arena: Arc::new(committed_arena),
        })
    }

    /// Compute the structural topology hash from the arena.
    fn compute_topology_hash(&self) -> u128 {
        compute_arena_topology_hash(&self.arena)
    }
}

impl Drop for MutableDraft {
    fn drop(&mut self) {
        if !self.committed {
            tracing::warn!(
                base_epoch = self.base_epoch,
                ops_applied = self.op_counter,
                "MutableDraft dropped without commit. Topology rolled back."
            );
        }
    }
}

/// Manual `Debug` impl — `MutableDraft` is not `Clone` (forking a transaction is invalid).
impl std::fmt::Debug for MutableDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutableDraft")
            .field("base_epoch", &self.base_epoch)
            .field("next_epoch", &self.next_epoch)
            .field("committed", &self.committed)
            .field("ops_applied", &self.op_counter)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_has_epoch_zero() {
        let state = TopologyState::empty();
        assert_eq!(state.epoch(), 0);
        assert_eq!(state.topology_version(), 0);
        assert_eq!(state.geometry_version(), 0);
    }

    #[test]
    fn commit_increments_epoch() {
        let state = TopologyState::empty();
        let draft = state.begin_mutation();
        let new_state = draft.commit().unwrap();
        assert_eq!(new_state.epoch(), 1);
    }

    #[test]
    fn drop_without_commit_is_safe() {
        let state = TopologyState::empty();
        {
            let _draft_dropped_without_commit = state.begin_mutation();
        }
        assert_eq!(state.epoch(), 0);
    }

    #[test]
    fn original_state_unchanged_after_mutation() {
        let original = TopologyState::empty();
        let draft = original.begin_mutation();
        let mutated = draft.commit().unwrap();

        assert_eq!(original.epoch(), 0);
        assert_eq!(mutated.epoch(), 1);
    }

    #[test]
    fn sequential_mutations_produce_increasing_epochs() {
        let s0 = TopologyState::empty();
        let s1 = s0.begin_mutation().commit().unwrap();
        let s2 = s1.begin_mutation().commit().unwrap();
        let s3 = s2.begin_mutation().commit().unwrap();

        assert_eq!(s0.epoch(), 0);
        assert_eq!(s1.epoch(), 1);
        assert_eq!(s2.epoch(), 2);
        assert_eq!(s3.epoch(), 3);
    }

    #[test]
    fn topology_hash_is_deterministic() {
        let state = TopologyState::empty();

        let first_mutation = state.begin_mutation().commit().unwrap();
        let second_mutation = state.begin_mutation().commit().unwrap();

        assert_eq!(first_mutation.topology_hash(), second_mutation.topology_hash());
    }

    #[test]
    fn geometry_only_commit_preserves_topology_hash() {
        let state = TopologyState::empty();

        let mut draft_topo = state.begin_mutation();
        draft_topo.bump_topology_version();
        let after_topo = draft_topo.commit().unwrap();

        let mut draft_geom = after_topo.begin_mutation();
        draft_geom.bump_geometry_version();
        let after_geom = draft_geom.commit().unwrap();

        assert_eq!(after_topo.topology_hash(), after_geom.topology_hash());
    }
}
