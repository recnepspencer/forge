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

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use crate::arena::{TopologyArena};
use crate::hashing::compute_arena_topology_hash;
use crate::lineage::{LineageEvent, OpSignature};
use crate::lineage_store::LineageStore;
use crate::replay::{ReplayLog, ReplayEntry};
use crate::handles::{FaceId, VertexId, HalfEdgeId, LoopId, ShellId, BodyId, LumpId, RegionId, EdgeId};
use crate::validate::{self, ValidationLevel};

/// Configuration for a mutable draft transaction.
///
/// Controls opt-in features like per-operation structural hashing
/// and deterministic seeding for reproducible operation sequences.
#[derive(Debug, Clone)]
pub struct DraftConfig {
    /// When true, compute and record the arena's structural signature
    /// after every Euler operation. Enables full replay hash trails
    /// at the cost of O(N) per operation.
    ///
    /// Default: `false` (hash is only computed once at commit time).
    pub per_op_hashing: bool,
    /// Base seed for deterministic RNG during this draft.
    ///
    /// Each operation receives `deterministic_seed + op_counter` as its
    /// entry seed in the replay log, producing unique reproducible seeds.
    ///
    /// Default: `0` (no external seed).
    pub deterministic_seed: u64,
    /// Strictness of topology validation at commit time.
    ///
    /// Default: `Full` in Debug, `Minimal` in Release.
    pub validation_level: ValidationLevel,
    /// When true, verify twin/next/prev consistency after every Euler op.
    ///
    /// Expensive — use only in dev/CI. Default: `false`.
    pub per_op_validation: bool,
}

impl Default for DraftConfig {
    fn default() -> Self {
        Self {
            per_op_hashing: false,
            deterministic_seed: 0,
            validation_level: ValidationLevel::default(),
            per_op_validation: false,
        }
    }
}

/// Immutable topology state with epoch versioning.
///
/// Every operation produces a NEW `TopologyState` — the old one is never
/// modified. This is the foundation for:
/// - **Undo/redo**: keep previous states as `Arc` references
/// - **Determinism** (D1): same input state + same ops = same output state
/// - **Transactionality** (D6): if an op fails, the old state is untouched
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Chronological log of lineage events that produced this state.
    ///
    /// Accumulated across epochs: each `commit()` appends the draft's events
    /// to the prior state's history so the full provenance chain survives.
    lineage_events: Arc<Vec<LineageEvent>>,
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
            lineage_events: Arc::new(Vec::new()),
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

    /// The chronological lineage event log accumulated across all epochs.
    pub fn lineage_events(&self) -> &[LineageEvent] {
        &self.lineage_events
    }

    /// Begin a transactional mutation by consuming the state (Zero-Cost).
    ///
    /// If the state holds the unique reference to the arena, reuses the allocation (O(1)).
    /// Otherwise, clones the arena (O(N)).
    ///
    /// # Example
    /// ```
    /// use forge_topo::state::TopologyState;
    ///
    /// let state = TopologyState::empty();
    /// let draft = state.into_mutation();
    /// // ... apply Euler operators ...
    /// // draft.commit() returns a new TopologyState
    /// ```
    pub fn into_mutation(self) -> MutableDraft {
        self.into_mutation_with(DraftConfig::default())
    }

    /// Begin a transactional mutation with explicit configuration.
    pub fn into_mutation_with(self, config: DraftConfig) -> MutableDraft {
        // CONSUME-ON-WRITE:
        // Try to unwrap the Arc. If we are the only owner, we get the Arena for free (O(1)).
        // If shared, we must clone (O(N)).
        let arena = match Arc::try_unwrap(self.arena) {
            Ok(arena) => arena,
            Err(arc) => (*arc).clone(),
        };

        // Carry forward the prior lineage history so new events append to it.
        let prior_events = match Arc::try_unwrap(self.lineage_events) {
            Ok(events) => events,
            Err(arc) => (*arc).clone(),
        };

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
            config,
            arena,
            lineage_store: LineageStore::new(),
            prior_lineage_events: prior_events,
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
    /// Draft configuration (per-op hashing, deterministic seed)
    config: DraftConfig,
    /// The mutable arena — cloned from the source state's Arc on begin_mutation
    arena: TopologyArena,
    /// Live lineage store tracking all entity provenance during this draft.
    lineage_store: LineageStore,
    /// Lineage events inherited from the prior committed state.
    prior_lineage_events: Vec<LineageEvent>,
}

