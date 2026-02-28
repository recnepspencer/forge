//! Mutable draft for transactional topology mutation (Doctrine D6).
//!
//! DOMAIN: MutableDraft wraps the arena for copy-on-write mutation.
//! Commit finalizes changes into a new TopologyState; drop rolls back.

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

use super::draft_config::DraftConfig;
use super::topology_state::TopologyState;

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
    pub(super) base_epoch: u64,
    /// The epoch this draft will produce if committed
    pub(super) next_epoch: u64,
    /// Current topology version (may be bumped during mutations)
    pub(super) topology_version: u64,
    /// Current geometry version (may be bumped during mutations)
    pub(super) geometry_version: u64,
    /// Lineage events recorded during this draft
    pub(super) lineage_log: Vec<LineageEvent>,
    /// Counter for assigning unique operation IDs within this draft
    pub(super) op_counter: u64,
    /// Whether commit() was called
    pub(super) committed: bool,
    /// Replay log for this draft (Milestone 0.4)
    pub(super) replay_log: ReplayLog,
    /// Current topology hash (for replay entry pre-hashes)
    pub(super) topology_hash: u128,
    /// Draft configuration (per-op hashing, deterministic seed)
    pub(super) config: DraftConfig,
    /// The mutable arena — cloned from the source state's Arc on begin_mutation
    pub(crate) arena: TopologyArena,
    /// Live lineage store tracking all entity provenance during this draft.
    pub(super) lineage_store: LineageStore,
    /// Lineage events inherited from the prior committed state.
    pub(super) prior_lineage_events: Vec<LineageEvent>,
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
    // Generated by `define_draft_proxies!` macro in `arena/crud_macro.rs`.
    // insert_*/remove_* for all 9 entity types are auto-generated there.

    /// Insert a pair of radial halfedges and wire their `radial_next` fields.
    pub fn insert_radial_pair(
        &mut self,
        data_a: crate::arena::HalfEdgeData,
        data_b: crate::arena::HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        self.arena.insert_radial_pair(data_a, data_b)
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
