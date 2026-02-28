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

use crate::arena::TopologyArena;
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
use crate::hashing::compute_arena_topology_hash;
use crate::lineage::{Lineage, LineageEvent, OpSignature};
use crate::lineage_store::LineageStore;
use crate::replay::{ReplayEntry, ReplayLog};
use crate::topology::history::lineage_link::ReidentificationLinkIndex;
use crate::topology::validators::validate::ValidationLevel;
use forge_core::KernelError;

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
    /// Queryable one-hop lineage linkage index for persistent re-identification.
    ///
    /// Built at commit time from `lineage_events`. Snapshot-scoped + deterministic.
    #[serde(default)]
    reidentification_link_index: Arc<ReidentificationLinkIndex>,
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
            reidentification_link_index: Arc::new(ReidentificationLinkIndex::default()),
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

    /// One-hop re-identification linkage index built from committed lineage events.
    pub fn reidentification_link_index(&self) -> &ReidentificationLinkIndex {
        &self.reidentification_link_index
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

    /// Record a lineage event during mutation on the explicit/manual lineage channel.
    ///
    /// Forge currently has two lineage event sources in a draft:
    /// - `lineage_store`: live arena-driven provenance events emitted by topology mutations
    /// - `lineage_log`: explicit/manual events emitted by higher-level orchestration/tests
    ///
    /// Commit semantics must persist both channels into the committed chronology.
    /// This method exists for the explicit/manual channel.
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
        let entry = ReplayEntry::new(signature.clone(), Vec::new(), seed, self.topology_hash);
        self.replay_log.record(entry);
    }

    /// Apply lineage tracking for the completed operation (called by `apply_op` runner).
    ///
    /// Currently a stub — expanded in Milestone 1.2 (Euler Lineage Tracking).
    pub fn apply_lineage(&mut self, _signature: &OpSignature) {}

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

        crate::topology::validators::structural::validate_topology(&self.arena, self.config.validation_level)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        // Drain lineage sources and append to the prior history.
        //
        // `lineage_log` is the explicit/manual lineage channel used by some
        // callers and tests; `lineage_store` is the live arena-driven lineage
        // event source. Both are part of the committed chronology.
        let mut new_events = std::mem::take(&mut self.lineage_log);
        new_events.extend(self.lineage_store.drain_events());
        let mut all_events = std::mem::take(&mut self.prior_lineage_events);
        all_events.extend(new_events);
        let reid_index =
            ReidentificationLinkIndex::from_lineage_events(self.next_epoch, &all_events);

        Ok(TopologyState {
            epoch: self.next_epoch,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            topology_hash,
            arena: Arc::new(committed_arena),
            lineage_events: Arc::new(all_events),
            reidentification_link_index: Arc::new(reid_index),
        })
    }




    /// Compute the structural topology hash from the arena.
    pub(crate) fn compute_topology_hash(&self) -> u128 {
        compute_arena_topology_hash(&self.arena)
    }
    // ── Proxy CRUD Methods ───────────────────────────────────────────
    // Arena is pure storage. Lineage recording (Phase 3) will be added
    // as a separate step after the arena call returns the handle/data.

    pub fn insert_face(&mut self, data: crate::arena::FaceData) -> FaceId {
        self.arena.insert_face(data)
    }

    pub fn remove_face(&mut self, id: FaceId) -> Result<crate::arena::FaceData, KernelError> {
        self.arena.remove_face(id)
    }

    pub fn insert_half_edge(&mut self, data: crate::arena::HalfEdgeData) -> HalfEdgeId {
        self.arena.insert_half_edge(data)
    }

    pub fn insert_radial_pair(
        &mut self,
        data_a: crate::arena::HalfEdgeData,
        data_b: crate::arena::HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        self.arena.insert_radial_pair(data_a, data_b)
    }

    pub fn remove_half_edge(
        &mut self,
        id: HalfEdgeId,
    ) -> Result<crate::arena::HalfEdgeData, KernelError> {
        self.arena.remove_half_edge(id)
    }

    pub fn insert_vertex(&mut self, data: crate::arena::VertexData) -> VertexId {
        self.arena.insert_vertex(data)
    }

    pub fn remove_vertex(&mut self, id: VertexId) -> Result<crate::arena::VertexData, KernelError> {
        self.arena.remove_vertex(id)
    }

    pub fn insert_loop(&mut self, data: crate::arena::LoopData) -> LoopId {
        self.arena.insert_loop(data)
    }

    pub fn remove_loop(&mut self, id: LoopId) -> Result<crate::arena::LoopData, KernelError> {
        self.arena.remove_loop(id)
    }

    pub fn insert_shell(&mut self, data: crate::arena::ShellData) -> ShellId {
        self.arena.insert_shell(data)
    }

    pub fn remove_shell(&mut self, id: ShellId) -> Result<crate::arena::ShellData, KernelError> {
        self.arena.remove_shell(id)
    }

    pub fn insert_body(&mut self, data: crate::arena::BodyData) -> BodyId {
        self.arena.insert_body(data)
    }

    pub fn remove_body(&mut self, id: BodyId) -> Result<crate::arena::BodyData, KernelError> {
        self.arena.remove_body(id)
    }

    pub fn insert_lump(&mut self, data: crate::arena::LumpData) -> LumpId {
        self.arena.insert_lump(data)
    }

    pub fn remove_lump(&mut self, id: LumpId) -> Result<crate::arena::LumpData, KernelError> {
        self.arena.remove_lump(id)
    }

    pub fn insert_region(&mut self, data: crate::arena::RegionData) -> RegionId {
        self.arena.insert_region(data)
    }

    pub fn remove_region(&mut self, id: RegionId) -> Result<crate::arena::RegionData, KernelError> {
        self.arena.remove_region(id)
    }

    pub fn insert_edge(&mut self, data: crate::arena::EdgeData) -> EdgeId {
        self.arena.insert_edge(data)
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> Result<crate::arena::EdgeData, KernelError> {
        self.arena.remove_edge(id)
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
    use crate::lineage::LineageEntityRef;
    use forge_core::{EntityKind, EntityRef};
    use serde_json::Value;

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

        assert_eq!(
            first_mutation.topology_hash(),
            second_mutation.topology_hash()
        );
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

    #[test]
    fn commit_builds_reidentification_index_from_generational_lineage_events() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let lineage = Lineage::root(7, OpSignature::with_id("make_face", 1));
        draft.lineage_store.record_creation_with_snapshot(
            EntityRef::new(EntityKind::Face, 42),
            LineageEntityRef::new(EntityKind::Face, 42, 3),
            lineage.clone(),
        );

        let committed = draft.commit().unwrap();
        let hits = committed
            .reidentification_link_index()
            .find_by_child_hash(lineage.get_ancestry_hash(), Some(EntityKind::Face));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].child_snapshot.index, 42);
        assert_eq!(hits[0].child_snapshot.generation, 3);
    }

    #[test]
    fn topology_state_serde_round_trip_preserves_reidentification_index() {
        let committed = TopologyState {
            epoch: 1,
            topology_version: 1,
            geometry_version: 0,
            topology_hash: 0,
            arena: Arc::new(crate::arena::TopologyArena::new()),
            lineage_events: Arc::new(Vec::new()),
            reidentification_link_index: Arc::new(crate::topology::history::lineage_link::ReidentificationLinkIndex {
                schema_version: crate::topology::history::lineage_link::LinkSchemaVersion::V1,
                epoch: 1,
                records: vec![crate::topology::history::lineage_link::ReidentificationLinkRecord {
                    schema_version: crate::topology::history::lineage_link::LinkSchemaVersion::V1,
                    child_snapshot: crate::topology::history::lineage_link::TopoSnapshotHandleRef {
                        kind: EntityKind::Face,
                        index: 5,
                        generation: 2,
                    },
                    child_ancestry_hash: 42,
                    parent_ancestry_hashes: vec![7],
                    parent_linkage_mode: crate::lineage::ParentLinkageMode::Single,
                    parent_snapshot: None,
                    origin_kind: crate::topology::history::lineage_link::EntityOriginKind::EulerOperator,
                    creation_op_name: "make_face".to_string(),
                    creation_op_invocation: 1,
                    epoch: 1,
                    origin_features: vec![1],
                }],
            }),
        };

        let json = serde_json::to_value(&committed).unwrap();
        let decoded: TopologyState = serde_json::from_value(json).unwrap();
        let hits = decoded
            .reidentification_link_index()
            .find_by_child_hash(42, Some(EntityKind::Face));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].child_snapshot.generation, 2);
    }

    #[test]
    fn topology_state_legacy_deserialize_missing_reidentification_index_defaults_empty() {
        let state = TopologyState::empty();
        let mut json = serde_json::to_value(&state).unwrap();
        if let Value::Object(map) = &mut json {
            map.remove("reidentification_link_index");
        }
        let decoded: TopologyState = serde_json::from_value(json).unwrap();
        assert!(decoded.reidentification_link_index().records.is_empty());
    }

    #[test]
    fn commit_level_reidentification_index_order_is_deterministic_despite_event_insertion_order() {
        let make_state = |first: (u32, u32), second: (u32, u32)| {
            let state = TopologyState::empty();
            let mut draft = state.into_mutation();
            let root = Lineage::root(1, OpSignature::with_id("root", 1));
            let child = Lineage::derive(&root, OpSignature::with_id("derive", 2));
            draft.lineage_store.record_creation_with_snapshot(
                EntityRef::new(EntityKind::Face, first.0),
                LineageEntityRef::new(EntityKind::Face, first.0, first.1),
                child.clone(),
            );
            draft.lineage_store.record_creation_with_snapshot(
                EntityRef::new(EntityKind::Face, second.0),
                LineageEntityRef::new(EntityKind::Face, second.0, second.1),
                child.clone(),
            );
            (draft.commit().unwrap(), child.get_ancestry_hash())
        };

        let (a, hash_a) = make_state((20, 2), (10, 1));
        let (b, hash_b) = make_state((10, 1), (20, 2));
        assert_eq!(hash_a, hash_b);

        let seq_a: Vec<(u32, u32)> = a
            .reidentification_link_index()
            .find_by_child_hash(hash_a, Some(EntityKind::Face))
            .into_iter()
            .map(|r| (r.child_snapshot.index, r.child_snapshot.generation))
            .collect();
        let seq_b: Vec<(u32, u32)> = b
            .reidentification_link_index()
            .find_by_child_hash(hash_b, Some(EntityKind::Face))
            .into_iter()
            .map(|r| (r.child_snapshot.index, r.child_snapshot.generation))
            .collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn commit_persists_manual_lineage_log_events() {
        let mut draft = TopologyState::empty().into_mutation();
        let root = Lineage::root(1, OpSignature::with_id("root", 1));
        let child = Lineage::derive(&root, OpSignature::with_id("child", 2));
        draft.log_lineage_event(LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Face, 42),
            entity_snapshot: None,
            lineage: child.clone(),
        });

        let committed = draft.commit().expect("manual lineage log event commit");
        assert_eq!(committed.lineage_events().len(), 1);
        match &committed.lineage_events()[0] {
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } => {
                assert_eq!(entity.kind(), EntityKind::Face);
                assert_eq!(entity.index(), 42);
                assert!(entity_snapshot.is_none());
                assert_eq!(lineage.get_ancestry_hash(), child.get_ancestry_hash());
            }
            other => panic!("expected EntityCreated event, got {:?}", other),
        }
    }

    #[test]
    fn commit_persists_both_lineage_channels_deterministically() {
        let mut draft = TopologyState::empty().into_mutation();

        let manual_root = Lineage::root(10, OpSignature::with_id("manual_root", 1));
        let manual_child = Lineage::derive(&manual_root, OpSignature::with_id("manual_child", 2));
        draft.log_lineage_event(LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Face, 100),
            entity_snapshot: None,
            lineage: manual_child.clone(),
        });

        let store_root = Lineage::root(20, OpSignature::with_id("store_root", 1));
        let store_child = Lineage::derive(&store_root, OpSignature::with_id("store_child", 2));
        draft.lineage_store.record_creation_with_snapshot(
            EntityRef::new(EntityKind::Face, 200),
            LineageEntityRef::new(EntityKind::Face, 200, 7),
            store_child.clone(),
        );

        let committed = draft.commit().expect("mixed lineage channels commit");
        let events = committed.lineage_events();
        assert_eq!(events.len(), 2, "both lineage channels must be persisted");

        let mut manual_seen = false;
        let mut store_seen = false;
        for ev in events {
            match ev {
                LineageEvent::EntityCreated {
                    entity,
                    entity_snapshot,
                    lineage,
                } if lineage.get_ancestry_hash() == manual_child.get_ancestry_hash() => {
                    assert_eq!(entity.kind(), EntityKind::Face);
                    assert_eq!(entity.index(), 100);
                    assert!(
                        entity_snapshot.is_none(),
                        "manual channel event should remain explicit legacy/none"
                    );
                    manual_seen = true;
                }
                LineageEvent::EntityCreated {
                    entity,
                    entity_snapshot,
                    lineage,
                } if lineage.get_ancestry_hash() == store_child.get_ancestry_hash() => {
                    assert_eq!(entity.kind(), EntityKind::Face);
                    assert_eq!(entity.index(), 200);
                    assert_eq!(
                        *entity_snapshot,
                        Some(LineageEntityRef::new(EntityKind::Face, 200, 7)),
                        "lineage_store event must preserve generational snapshot"
                    );
                    store_seen = true;
                }
                _ => {}
            }
        }
        assert!(
            manual_seen && store_seen,
            "must persist one event from each lineage channel"
        );
    }
}
