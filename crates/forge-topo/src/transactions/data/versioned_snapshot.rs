//! Immutable epoch-versioned topology state.
//!
//! DOMAIN: TopologyState is the read-only snapshot of topology. All mutation
//! goes through MutableDraft. Previous states survive as Arc references for undo.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::b_rep::TopologyArena;
use crate::provenance::LineageStore;
use crate::provenance::ReidentificationLinkIndex;
use crate::provenance::{LineageEvent, ReplayLog};

use crate::transactions::data::draft_configuration::DraftConfig;
use crate::transactions::logic::mutable_draft::MutableDraft;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyState {
    /// Monotonically increasing epoch counter
    pub(crate) epoch: u64,
    /// Topology version (changes when connectivity changes)
    pub(crate) topology_version: u64,
    /// Geometry version (changes when positions change without topology change)
    pub(crate) geometry_version: u64,
    /// Structural hash of all topology (Merkle-style aggregate)
    pub(crate) topology_hash: u128,
    /// The topology arena holding all entity data (Milestone 0.5.1).
    /// Wrapped in `Arc` for cheap cloning and structural sharing.
    pub(crate) arena: Arc<TopologyArena>,
    /// Chronological log of lineage events that produced this state.
    ///
    /// Accumulated across epochs: each `commit()` appends the draft's events
    /// to the prior state's history so the full provenance chain survives.
    pub(crate) lineage_events: Arc<Vec<LineageEvent>>,
    /// Queryable one-hop lineage linkage index for persistent re-identification.
    ///
    /// Built at commit time from `lineage_events`. Snapshot-scoped + deterministic.
    #[serde(default)]
    pub(crate) reidentification_link_index: Arc<ReidentificationLinkIndex>,
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
    /// use forge_topo::transactions::TopologyState;
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
    ///
    /// The `group_policy` on `config` is auto-resolved from the arena's
    /// declared `ShellKind` metadata (Option A: model-derived context).
    /// If the caller has already set a custom `group_policy`, it is
    /// overwritten — the model is the source of truth for per-op policy.
    pub fn into_mutation_with(self, mut config: DraftConfig) -> MutableDraft {
        // CONSUME-ON-WRITE:
        // Try to unwrap the Arc. If we are the only owner, we get the Arena for free (O(1)).
        // If shared, we must clone (O(N)).
        let arena = match Arc::try_unwrap(self.arena) {
            Ok(arena) => arena,
            Err(arc) => (*arc).clone(),
        };

        // ── Option A: derive group policy from declared shell metadata ──
        let ctx =
            crate::validators::group_policy_runtime::topology_context_from_shell_metadata(&arena);
        config.group_policy = crate::validators::group_policy_runtime::GroupPolicyRuntime::resolve(
            0, // no force-skip (would come from GroupPolicyConfig in forge-kernel)
            0, // no force-per-op
            config.group_policy.max_cost_snapshot(),
            &ctx,
        );

        // Carry forward the prior lineage history so new events append to it.
        let prior_events = match Arc::try_unwrap(self.lineage_events) {
            Ok(events) => events,
            Err(arc) => (*arc).clone(),
        };

        let mut draft = MutableDraft {
            draft_id: crate::identity::DraftId::new(self.epoch + 1),
            base_epoch: self.epoch,
            next_epoch: self.epoch + 1,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            op_counter: crate::identity::OperationId::new(0),
            committed: false,
            replay_log: ReplayLog::new(),
            topology_hash: self.topology_hash,
            config,
            arena,
            lineage_store: LineageStore::from_prior_events(&prior_events),
            prior_lineage_events: prior_events,
            mutation_journal: crate::transactions::data::mutation_journal::MutationJournal::new(),
            poisoned: false,
            rollback_applied: false,
            event_bus: forge_signal::facade::specialist::EventBus::new(),
            pending_operation_events: Vec::new(),
        };

        crate::transactions::logic::subscribers::register_operation_subscribers(
            &mut draft.event_bus,
        )
        .expect("topo operation subscribers must register with valid deterministic DAG");

        draft
    }
}
