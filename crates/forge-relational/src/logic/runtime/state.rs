use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::{Mutex, RwLock};

use crate::durability::data::{DurableCheckpoint, DurableStore};
use crate::history::data::{BranchId, VersionNode};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::publication::data::PublicationBundle;
use crate::publication::data::diff::PatchStreamPosition;
use crate::replay::data::{CanonicalCommitEnvelope, RelationalReplayRecord};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::data::RelationalReadView;
use crate::storage::overlay::{
    BorrowedWorkingState, OverlayStateView, PartitionState, RelationalDraft, SnapshotState,
};
use crate::symbols::data::StringInterner;

use super::{
    CompiledExecutionArtifact, RelationalRuntimeConfig, RuntimeComplexityCounters,
};

#[derive(Debug, Default)]
pub(crate) struct SnapshotRegistry {
    pub(crate) active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    pub(crate) published_handles: BTreeMap<SnapshotId, crate::identity::data::VersionId>,
    pub(crate) visibility_states:
        RwLock<BTreeMap<crate::identity::data::VersionId, SnapshotState>>,
    pub(crate) visibility_residency:
        RwLock<BTreeMap<crate::identity::data::VersionId, VisibilityResidency>>,
    pub(crate) recent_policy: Mutex<DeterministicVersionWindowPolicy>,
    pub(crate) replay_retained: BTreeMap<crate::identity::data::VersionId, ReplayRetentionState>,
    pub(crate) next_snapshot_id: u64,
}

