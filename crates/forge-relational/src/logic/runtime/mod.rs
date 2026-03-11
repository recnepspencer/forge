use std::collections::{BTreeMap, VecDeque};
use std::sync::{Mutex, RwLock};

use crate::durability::data::DurableStore;
use crate::query::data::QueryWorkPacket;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::PartitionState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

mod state;

pub use crate::config::data::RelationalRuntimeConfig;
pub use crate::durability::data::RecoveryOutcome;
#[allow(unused_imports)]
pub use crate::performance::data::{
    ComplexityContract, ComplexityStatus, RuntimeComplexityCounters, COMPLEXITY_CONTRACTS,
};
pub use crate::replay::data::{RelationalReplayRecord, ReplaySchemaVersion};
pub use crate::simulation::data::{
    CompiledArtifactCompatibility, CompiledArtifactError, CompiledExecutionArtifact,
    TopologyFreezeMode,
};
#[allow(unused_imports)]
pub use crate::storage::data::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, EntityReadRecord,
    IndexedReadOutcome, PacketResult, PartitionStorageStats, RecordLifecycleState,
    RelationReadRecord, RelationalReadView, RetentionPassOutcome, RetentionPlan, StorageStats,
};
#[allow(unused_imports)]
pub use crate::validation::data::{
    InvariantCatalog, InvariantCheckResult, InvariantClass, InvariantExecutionPoint,
    InvariantFailureEffect, InvariantRule, InvariantViolation, StorageInvariantReport,
};

use crate::storage::logic::state::{BorrowedWorkingState, OverlayStateView};
pub(crate) use crate::storage::logic::state::{PartitionAccess, RelationalDraft, WorkingState};
pub(crate) use state::{
    DeterministicVersionWindowPolicy, DurabilityState, HistoryState, IndexState, LineageState,
    PublicationState, ReplayRetentionState, RuntimeInstrumentation, RuntimeSequenceState,
    SimulationState, SnapshotHandleBinding, SnapshotRegistry, VisibilityResidency,
};
pub use state::RelationalRuntime;

pub struct SnapshotGuard<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
    handle: SnapshotHandle,
}

impl SnapshotGuard<'_> {
    pub fn handle(&self) -> &SnapshotHandle {
        &self.handle
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.handle.snapshot_id
    }

    pub fn read(&self) -> Option<RelationalReadView> {
        self.runtime.read_snapshot(&self.handle)
    }
}

impl Drop for SnapshotGuard<'_> {
    fn drop(&mut self) {
        let _ = self.runtime.release_snapshot(&self.handle);
    }
}

