//! Mutable draft for transactional topology mutation (Doctrine D6).
//!
//! DOMAIN: MutableDraft wraps the arena for copy-on-write mutation.
//! Commit finalizes changes into a new TopologyState; drop rolls back.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::b_rep::TopologyArena;
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
use crate::identity::{DraftId, OperationCount, OperationId};
use crate::operations::operator::TopoOperator;
use crate::provenance::LineageStore;
use crate::provenance::ReidentificationLinkIndex;
use crate::provenance::{Lineage, LineageEvent, OpSignature};
use crate::provenance::{LineageMode, LineageRecorder, OperationLineageContext, FEATURE_ID_SYSTEM};
use crate::provenance::{ReplayEntry, ReplayLog};
use crate::transactions::compute_arena_topology_hash;
use crate::transactions::data::mutation_journal::MutationJournal;
use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::validators::validate::ValidationLevel;

use forge_core::{
    ErrorContext, ErrorScope, KernelError, LineageDelta, OperationMetrics, OperationResult,
    TopologyError,
};
use forge_signal::facade::{CheckpointBarrier, EventBus};

use crate::transactions::data::draft_configuration::DraftConfig;
use crate::transactions::data::versioned_snapshot::TopologyState;

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
    /// Unique identity of this draft transaction.
    pub(crate) draft_id: DraftId,
    /// The epoch of the state we forked from
    pub(crate) base_epoch: u64,
    /// The epoch this draft will produce if committed
    pub(crate) next_epoch: u64,
    /// Current topology version (may be bumped during mutations)
    pub(crate) topology_version: u64,
    /// Current geometry version (may be bumped during mutations)
    pub(crate) geometry_version: u64,
    /// Counter for assigning unique operation IDs within this draft
    pub(crate) op_counter: OperationId,
    /// Whether commit() was called
    pub(crate) committed: bool,
    /// Replay log for this draft (Milestone 0.4)
    pub(crate) replay_log: ReplayLog,
    /// Current topology hash (for replay entry pre-hashes)
    pub(crate) topology_hash: u128,
    /// Draft configuration (per-op hashing, deterministic seed)
    pub(crate) config: DraftConfig,
    /// The mutable arena — cloned from the source state's Arc on begin_mutation
    pub(crate) arena: TopologyArena,
    /// Live lineage store tracking all entity provenance during this draft.
    pub(crate) lineage_store: LineageStore,
    /// Lineage events inherited from the prior committed state.
    pub(crate) prior_lineage_events: Vec<LineageEvent>,
    /// Per-operation mutation journal — records every insert/remove automatically.
    pub(crate) mutation_journal: MutationJournal,
    /// If true, a previous operation failed and this draft MUST NOT be used.
    pub(crate) poisoned: bool,
    /// True once rollback callbacks have been executed for this draft lifecycle.
    pub(crate) rollback_applied: bool,
    /// Signal-unified operation lifecycle runtime (subscriber wiring lands incrementally).
    pub(crate) event_bus: EventBus<TopoOperationEvent, TopoSubscriberDataId, MutableDraft>,
    /// Buffered per-operation events emitted by proxies/chokepoints.
    ///
    /// Runner drains this into the event bus at checkpoint boundaries.
    pub(crate) pending_operation_events: Vec<TopoOperationEvent>,
}

impl MutableDraft {
    /// The current topology hash of this draft.
    pub fn topology_hash(&self) -> u128 {
        self.topology_hash
    }

    /// Unique identity of this draft transaction.
    pub fn draft_id(&self) -> DraftId {
        self.draft_id
    }

    /// Get the next unique operation ID for this draft.
    pub fn next_op_id(&mut self) -> OperationId {
        let next = self.op_counter.get() + 1;
        let op_id = OperationId::new(next);
        self.op_counter = op_id;
        op_id
    }

    /// Execute rollback callbacks exactly once for this draft.
    pub(crate) fn apply_rollback_once(&mut self) {
        if self.rollback_applied {
            return;
        }
        self.pending_operation_events.clear();
        self.emit_operation_event(TopoOperationEvent::DraftRolledBack {
            draft_id: self.draft_id,
            ops_completed: OperationCount::new(self.op_counter.get()),
        });
        let mut event_bus = std::mem::take(&mut self.event_bus);
        self.drain_pending_events_into(&mut event_bus);
        event_bus.rollback(self);
        self.event_bus = event_bus;
        self.rollback_applied = true;
    }

    /// Log the start of an operation (called by `execute()` method).
    ///
    /// Records the current topology hash as `pre_hash` and computes a
    /// deterministic seed from the config's base seed + op counter.
    pub fn log_operation_start(
        &mut self,
        signature: &OpSignature,
        op_schema_version: u32,
        semantic_summary: String,
    ) {
        let op_id = signature.get_invocation_id();
        let seed = self.config.deterministic_seed.wrapping_add(op_id.get());
        let entry = ReplayEntry::new(
            self.draft_id,
            op_id,
            signature.clone(),
            Vec::new(),
            op_schema_version,
            1,
            seed,
            self.topology_hash,
            semantic_summary,
        );
        self.replay_log.record(entry);
    }