impl MutableDraft {
    /// The current topology hash of this draft.
    pub fn topology_hash(&self) -> u128 {
        self.topology_hash
    }

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
    ///
    /// Records the current topology hash as `pre_hash` and computes a
    /// deterministic seed from the config's base seed + op counter.
    pub fn log_operation_start(&mut self, signature: &OpSignature) {
        let seed = self.config.deterministic_seed.wrapping_add(self.op_counter);
        let entry = ReplayEntry::new(
            signature.clone(),
            Vec::new(),
            seed,
            self.topology_hash,
        );
        self.replay_log.record(entry);
    }

    /// Apply lineage tracking for the completed operation (called by `apply_op` runner).
    ///
    /// Currently a placeholder — expanded in Milestone 1.2 (Euler Lineage Tracking).
    pub fn apply_lineage(&mut self, _signature: &OpSignature) {
    }

    /// The draft's configuration.
    pub fn config(&self) -> &DraftConfig {
        &self.config
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

    /// Mutable access to the replay log.
    pub fn replay_log_mut(&mut self) -> &mut ReplayLog {
        &mut self.replay_log
    }

    /// Set the current topology hash.
    pub(crate) fn set_topology_hash(&mut self, hash: u128) {
        self.topology_hash = hash;
    }

    /// Read-only access to the draft's arena.
    pub fn arena(&self) -> &TopologyArena {
        &self.arena
    }

    /// Mutable access to the draft's arena (for Euler operators).
    pub fn arena_mut(&mut self) -> &mut TopologyArena {
        &mut self.arena
    }

    /// Read-only access to the lineage store.
    pub fn lineage_store(&self) -> &LineageStore {
        &self.lineage_store
    }

    /// Mutable access to the lineage store.
    pub fn lineage_store_mut(&mut self) -> &mut LineageStore {
        &mut self.lineage_store
    }

    /// Disjoint mutable access to both the arena and the lineage store.
    ///
    /// Essential for Euler operators to pass the lineage store to arena hooks
    /// without violating borrow checker rules.
    pub fn unbundle_mut(&mut self) -> (&mut TopologyArena, &mut LineageStore) {
        (&mut self.arena, &mut self.lineage_store)
    }

    /// Take ownership of the lineage store, replacing it with an empty one.
    ///
    /// Use this to extract lineage data before commit (or on error paths
    /// when the draft will be dropped without committing).
    pub fn take_lineage_store(&mut self) -> LineageStore {
        std::mem::take(&mut self.lineage_store)
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

        validate::validate_topology(&self.arena, self.config.validation_level)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        // Drain the lineage store and append to the prior history.
        let new_events = self.lineage_store.drain_events();
        let mut all_events = std::mem::take(&mut self.prior_lineage_events);
        all_events.extend(new_events);

        Ok(TopologyState {
            epoch: self.next_epoch,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            topology_hash,
            arena: Arc::new(committed_arena),
            lineage_events: Arc::new(all_events),
        })
    }

    /// Commit with an explicit topology manifold policy.
    ///
    /// `level` overrides the draft's configured `ValidationLevel`.
    /// `mode` controls what topology is semantically permitted at commit time:
    ///   - `ManifoldStrict` (default): rejects valence > 2 edges.
    ///   - `NmtIntermediate`: permits valence > 2 for internal pipeline checkpoints.
    ///
    /// Default `commit()` always uses `ManifoldStrict`. This cannot be silently changed.
    /// Callers requiring NMT semantics must use this method explicitly.
    pub fn commit_with_mode(
        mut self,
        level: validate::ValidationLevel,
        mode: validate::TopologyMode,
    ) -> Result<TopologyState, KernelError> {
        self.committed = true;

        validate::validate_topology_with_mode(&self.arena, level, mode)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        let new_events = self.lineage_store.drain_events();
        let mut all_events = std::mem::take(&mut self.prior_lineage_events);
        all_events.extend(new_events);

        Ok(TopologyState {
            epoch: self.next_epoch,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            topology_hash,
            arena: Arc::new(committed_arena),
            lineage_events: Arc::new(all_events),
        })
    }

    /// Compute the structural topology hash from the arena.
    pub(crate) fn compute_topology_hash(&self) -> u128 {
        compute_arena_topology_hash(&self.arena)
    }
    // ── Proxy CRUD Methods (Option B Lineage Hooks) ────────────────

    pub fn insert_face(&mut self, data: crate::arena::FaceData) -> FaceId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_face(data, Some(store))
    }