impl RelationalRuntime {
    pub fn new(config: RelationalRuntimeConfig) -> Self {
        Self {
            partitions: BTreeMap::new(),
            snapshots: SnapshotRegistry {
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
            },
            publication: PublicationState::default(),
            history: HistoryState {
                branch_heads: BTreeMap::from([(config.main_branch.clone(), None)]),
                commit_graph: BTreeMap::new(),
                commit_envelopes: BTreeMap::new(),
                patch_stream_index: BTreeMap::new(),
                next_commit_id: 1,
                next_version_id: 1,
            },
            indexes: IndexState {
                definitions: BTreeMap::new(),
                generations: BTreeMap::new(),
                entity_unique_field_index: BTreeMap::new(),
                next_index_id: 1,
                next_generation_id: 1,
            },
            lineage: LineageState {
                nodes: BTreeMap::new(),
                events: Vec::new(),
                correspondence_candidates: Vec::new(),
                next_lineage_id: 1,
                next_event_id: 1,
            },
            durability: DurabilityState {
                log: Vec::new(),
                checkpoints: Vec::new(),
                store: config
                    .durable_store_layout
                    .clone()
                    .map(|layout| DurableStore {
                        layout,
                        segments: Vec::new(),
                        checkpoints: Vec::new(),
                    }),
            },
            sequence: RuntimeSequenceState {
                next_transaction_id: 1,
                next_savepoint_id: 1,
            },
            symbols: StringInterner::default(),
            instrumentation: RuntimeInstrumentation {
                complexity_counters: Mutex::new(RuntimeComplexityCounters::default()),
            },
            simulation: SimulationState {
                compiled_artifacts: BTreeMap::new(),
                next_compiled_artifact_id: 1,
            },
            config,
        }
    }

    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    pub fn fork(&self) -> Self {
        Self {
            config: self.config.clone(),
            partitions: self.partitions.clone(),
            snapshots: SnapshotRegistry {
                active: self.snapshots.active.clone(),
                published_handles: self.snapshots.published_handles.clone(),
                visibility_states: RwLock::new(
                    self.snapshots
                        .visibility_states
                        .read()
                        .expect("visibility state lock poisoned")
                        .clone(),
                ),
                visibility_residency: RwLock::new(
                    self.snapshots
                        .visibility_residency
                        .read()
                        .expect("visibility residency lock poisoned")
                        .clone(),
                ),
                recent_policy: Mutex::new(
                    self.snapshots
                        .recent_policy
                        .lock()
                        .expect("recent visibility policy lock poisoned")
                        .clone(),
                ),
                replay_retained: self.snapshots.replay_retained.clone(),
                next_snapshot_id: self.snapshots.next_snapshot_id,
            },
            publication: self.publication.clone(),
            history: self.history.clone(),
            indexes: self.indexes.clone(),
            lineage: self.lineage.clone(),
            durability: self.durability.clone(),
            sequence: self.sequence.clone(),
            symbols: self.symbols.clone(),
            instrumentation: RuntimeInstrumentation {
                complexity_counters: Mutex::new(
                    self.instrumentation
                        .complexity_counters
                        .lock()
                        .expect("complexity counter lock poisoned")
                        .clone(),
                ),
            },
            simulation: self.simulation.clone(),
        }
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
            .map(|partition| partition.entity_arena.generations.len())
            .sum()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.relation_arena.generations.len())
            .sum()
    }

    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id =
            crate::transactions::data::TransactionId(self.sequence.next_transaction_id);
        self.sequence.next_transaction_id += 1;
        RelationalTransaction {
            runtime: self,
            transaction_id,
            options,
            batches: Vec::new(),
            savepoints: Vec::new(),
            last_merged_plan: None,
        }
    }

    pub fn snapshot(&mut self) -> SnapshotHandle {
        let (handle, state) = self.snapshot_state_for_current(self.current_version_id());
        self.snapshots.active.insert(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.config.visibility_cache_policy.protect_active_snapshots {
            self.insert_visibility_state(state.clone());
            self.bump_active_snapshot_ref(handle.version_id, 1);
        }
        handle
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard<'_>> {
        if self.read_or_reconstruct_visibility_state(version_id, false).is_none() {
            return None;
        }
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let state = self.build_visibility_state(
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinned,
        );
        self.pin_snapshot_state(&state);
        let handle = state.handle.clone();
        self.snapshots.active.insert(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.config.visibility_cache_policy.protect_active_snapshots {
            self.insert_visibility_state(state);
            self.bump_active_snapshot_ref(handle.version_id, 1);
        }
        Some(SnapshotGuard {
            runtime: self,
            handle,
        })
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        if let Some(binding) = self.snapshots.active.remove(&handle.snapshot_id) {
            let state = self
                .visibility_state_for_version(binding.version_id)
                .unwrap_or_else(|| {
                    self.build_visibility_state(
                        binding.version_id,
                        SnapshotId(0),
                        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                    )
                });
            self.unpin_snapshot_state(&state);
            if self.config.visibility_cache_policy.protect_active_snapshots {
                self.bump_active_snapshot_ref(binding.version_id, -1);
            }
            self.evict_visibility_cache_if_needed();
            if self.config.mvcc.snapshot_release_policy
                == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
            {
                self.run_retention_pass();
            }
            return true;
        }
        self.snapshots
            .published_handles
            .remove(&handle.snapshot_id)
            .is_some()
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        if let Some(binding) = self.snapshots.active.get(&handle.snapshot_id) {
            let state = self.read_or_reconstruct_visibility_state(
                binding.version_id,
                !self.config.visibility_cache_policy.protect_active_snapshots,
            )?;
            let mut read_view = self.read_from_snapshot_state(&state);
            read_view.snapshot = SnapshotHandle {
                snapshot_id: handle.snapshot_id,
                version_id: binding.version_id,
                read_policy: binding.read_policy,
            };
            return Some(read_view);
        }
        let version_id = *self.snapshots.published_handles.get(&handle.snapshot_id)?;
        let mut read_view = self.read_version(version_id);
        read_view.snapshot = handle.clone();
        Some(read_view)
    }

    pub fn read_version(&self, version_id: crate::identity::data::VersionId) -> RelationalReadView {
        let state = self
            .read_or_reconstruct_visibility_state(version_id, true)
            .unwrap_or_else(|| {
                self.build_visibility_state(
                    version_id,
                    SnapshotId(0),
                    SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                )
            });
        self.read_from_snapshot_state(&state)
    }

    pub fn execute_read_packet(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<PacketResult> {
        self.read_snapshot(handle)
            .map(|read_view| read_view.execute_packet(packet))
    }

    pub fn complexity_contracts(&self) -> &'static [ComplexityContract] {
        COMPLEXITY_CONTRACTS
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

    pub fn invariants(&self, class: InvariantClass) -> StorageInvariantReport {
        StorageInvariantReport {
            violations: self
                .run_invariants_for_state(
                    &self.current_state(),
                    self.current_version_id(),
                    InvariantExecutionPoint::MutationSensitive,
                    false,
                    None,
                )
                .into_iter()
                .filter(|result| result.class == class)
                .flat_map(|result| result.violations)
                .collect(),
        }
    }

    pub fn run_invariants(
        &self,
        execution_point: InvariantExecutionPoint,
        include_harness_heavy: bool,
    ) -> Vec<InvariantCheckResult> {
        self.run_invariants_for_state(
            &self.current_state(),
            self.current_version_id(),
            execution_point,
            include_harness_heavy,
            None,
        )
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

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let Some(envelope) = self.history.commit_envelopes.remove(&commit_id) else {
            return false;
        };
        self.history
            .patch_stream_index
            .remove(&envelope.patch.position);
        true
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::data::diff::RelationalPatchRecord),
    ) -> bool {
        let Some(envelope) = self.history.commit_envelopes.get_mut(&commit_id) else {
            return false;
        };
        mutate(&mut envelope.patch);
        true
    }

    #[cfg(test)]
    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partition(entity_id.partition_id)
            .map(|partition| {
                partition.entity_arena.payload_history[entity_id.local_slot.0 as usize].len()
            })
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
                    .payload_history
                    .get(relation_id.local_slot.0 as usize)
            })
            .map(Vec::len)
            .unwrap_or(0)
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
}
