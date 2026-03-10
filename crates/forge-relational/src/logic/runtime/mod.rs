pub(crate) mod apply;
pub(crate) mod merge;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::durability::data::{DurableCheckpoint, DurableCommitEnvelope, DurableStore};
use crate::history::data::{BranchId, VersionNode};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::publication::data::PublicationBundle;
use crate::query::data::QueryWorkPacket;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PartitionState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

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

use crate::storage::logic::state::{BorrowedWorkingState, SnapshotState};
pub(crate) use crate::storage::logic::state::{PartitionAccess, WorkingState};

#[derive(Debug, Clone, Default)]
pub(crate) struct SnapshotRegistry {
    pub(crate) active: BTreeMap<SnapshotId, SnapshotState>,
    pub(crate) version_visibility_cache: BTreeMap<crate::identity::data::VersionId, SnapshotState>,
    pub(crate) replay_retained: BTreeMap<crate::identity::data::VersionId, ReplayRetentionState>,
    pub(crate) next_snapshot_id: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct ReplayRetentionState {
    pub(crate) state: SnapshotState,
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
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
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

#[derive(Debug, Clone, Default)]
pub(crate) struct LineageState {
    pub(crate) nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) events: Vec<LineageEventRecord>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_event_id: u64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DurabilityState {
    pub(crate) log: Vec<DurableCommitEnvelope>,
    pub(crate) checkpoints: Vec<DurableCheckpoint>,
    pub(crate) store: Option<DurableStore>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeSequenceState {
    pub(crate) next_transaction_id: u64,
    pub(crate) next_savepoint_id: u64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeInstrumentation {
    pub(crate) complexity_counters: RefCell<RuntimeComplexityCounters>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SimulationState {
    pub(crate) compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    pub(crate) next_compiled_artifact_id: u64,
}

#[derive(Debug, Clone)]
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
    pub fn new(config: RelationalRuntimeConfig) -> Self {
        Self {
            partitions: BTreeMap::new(),
            snapshots: SnapshotRegistry {
                active: BTreeMap::new(),
                version_visibility_cache: BTreeMap::new(),
                replay_retained: BTreeMap::new(),
                next_snapshot_id: 1,
            },
            publication: PublicationState::default(),
            history: HistoryState {
                branch_heads: BTreeMap::from([(config.main_branch.clone(), None)]),
                commit_graph: BTreeMap::new(),
                commit_envelopes: BTreeMap::new(),
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
                complexity_counters: RefCell::new(RuntimeComplexityCounters::default()),
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
        self.snapshots.active.insert(handle.snapshot_id, state);
        handle
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        let Some(state) = self.snapshots.active.remove(&handle.snapshot_id) else {
            return false;
        };
        self.unpin_snapshot_state(&state);
        if self.config.mvcc.snapshot_release_policy
            == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
        {
            self.run_retention_pass();
        }
        true
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        self.snapshots
            .active
            .get(&handle.snapshot_id)
            .map(|state| self.read_from_snapshot_state(state))
    }

    pub fn read_version(&self, version_id: crate::identity::data::VersionId) -> RelationalReadView {
        if let Some(state) = self.snapshots.version_visibility_cache.get(&version_id) {
            return self.read_from_snapshot_state(state);
        }
        if let Some(snapshot_state) = self
            .snapshots
            .active
            .values()
            .find(|state| state.handle.version_id == version_id)
        {
            return self.read_from_snapshot_state(snapshot_state);
        }
        let current_state = self.current_state();
        RelationalReadView {
            snapshot: SnapshotHandle {
                snapshot_id: SnapshotId(0),
                version_id,
                read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
            },
            entities: self.visible_entities_from_state(&current_state, version_id),
            relations: self.visible_relations_from_state(&current_state, version_id),
        }
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
        self.instrumentation.complexity_counters.borrow().clone()
    }

    pub fn reset_complexity_counters(&self) {
        *self.instrumentation.complexity_counters.borrow_mut() =
            RuntimeComplexityCounters::default();
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
        self.history.commit_envelopes.remove(&commit_id).is_some()
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
                    .get(&(relation_id.local_slot.0 as usize))
                    .map(Vec::len)
            })
            .unwrap_or(0)
    }

    pub(crate) fn current_state(&self) -> BorrowedWorkingState<'_> {
        BorrowedWorkingState::new(&self.partitions)
    }

    pub(crate) fn take_working_state(&mut self) -> WorkingState {
        WorkingState::new(
            std::mem::take(&mut self.partitions),
            self.config.adjacency_policy.clone(),
        )
    }
}