    /// Debug-only guard: assert that every entity in the arena has lineage.
    ///
    /// Fires at the end of `execute()` to catch missing stamps immediately
    /// rather than six operations later.
    #[cfg(debug_assertions)]
    pub fn validate_lineage_coverage(&self, signature: &OpSignature) {
        let arena_count = self.arena.face_count()
            + self.arena.vertex_count()
            + self.arena.half_edge_count()
            + self.arena.edge_count()
            + self.arena.loop_count()
            + self.arena.shell_count()
            + self.arena.body_count()
            + self.arena.lump_count()
            + self.arena.region_count();
        let lineage_count = self.lineage_store.active_count();
        // Only assert if lineage store is non-empty (i.e., lineage wiring is active).
        // This avoids false positives during the migration period where not all
        // code paths stamp lineage yet.
        if lineage_count > 0 {
            debug_assert_eq!(
                arena_count, lineage_count,
                "Lineage coverage gap after {}: arena has {} entities, lineage tracks {}",
                signature, arena_count, lineage_count
            );
        }
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

    // ── Provenance Stamping API ─────────────────────────────────────────
    // See `provenance_stamping.rs` for `stamp_children_of()` and
    // `stamp_merged_children_of()`.

    /// Read-only access to the mutation journal for the current operation.
    pub fn mutation_journal(&self) -> &MutationJournal {
        &self.mutation_journal
    }

    /// Mutable event bus access for subscriber registration/tests.
    pub fn event_bus_mut(
        &mut self,
    ) -> &mut EventBus<TopoOperationEvent, TopoSubscriberDataId, MutableDraft> {
        &mut self.event_bus
    }

    /// Queue one lifecycle event for the current operation.
    pub(crate) fn emit_operation_event(&mut self, event: TopoOperationEvent) {
        self.pending_operation_events.push(event);
    }

    /// Drain buffered operation events into an active event bus.
    pub(crate) fn drain_pending_events_into(
        &mut self,
        event_bus: &mut EventBus<TopoOperationEvent, TopoSubscriberDataId, MutableDraft>,
    ) {
        for event in std::mem::take(&mut self.pending_operation_events) {
            event_bus.emit(event);
        }
    }

    /// Mutable access to the mutation journal (for testing and runner internals).
    pub(crate) fn mutation_journal_mut(&mut self) -> &mut MutationJournal {
        &mut self.mutation_journal
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
        if self.poisoned {
            return Err(KernelError::InternalError {
                message:
                    "Cannot commit a poisoned draft. A previous operation failed mid-transaction."
                        .to_string(),
                context: None,
            });
        }
        self.committed = true;

        // ── Debug: verify side-car vectors haven't drifted from slot vectors ──
        #[cfg(debug_assertions)]
        self.arena.assert_sidecar_parity();

        // Enforce checkpoint progression at commit-time: direct mutation flows
        // (that do not call `draft.execute`) still need PerOperation domains
        // flushed before strict PerCommit freshness checks run.
        self.arena
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)?;
        self.arena
            .apply_cache_checkpoint(CheckpointBarrier::PerCommit)?;
        let mut event_bus = std::mem::take(&mut self.event_bus);
        self.drain_pending_events_into(&mut event_bus);
        event_bus
            .flush(CheckpointBarrier::PerCommit, &mut self)
            .map_err(|e| KernelError::InternalError {
                message: format!("Event bus commit checkpoint failed: {e}"),
                context: None,
            })?;
        self.event_bus = event_bus;

        crate::validators::structural::validate_topology(
            &self.arena,
            self.config.validation_level,
        )?;

        // ── Debug: verify declared ShellKind matches structural reality ──
        // Catches operators that change topology character without updating
        // ShellKind (e.g., sealing a Sheet into a Solid).
        #[cfg(debug_assertions)]
        crate::validators::group_policy_runtime::verify_shell_kind_matches_structure(&self.arena);

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        // Drain lineage from the single source of truth (LineageStore)
        // and append to the prior history.
        let new_events = self.lineage_store.drain_events();
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
    // Generated by `define_draft_proxies!` macro in `arena/crud_macro.rs`.
    // insert_*/remove_* for all 9 entity types are auto-generated there.

    /// Insert a pair of radial halfedges and wire their `radial_next` fields.
    pub fn insert_radial_pair(
        &mut self,
        data_a: crate::b_rep::HalfEdgeData,
        data_b: crate::b_rep::HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        let (a, b) = self.arena.insert_radial_pair(data_a, data_b);
        let entity_a = forge_core::EntityRef::from(a);
        let entity_b = forge_core::EntityRef::from(b);
        self.mutation_journal.record_creation(entity_a);
        self.mutation_journal.record_creation(entity_b);
        self.emit_operation_event(TopoOperationEvent::EntityCreated(entity_a));
        self.emit_operation_event(TopoOperationEvent::EntityCreated(entity_b));
        (a, b)
    }

    // ── Operation Runner ─────────────────────────────────────────────
    // See `operation_runner.rs` for `execute()`.
}

impl Drop for MutableDraft {
    fn drop(&mut self) {
        if !self.committed {
            self.apply_rollback_once();
            tracing::warn!(
                base_epoch = self.base_epoch,
                ops_applied = self.op_counter.get(),
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