impl SnapshotRegistry {
    pub(crate) fn new(config: &RelationalRuntimeConfig) -> Self {
        Self {
            active: BTreeMap::new(),
            published_handles: BTreeMap::new(),
            visibility_states: RwLock::new(BTreeMap::new()),
            visibility_residency: RwLock::new(BTreeMap::new()),
            recent_policy: Mutex::new(DeterministicVersionWindowPolicy {
                recent_version_window: config.visibility_cache_policy.recent_version_window,
                order: VecDeque::new(),
                resident_count: 0,
            }),
            replay_retained: BTreeMap::new(),
            next_snapshot_id: 1,
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            active: self.active.clone(),
            published_handles: self.published_handles.clone(),
            visibility_states: RwLock::new(
                self.visibility_states
                    .read()
                    .expect("visibility state lock poisoned")
                    .clone(),
            ),
            visibility_residency: RwLock::new(
                self.visibility_residency
                    .read()
                    .expect("visibility residency lock poisoned")
                    .clone(),
            ),
            recent_policy: Mutex::new(
                self.recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .clone(),
            ),
            replay_retained: self.replay_retained.clone(),
            next_snapshot_id: self.next_snapshot_id,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotHandleBinding {
    pub(crate) version_id: crate::identity::data::VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VisibilityResidency {
    pub(crate) branch_head_refs: u32,
    pub(crate) replay_refs: u32,
    pub(crate) active_snapshot_refs: u32,
    pub(crate) recent_resident: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DeterministicVersionWindowPolicy {
    pub(crate) recent_version_window: usize,
    pub(crate) order: VecDeque<crate::identity::data::VersionId>,
    pub(crate) resident_count: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ReplayRetentionState {
    pub(crate) ref_count: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PublicationState {
    pub(crate) diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    pub(crate) latest_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
}

#[derive(Debug, Clone)]
pub(crate) struct HistoryState {
    pub(crate) branch_heads: BTreeMap<BranchId, Option<crate::history::data::CommitReference>>,
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    pub(crate) commit_envelopes: BTreeMap<crate::history::data::CommitId, CanonicalCommitEnvelope>,
    pub(crate) patch_stream_index: BTreeMap<PatchStreamPosition, crate::history::data::CommitId>,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
}

impl HistoryState {
    pub(crate) fn new(main_branch: BranchId) -> Self {
        Self {
            branch_heads: BTreeMap::from([(main_branch, None)]),
            commit_graph: BTreeMap::new(),
            commit_envelopes: BTreeMap::new(),
            patch_stream_index: BTreeMap::new(),
            next_commit_id: 1,
            next_version_id: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct IndexState {
    pub(crate) definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    pub(crate) generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    pub(crate) entity_unique_field_index:
        BTreeMap<String, BTreeMap<String, BTreeSet<crate::identity::data::EntityId>>>,
    pub(crate) next_index_id: u64,
    pub(crate) next_generation_id: u64,
}

impl IndexState {
    pub(crate) fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
            generations: BTreeMap::new(),
            entity_unique_field_index: BTreeMap::new(),
            next_index_id: 1,
            next_generation_id: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LineageState {
    pub(crate) nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) events: Vec<LineageEventRecord>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_event_id: u64,
}

impl LineageState {
    pub(crate) fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            events: Vec::new(),
            correspondence_candidates: Vec::new(),
            next_lineage_id: 1,
            next_event_id: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DurabilityState {
    pub(crate) log: Vec<CanonicalCommitEnvelope>,
    pub(crate) checkpoints: Vec<DurableCheckpoint>,
    pub(crate) store: Option<DurableStore>,
}

impl DurabilityState {
    pub(crate) fn new(config: &RelationalRuntimeConfig) -> Self {
        Self {
            log: Vec::new(),
            checkpoints: Vec::new(),
            store: config.durability.store_layout.clone().map(|layout| DurableStore {
                    layout,
                    segments: Vec::new(),
                    checkpoints: Vec::new(),
                }),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeSequenceState {
    pub(crate) next_transaction_id: u64,
    pub(crate) next_savepoint_id: u64,
}

impl RuntimeSequenceState {
    pub(crate) fn new() -> Self {
        Self {
            next_transaction_id: 1,
            next_savepoint_id: 1,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RuntimeInstrumentation {
    pub(crate) complexity_counters: Mutex<RuntimeComplexityCounters>,
}

impl RuntimeInstrumentation {
    pub(crate) fn new() -> Self {
        Self {
            complexity_counters: Mutex::new(RuntimeComplexityCounters::default()),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            complexity_counters: Mutex::new(
                self.complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .clone(),
            ),
        }
    }

    pub(crate) fn count(&self, update: impl FnOnce(&mut RuntimeComplexityCounters)) {
        update(
            &mut self
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned"),
        );
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SimulationState {
    pub(crate) compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    pub(crate) next_compiled_artifact_id: u64,
}

impl SimulationState {
    pub(crate) fn new() -> Self {
        Self {
            compiled_artifacts: BTreeMap::new(),
            next_compiled_artifact_id: 1,
        }
    }
}

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) snapshots: SnapshotRegistry,
    pub(crate) publication: PublicationState,
    pub(crate) history: HistoryState,
    pub(crate) indexes: IndexState,
    pub(crate) lineage: LineageState,
    pub(crate) durability: DurabilityState,
    pub(crate) sequence: RuntimeSequenceState,
    pub(crate) symbols: StringInterner,
    pub(crate) instrumentation: RuntimeInstrumentation,
    pub(crate) simulation: SimulationState,
}

impl RelationalRuntime {
    pub(crate) fn active_snapshot_count(&self) -> usize {
        self.snapshots.active.len()
    }

    pub(crate) fn branch_head_versions(&self) -> Vec<crate::identity::data::VersionId> {
        self.history
            .branch_heads
            .values()
            .filter_map(|head| head.as_ref().map(|head| head.version_id))
            .collect()
    }

    pub(crate) fn durable_store_layout(
        &self,
    ) -> Option<crate::durability::data::DurableStoreLayout> {
        self.config.durability.store_layout.clone()
    }

    pub(crate) fn set_durable_store(
        &mut self,
        store: Option<crate::durability::data::DurableStore>,
    ) {
        self.durability.store = store;
    }

    pub(crate) fn latest_durable_checkpoint(&self) -> Option<&DurableCheckpoint> {
        self.durability.checkpoints.last()
    }

    pub(crate) fn durable_log_len(&self) -> usize {
        self.durability.log.len()
    }

    pub(crate) fn push_durable_log_entry(
        &mut self,
        envelope: crate::replay::data::CanonicalCommitEnvelope,
    ) {
        self.durability.log.push(envelope);
    }

    pub(crate) fn last_durable_log_commit_id(
        &self,
    ) -> Option<crate::history::data::CommitId> {
        self.durability
            .log
            .last()
            .map(|entry| entry.commit.commit_id)
    }

    pub(crate) fn retain_durable_log_newer_than(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) {
        self.durability
            .log
            .retain(|entry| entry.commit.commit_id > commit_id);
    }

    pub(crate) fn drain_oldest_durable_log_entries(&mut self, count: usize) {
        self.durability.log.drain(0..count);
    }

    pub(crate) fn push_durable_checkpoint(&mut self, checkpoint: DurableCheckpoint) {
        self.durability.checkpoints.push(checkpoint);
    }

    pub(crate) fn commit_envelopes_snapshot(
        &self,
    ) -> Vec<crate::replay::data::CanonicalCommitEnvelope> {
        self.history.commit_envelopes.values().cloned().collect()
    }

    pub(crate) fn lineage_nodes_snapshot(&self) -> Vec<crate::lineage::data::LineageNode> {
        self.lineage.nodes.values().cloned().collect()
    }

    pub(crate) fn lineage_events_snapshot(
        &self,
    ) -> Vec<crate::lineage::data::LineageEventRecord> {
        self.lineage.events.clone()
    }

    pub(crate) fn correspondence_candidates_snapshot(
        &self,
    ) -> Vec<crate::lineage::data::CorrespondenceCandidate> {
        self.lineage.correspondence_candidates.clone()
    }

    pub(crate) fn index_definitions_snapshot(
        &self,
    ) -> Vec<crate::indexes::data::DerivedIndexDefinition> {
        self.indexes.definitions.values().cloned().collect()
    }

    pub(crate) fn index_generations_snapshot(
        &self,
    ) -> Vec<crate::indexes::data::DerivedIndexGeneration> {
        self.indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter().cloned())
            .collect()
    }

    pub(crate) fn symbol_table_snapshot(
        &self,
    ) -> crate::symbols::data::SymbolTableSnapshot {
        self.symbols.snapshot()
    }

    pub(crate) fn resolve_symbol_name(
        &self,
        symbol: crate::symbols::data::Symbol,
    ) -> Option<&str> {
        self.symbols.resolve(symbol)
    }

    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    pub(crate) fn partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> Option<&PartitionState> {
        self.partitions.get(&partition_id)
    }

    pub(crate) fn entity_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.entity_arena.slot_count())
            .sum()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.relation_arena.slot_count())
            .sum()
    }

    pub(crate) fn entity_chunk_size(&self) -> usize {
        self.config.storage_layout.entity_chunk_size.max(1)
    }

    pub(crate) fn relation_chunk_size(&self) -> usize {
        self.config.storage_layout.relation_chunk_size.max(1)
    }

    pub fn complexity_counters(&self) -> RuntimeComplexityCounters {
        self.instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone()
    }

    pub fn reset_complexity_counters(&self) {
        *self
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned") = RuntimeComplexityCounters::default();
    }

    pub(crate) fn primary_schema_version(&self) -> crate::schema::data::SchemaVersionId {
        self.config
            .schema_registry
            .entity_kinds
            .values()
            .next()
            .map(|registration| registration.schema_version_id)
            .or_else(|| {
                self.config
                    .schema_registry
                    .relation_kinds
                    .values()
                    .next()
                    .map(|registration| registration.schema_version_id)
            })
            .unwrap_or(crate::schema::data::SchemaVersionId(0))
    }

    pub(crate) fn current_state(&self) -> BorrowedWorkingState<'_> {
        BorrowedWorkingState::new(&self.partitions)
    }

    pub(crate) fn touched_partition_overlay(
        &self,
        touched_partitions: impl IntoIterator<Item = crate::identity::data::PartitionId>,
    ) -> RelationalDraft {
        RelationalDraft::from_touched_partitions(
            &self.partitions,
            touched_partitions,
            self.config.adjacency_policy.clone(),
        )
    }

    pub(crate) fn overlay_state_view<'a>(
        &'a self,
        staged: &'a RelationalDraft,
    ) -> OverlayStateView<'a, RelationalDraft> {
        OverlayStateView::new(&self.partitions, staged)
    }

    pub(crate) fn mutation_config(&self) -> crate::config::data::MutationConfig {
        crate::config::data::MutationConfig {
            patch_surface_policy: self.config.publication.patch_surface_policy,
            cascade_delete_policy: self.config.cascade_delete_policy,
            adjacency_policy: self.config.adjacency_policy.clone(),
            cross_context_policy: self.config.cross_context_policy,
        }
    }

    pub(crate) fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        self.snapshots
            .active
            .values()
            .map(|binding| binding.version_id)
            .chain(self.snapshots.replay_retained.keys().copied())
            .min()
            .unwrap_or(published_version)
    }

    pub(crate) fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, SnapshotState) {
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let state = self.build_visibility_state(
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.pin_snapshot_state(&state);
        (state.handle.clone(), state)
    }

    pub(crate) fn visibility_state_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotState> {
        self.snapshots
            .visibility_states
            .read()
            .expect("visibility state lock poisoned")
            .get(&version_id)
            .cloned()
    }

    pub(crate) fn insert_visibility_state(&self, state: SnapshotState) {
        self.snapshots
            .visibility_states
            .write()
            .expect("visibility state lock poisoned")
            .insert(state.handle.version_id, state);
    }

    pub(crate) fn visibility_residency_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> VisibilityResidency {
        self.snapshots
            .visibility_residency
            .read()
            .expect("visibility residency lock poisoned")
            .get(&version_id)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn bump_active_snapshot_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.active_snapshot_refs =
                residency.active_snapshot_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.instrumentation
                .count(|counters| counters.visibility_cache_snapshot_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_replay_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.replay_refs = residency.replay_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.instrumentation
                .count(|counters| counters.visibility_cache_replay_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_visibility_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        let mut residency = self
            .snapshots
            .visibility_residency
            .write()
            .expect("visibility residency lock poisoned");
        let entry = residency.entry(version_id).or_default();
        update(entry);
        if entry.branch_head_refs == 0
            && entry.replay_refs == 0
            && entry.active_snapshot_refs == 0
            && !entry.recent_resident
        {
            residency.remove(&version_id);
        }
        drop(residency);
        self.maybe_remove_unprotected_visibility_state(version_id);
    }

    pub(crate) fn protect_branch_head_version(&self, version_id: crate::identity::data::VersionId) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.branch_head_refs += 1;
        });
    }

    pub(crate) fn ensure_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        recent_candidate: bool,
    ) -> SnapshotState {
        if let Some(state) = self.visibility_state_for_version(version_id) {
            self.instrumentation
                .count(|counters| counters.visibility_cache_hits += 1);
            return state;
        }
        self.instrumentation
            .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
        let state = self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.insert_visibility_state(state.clone());
        if recent_candidate {
            self.mark_recent_visibility_state(version_id);
        }
        state
    }

    pub(crate) fn read_or_reconstruct_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        allow_recent_admission: bool,
    ) -> Option<SnapshotState> {
        if version_id.0 == 0 || version_id.0 > self.current_version_id().0 {
            return None;
        }
        if let Some(state) = self.visibility_state_for_version(version_id) {
            self.instrumentation
                .count(|counters| counters.visibility_cache_hits += 1);
            return Some(state);
        }
        let recent_candidate = allow_recent_admission
            && self.config.visibility_cache_policy.enabled
            && self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .recent_version_window
                > 0
            && !self.is_protected_visibility_version(version_id);
        if recent_candidate || self.is_protected_visibility_version(version_id) {
            return Some(self.ensure_visibility_state(version_id, recent_candidate));
        }
        self.instrumentation
            .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
        Some(self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        ))
    }

    pub(crate) fn is_protected_visibility_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let residency = self.visibility_residency_for_version(version_id);
        residency.branch_head_refs > 0
            || residency.replay_refs > 0
            || residency.active_snapshot_refs > 0
    }

    pub(crate) fn mark_recent_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
    ) {
        if !self.config.visibility_cache_policy.enabled
            || self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .recent_version_window
                == 0
        {
            return;
        }
        {
            let mut residency = self
                .snapshots
                .visibility_residency
                .write()
                .expect("visibility residency lock poisoned");
            let entry = residency.entry(version_id).or_default();
            if entry.recent_resident {
                return;
            }
            entry.recent_resident = true;
        }
        {
            let mut recent_policy = self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned");
            recent_policy.order.push_back(version_id);
            recent_policy.resident_count += 1;
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn evict_visibility_cache_if_needed(&self) {
        let window = self
            .snapshots
            .recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .recent_version_window;
        if !self.config.visibility_cache_policy.enabled || window == 0 {
            return;
        }
        loop {
            if self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .resident_count
                <= window
            {
                break;
            }
            let scan_len = self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .order
                .len();
            if scan_len == 0 {
                break;
            }
            let mut evicted = false;
            for _ in 0..scan_len {
                let candidate = self
                    .snapshots
                    .recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .order
                    .pop_front();
                let Some(version_id) = candidate else {
                    break;
                };
                let mut residency = self
                    .snapshots
                    .visibility_residency
                    .write()
                    .expect("visibility residency lock poisoned");
                let Some(entry) = residency.get_mut(&version_id) else {
                    continue;
                };
                if !entry.recent_resident {
                    continue;
                }
                if entry.branch_head_refs > 0
                    || entry.replay_refs > 0
                    || entry.active_snapshot_refs > 0
                {
                    drop(residency);
                    self.snapshots
                        .recent_policy
                        .lock()
                        .expect("recent visibility policy lock poisoned")
                        .order
                        .push_back(version_id);
                    continue;
                }
                entry.recent_resident = false;
                self.snapshots
                    .recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .resident_count -= 1;
                if entry.branch_head_refs == 0
                    && entry.replay_refs == 0
                    && entry.active_snapshot_refs == 0
                {
                    residency.remove(&version_id);
                }
                drop(residency);
                self.snapshots
                    .visibility_states
                    .write()
                    .expect("visibility state lock poisoned")
                    .remove(&version_id);
                self.instrumentation
                    .count(|counters| counters.visibility_cache_recent_evictions += 1);
                evicted = true;
                break;
            }
            if !evicted {
                break;
            }
        }
    }

    pub(crate) fn maybe_remove_unprotected_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
    ) {
        let residency = self.visibility_residency_for_version(version_id);
        if residency.branch_head_refs == 0
            && residency.replay_refs == 0
            && residency.active_snapshot_refs == 0
            && !residency.recent_resident
        {
            self.snapshots
                .visibility_states
                .write()
                .expect("visibility state lock poisoned")
                .remove(&version_id);
        }
    }

    pub(crate) fn read_from_snapshot_state(&self, state: &SnapshotState) -> RelationalReadView {
        let current_state = self.current_state();
        let mut entities = Vec::with_capacity(state.pinned_entity_count);
        let mut relations = Vec::with_capacity(state.pinned_relation_count);
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                let entity_id = crate::identity::data::EntityId::new(*partition_id, slot as u64, 0);
                if let Some(record) = self.entity_record_for_id_at_version(
                    &current_state,
                    entity_id,
                    state.handle.version_id,
                ) {
                    entities.push(record);
                }
            }
            for slot in pins.relation_slots.iter_set_slots() {
                let relation_id =
                    crate::identity::data::RelationId::new(*partition_id, slot as u64, 0);
                if let Some(record) = self.relation_record_for_id_at_version(
                    &current_state,
                    relation_id,
                    state.handle.version_id,
                ) {
                    relations.push(record);
                }
            }
        }
        self.instrumentation.count(|counters| {
            counters.visible_entity_records_materialized += entities.len();
            counters.visible_relation_records_materialized += relations.len();
        });
        RelationalReadView {
            snapshot: state.handle.clone(),
            entities,
            relations,
        }
    }

    #[cfg(test)]
    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partition(entity_id.partition_id)
            .and_then(|partition| {
                partition
                    .entity_arena
                    .payload_history_at(entity_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }

    #[cfg(test)]
    pub(crate) fn relation_history_len_for_test(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> usize {
        self.partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .payload_history_at(relation_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }
}