    pub fn remove_face(&mut self, id: FaceId) -> Result<crate::arena::FaceData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_face(id, Some(store))
    }

    pub fn insert_half_edge(&mut self, data: crate::arena::HalfEdgeData) -> HalfEdgeId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_half_edge(data, Some(store))
    }

    pub fn insert_radial_pair(&mut self, data_a: crate::arena::HalfEdgeData, data_b: crate::arena::HalfEdgeData) -> (HalfEdgeId, HalfEdgeId) {
        let (arena, store) = self.unbundle_mut();
        arena.insert_radial_pair(data_a, data_b, Some(store))
    }

    pub fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<crate::arena::HalfEdgeData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_half_edge(id, Some(store))
    }

    pub fn insert_vertex(&mut self, data: crate::arena::VertexData) -> VertexId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_vertex(data, Some(store))
    }

    pub fn remove_vertex(&mut self, id: VertexId) -> Result<crate::arena::VertexData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_vertex(id, Some(store))
    }

    pub fn insert_loop(&mut self, data: crate::arena::LoopData) -> LoopId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_loop(data, Some(store))
    }

    pub fn remove_loop(&mut self, id: LoopId) -> Result<crate::arena::LoopData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_loop(id, Some(store))
    }

    pub fn insert_shell(&mut self, data: crate::arena::ShellData) -> ShellId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_shell(data, Some(store))
    }

    pub fn remove_shell(&mut self, id: ShellId) -> Result<crate::arena::ShellData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_shell(id, Some(store))
    }

    pub fn insert_body(&mut self, data: crate::arena::BodyData) -> BodyId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_body(data, Some(store))
    }

    pub fn remove_body(&mut self, id: BodyId) -> Result<crate::arena::BodyData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_body(id, Some(store))
    }

    pub fn insert_lump(&mut self, data: crate::arena::LumpData) -> LumpId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_lump(data, Some(store))
    }

    pub fn remove_lump(&mut self, id: LumpId) -> Result<crate::arena::LumpData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_lump(id, Some(store))
    }

    pub fn insert_region(&mut self, data: crate::arena::RegionData) -> RegionId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_region(data, Some(store))
    }

    pub fn remove_region(&mut self, id: RegionId) -> Result<crate::arena::RegionData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_region(id, Some(store))
    }

    pub fn insert_edge(&mut self, data: crate::arena::EdgeData) -> EdgeId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_edge(data, Some(store))
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> Result<crate::arena::EdgeData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_edge(id, Some(store))
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
        let draft = state.into_mutation();
        let new_state = draft.commit().unwrap();
        assert_eq!(new_state.epoch(), 1);
    }

    #[test]
    fn drop_without_commit_is_safe() {
        let state = TopologyState::empty();
        {
            let _draft_dropped_without_commit = state.clone().into_mutation();
        }
        assert_eq!(state.epoch(), 0);
    }

    #[test]
    fn original_state_unchanged_after_mutation() {
        let original = TopologyState::empty();
        let draft = original.clone().into_mutation();
        let mutated = draft.commit().unwrap();

        assert_eq!(original.epoch(), 0);
        assert_eq!(mutated.epoch(), 1);
    }

    #[test]
    fn sequential_mutations_produce_increasing_epochs() {
        let s0 = TopologyState::empty();
        let s1 = s0.clone().into_mutation().commit().unwrap();
        let s2 = s1.clone().into_mutation().commit().unwrap();
        let s3 = s2.clone().into_mutation().commit().unwrap();

        assert_eq!(s0.epoch(), 0);
        assert_eq!(s1.epoch(), 1);
        assert_eq!(s2.epoch(), 2);
        assert_eq!(s3.epoch(), 3);
    }

    #[test]
    fn topology_hash_is_deterministic() {
        let state = TopologyState::empty();

        let first_mutation = state.clone().into_mutation().commit().unwrap();
        let second_mutation = state.into_mutation().commit().unwrap();

        assert_eq!(first_mutation.topology_hash(), second_mutation.topology_hash());
    }

    #[test]
    fn geometry_only_commit_preserves_topology_hash() {
        let state = TopologyState::empty();

        let mut draft_topo = state.into_mutation();
        draft_topo.bump_topology_version();
        let after_topo = draft_topo.commit().unwrap();

        let mut draft_geom = after_topo.clone().into_mutation();
        draft_geom.bump_geometry_version();
        let after_geom = draft_geom.commit().unwrap();

        assert_eq!(after_topo.topology_hash(), after_geom.topology_hash());
    }
}
