pub(crate) mod apply;
pub(crate) mod merge;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::durability::data::{DurableCheckpoint, DurableCommitEnvelope};
use crate::history::data::{
    BranchCreateError, BranchHead, BranchId, VersionGraphSnapshot, VersionNode,
};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::publication::data::PublicationBundle;
use crate::query::data::QueryWorkPacket;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::snapshots::data::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
use crate::storage::logic::state::PartitionState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

pub use crate::config::data::RelationalRuntimeConfig;
pub use crate::diagnostics::data::RelationalDiagnosticsFacade;
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
    RelationReadRecord, RelationalReadView, RetentionPassOutcome, StorageStats,
};
#[allow(unused_imports)]
pub use crate::validation::data::{
    InvariantCatalog, InvariantCheckResult, InvariantClass, InvariantExecutionPoint,
    InvariantFailureEffect, InvariantRule, InvariantViolation, StorageInvariantReport,
};

use crate::storage::logic::state::{BorrowedWorkingState, SnapshotState};
pub(crate) use crate::storage::logic::state::{PartitionAccess, WorkingState};
#[derive(Debug, Clone)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) snapshots: BTreeMap<SnapshotId, SnapshotState>,
    pub(crate) diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    pub(crate) latest_publication_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
    pub(crate) branch_heads: BTreeMap<BranchId, Option<crate::history::data::CommitReference>>,
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    pub(crate) commit_envelopes: BTreeMap<crate::history::data::CommitId, CanonicalCommitEnvelope>,
    pub(crate) index_definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    pub(crate) index_generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    pub(crate) lineage_nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) lineage_events: Vec<LineageEventRecord>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) entity_unique_field_index:
        BTreeMap<String, BTreeMap<String, BTreeSet<crate::identity::data::EntityId>>>,
    pub(crate) durable_log: Vec<DurableCommitEnvelope>,
    pub(crate) durable_checkpoints: Vec<DurableCheckpoint>,
    pub(crate) next_index_id: u64,
    pub(crate) next_index_generation_id: u64,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_lineage_event_id: u64,
    pub(crate) next_transaction_id: u64,
    pub(crate) next_savepoint_id: u64,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
    pub(crate) next_snapshot_id: u64,
    pub(crate) symbol_interner: RefCell<StringInterner>,
    pub(crate) complexity_counters: RefCell<RuntimeComplexityCounters>,
    pub(crate) compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    pub(crate) next_compiled_artifact_id: u64,
}

impl RelationalRuntime {
    pub fn new(config: RelationalRuntimeConfig) -> Self {
        Self {
            partitions: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            diagnostics: Vec::new(),
            latest_publication_bundle: None,
            branch_heads: BTreeMap::from([(config.main_branch.clone(), None)]),
            commit_graph: BTreeMap::new(),
            commit_envelopes: BTreeMap::new(),
            index_definitions: BTreeMap::new(),
            index_generations: BTreeMap::new(),
            lineage_nodes: BTreeMap::new(),
            lineage_events: Vec::new(),
            correspondence_candidates: Vec::new(),
            entity_unique_field_index: BTreeMap::new(),
            durable_log: Vec::new(),
            durable_checkpoints: Vec::new(),
            next_index_id: 1,
            next_index_generation_id: 1,
            next_lineage_id: 1,
            next_lineage_event_id: 1,
            next_transaction_id: 1,
            next_savepoint_id: 1,
            next_commit_id: 1,
            next_version_id: 1,
            next_snapshot_id: 1,
            symbol_interner: RefCell::new(StringInterner::default()),
            complexity_counters: RefCell::new(RuntimeComplexityCounters::default()),
            compiled_artifacts: BTreeMap::new(),
            next_compiled_artifact_id: 1,
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

    fn entity_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.entity_arena.generations.len())
            .sum()
    }

    fn relation_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.relation_arena.generations.len())
            .sum()
    }

    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id = crate::transactions::data::TransactionId(self.next_transaction_id);
        self.next_transaction_id += 1;
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
        self.snapshots.insert(handle.snapshot_id, state);
        handle
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        let Some(state) = self.snapshots.remove(&handle.snapshot_id) else {
            return false;
        };
        for entity_id in state.pinned_entities {
            self.unpin_entity(entity_id);
        }
        for relation_id in state.pinned_relations {
            self.unpin_relation(relation_id);
        }
        if self.config.mvcc.snapshot_release_policy
            == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
        {
            self.run_retention_pass();
        }
        true
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        self.snapshots.get(&handle.snapshot_id).map(|state| {
            let current_state = self.current_state();
            let entities = state
                .pinned_entities
                .iter()
                .filter_map(|entity_id| {
                    self.entity_record_for_id_at_version(
                        &current_state,
                        *entity_id,
                        state.handle.version_id,
                    )
                })
                .collect::<Vec<_>>();
            let relations = state
                .pinned_relations
                .iter()
                .filter_map(|relation_id| {
                    self.relation_record_for_id_at_version(
                        &current_state,
                        *relation_id,
                        state.handle.version_id,
                    )
                })
                .collect::<Vec<_>>();
            {
                let mut counters = self.complexity_counters.borrow_mut();
                counters.visible_entity_records_materialized += entities.len();
                counters.visible_relation_records_materialized += relations.len();
            }
            RelationalReadView {
                snapshot: state.handle.clone(),
                entities,
                relations,
            }
        })
    }

    pub fn read_version(&self, version_id: crate::identity::data::VersionId) -> RelationalReadView {
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

    pub fn diagnostics(&self) -> RelationalDiagnosticsFacade {
        RelationalDiagnosticsFacade {
            artifacts: self.diagnostics.clone(),
        }
    }

    pub fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.latest_publication_bundle.as_ref()
    }

    pub fn complexity_contracts(&self) -> &'static [ComplexityContract] {
        COMPLEXITY_CONTRACTS
    }

    pub fn complexity_counters(&self) -> RuntimeComplexityCounters {
        self.complexity_counters.borrow().clone()
    }

    pub fn reset_complexity_counters(&self) {
        *self.complexity_counters.borrow_mut() = RuntimeComplexityCounters::default();
    }

    pub fn latest_patch(&self) -> Option<&crate::publication::data::diff::RelationalPatchRecord> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.replay)
    }

    pub fn latest_commit(&self) -> Option<&crate::history::data::CommitReference> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.commit)
    }

    pub fn branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Option<&crate::history::data::CommitReference> {
        self.branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }

    pub fn branches(&self) -> Vec<BranchHead> {
        self.branch_heads
            .iter()
            .map(|(branch_id, head)| BranchHead {
                branch_id: branch_id.clone(),
                head: head.clone(),
            })
            .collect()
    }

    pub fn partition_ids(&self) -> Vec<crate::identity::data::PartitionId> {
        self.partitions.keys().copied().collect()
    }

    pub fn partition_storage_stats(&self) -> Vec<PartitionStorageStats> {
        self.partitions
            .iter()
            .map(|(partition_id, partition)| PartitionStorageStats {
                partition_id: *partition_id,
                entity_slots: partition.entity_arena.generations.len(),
                entity_chunks: partition
                    .entity_arena
                    .generations
                    .len()
                    .div_ceil(self.config.storage_layout.entity_chunk_size.max(1)),
                live_entities: partition
                    .entity_arena
                    .lifecycle
                    .iter()
                    .filter(|state| **state == RecordLifecycleState::Live)
                    .count(),
                deleted_entities: partition
                    .entity_arena
                    .lifecycle
                    .iter()
                    .filter(|state| {
                        matches!(
                            state,
                            RecordLifecycleState::DeletedRetained
                                | RecordLifecycleState::PinnedBySnapshot
                                | RecordLifecycleState::PinnedByBranch
                                | RecordLifecycleState::PinnedByReplayRetention
                                | RecordLifecycleState::Reclaimable
                        )
                    })
                    .count(),
                reusable_entity_slots: partition
                    .entity_arena
                    .lifecycle
                    .iter()
                    .filter(|state| **state == RecordLifecycleState::Reusable)
                    .count(),
                relation_slots: partition.relation_arena.generations.len(),
                relation_chunks: partition
                    .relation_arena
                    .generations
                    .len()
                    .div_ceil(self.config.storage_layout.relation_chunk_size.max(1)),
                live_relations: partition
                    .relation_arena
                    .lifecycle
                    .iter()
                    .filter(|state| **state == RecordLifecycleState::Live)
                    .count(),
                deleted_relations: partition
                    .relation_arena
                    .lifecycle
                    .iter()
                    .filter(|state| {
                        matches!(
                            state,
                            RecordLifecycleState::DeletedRetained
                                | RecordLifecycleState::PinnedBySnapshot
                                | RecordLifecycleState::PinnedByBranch
                                | RecordLifecycleState::PinnedByReplayRetention
                                | RecordLifecycleState::Reclaimable
                        )
                    })
                    .count(),
                reusable_relation_slots: partition
                    .relation_arena
                    .lifecycle
                    .iter()
                    .filter(|state| **state == RecordLifecycleState::Reusable)
                    .count(),
            })
            .collect()
    }

    pub fn version_graph(&self) -> VersionGraphSnapshot {
        VersionGraphSnapshot {
            branches: self.branches(),
            commits: self.commit_graph.values().cloned().collect(),
        }
    }

    pub fn ancestor_chain(
        &self,
        commit_id: crate::history::data::CommitId,
    ) -> Vec<crate::history::data::CommitId> {
        let mut ordered = self.ancestor_set(commit_id).into_iter().collect::<Vec<_>>();
        ordered.sort_by_key(|id| id.0);
        ordered
    }

    pub fn latest_common_ancestor_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> Option<crate::history::data::CommitId> {
        let left = self.branch_head(left_branch)?.commit_id;
        let right = self.branch_head(right_branch)?.commit_id;
        self.latest_common_ancestor(left, right)
    }

    pub fn can_merge_branch_into(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> bool {
        let Some(source_head) = self.branch_head(source_branch) else {
            return false;
        };
        let Some(target_head) = self.branch_head(target_branch) else {
            return false;
        };
        self.latest_common_ancestor(target_head.commit_id, source_head.commit_id)
            .is_some()
    }

    pub fn inspect_merge(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> crate::history::data::MergeInspection {
        let source_head = self.branch_head(source_branch).cloned();
        let target_head = self.branch_head(target_branch).cloned();
        let merge_base =
            source_head
                .as_ref()
                .zip(target_head.as_ref())
                .and_then(|(source, target)| {
                    self.latest_common_ancestor(source.commit_id, target.commit_id)
                });

        let source_only_commits = source_head
            .as_ref()
            .map(|head| self.branch_unique_commits(head.commit_id, merge_base))
            .unwrap_or_default();
        let target_only_commits = target_head
            .as_ref()
            .map(|head| self.branch_unique_commits(head.commit_id, merge_base))
            .unwrap_or_default();
        let conflicting_records = self.merge_conflicts_between(
            source_only_commits.as_slice(),
            target_only_commits.as_slice(),
        );

        crate::history::data::MergeInspection {
            source_branch: source_branch.clone(),
            target_branch: target_branch.clone(),
            source_head,
            target_head,
            merge_base,
            source_only_commits,
            target_only_commits,
            can_merge: merge_base.is_some() && conflicting_records.is_empty(),
            conflicting_records,
        }
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        if self.branch_heads.contains_key(&new_branch) {
            return Err(BranchCreateError::BranchAlreadyExists);
        }
        let Some(source_head) = self.branch_heads.get(from_branch).cloned() else {
            return Err(BranchCreateError::SourceBranchMissing);
        };
        self.branch_heads.insert(new_branch, source_head);
        Ok(())
    }

    pub fn inspect_snapshot(&self, handle: &SnapshotHandle) -> Option<SnapshotInspectionSummary> {
        self.snapshots.get(&handle.snapshot_id).map(|state| {
            let current_state = self.current_state();
            let entities =
                self.visible_entities_from_state(&current_state, state.handle.version_id);
            let relations =
                self.visible_relations_from_state(&current_state, state.handle.version_id);
            SnapshotInspectionSummary {
                version_id: state.handle.version_id,
                entity_count: entities.len(),
                relation_count: relations.len(),
                pinned_entity_count: state.pinned_entities.len(),
                pinned_relation_count: state.pinned_relations.len(),
            }
        })
    }

    pub fn storage_stats(&self) -> StorageStats {
        let chunked_summary = self.chunked_storage_summary(self.current_version_id());
        StorageStats {
            entity_slots: self.entity_slot_count(),
            entity_chunks: chunked_summary.entity_chunks.len(),
            live_entities: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .entity_arena
                        .lifecycle
                        .iter()
                        .filter(|state| **state == RecordLifecycleState::Live)
                        .count()
                })
                .sum(),
            deleted_entities: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .entity_arena
                        .lifecycle
                        .iter()
                        .filter(|state| {
                            matches!(
                                state,
                                RecordLifecycleState::DeletedRetained
                                    | RecordLifecycleState::PinnedBySnapshot
                                    | RecordLifecycleState::PinnedByBranch
                                    | RecordLifecycleState::PinnedByReplayRetention
                                    | RecordLifecycleState::Reclaimable
                            )
                        })
                        .count()
                })
                .sum(),
            reusable_entity_slots: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .entity_arena
                        .lifecycle
                        .iter()
                        .filter(|state| **state == RecordLifecycleState::Reusable)
                        .count()
                })
                .sum(),
            relation_slots: self.relation_slot_count(),
            relation_chunks: chunked_summary.relation_chunks.len(),
            live_relations: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .relation_arena
                        .lifecycle
                        .iter()
                        .filter(|state| **state == RecordLifecycleState::Live)
                        .count()
                })
                .sum(),
            deleted_relations: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .relation_arena
                        .lifecycle
                        .iter()
                        .filter(|state| {
                            matches!(
                                state,
                                RecordLifecycleState::DeletedRetained
                                    | RecordLifecycleState::PinnedBySnapshot
                                    | RecordLifecycleState::PinnedByBranch
                                    | RecordLifecycleState::PinnedByReplayRetention
                                    | RecordLifecycleState::Reclaimable
                            )
                        })
                        .count()
                })
                .sum(),
            reusable_relation_slots: self
                .partitions
                .values()
                .map(|partition| {
                    partition
                        .relation_arena
                        .lifecycle
                        .iter()
                        .filter(|state| **state == RecordLifecycleState::Reusable)
                        .count()
                })
                .sum(),
            snapshot_count: self.snapshots.len(),
        }
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

    pub fn run_retention_pass(&mut self) -> RetentionPassOutcome {
        let mut outcome = RetentionPassOutcome {
            entity_reclaimable: 0,
            entity_reclaimed: 0,
            entity_chunks_scanned: 0,
            relation_reclaimable: 0,
            relation_reclaimed: 0,
            relation_chunks_scanned: 0,
        };

        let entity_chunk_size = self.config.storage_layout.entity_chunk_size.max(1);
        let relation_chunk_size = self.config.storage_layout.relation_chunk_size.max(1);
        let retention_fence = self.retention_fence_version(self.current_version_id());

        let partition_ids = self.partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in partition_ids {
            let entity_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.entity_arena.lifecycle.len())
                .unwrap_or(0);
            for slot_start in (0..entity_len).step_by(entity_chunk_size) {
                outcome.entity_chunks_scanned += 1;
                let slot_end = (slot_start + entity_chunk_size).min(entity_len);
                for slot in slot_start..slot_end {
                    self.complexity_counters
                        .borrow_mut()
                        .retention_entity_slots_scanned += 1;
                    let retired_at = self
                        .partitions
                        .get(&partition_id)
                        .and_then(|partition| partition.entity_arena.retired_at[slot]);
                    if let Some(version) = retired_at {
                        self.refresh_entity_retention_state(
                            partition_id,
                            slot,
                            Some(version),
                            retention_fence,
                        );
                        if self.partitions.get(&partition_id).is_some_and(|partition| {
                            partition.entity_arena.lifecycle[slot]
                                == RecordLifecycleState::Reclaimable
                        }) {
                            outcome.entity_reclaimable += 1;
                            if self.config.mvcc.auto_reclaim_deleted_records
                                && outcome.entity_reclaimed < self.config.mvcc.reclaim_batch_size
                            {
                                let partition = self
                                    .partitions
                                    .get_mut(&partition_id)
                                    .expect("entity partition for reclaim");
                                partition.entity_arena.lifecycle[slot] =
                                    RecordLifecycleState::Reusable;
                                partition.entity_arena.kind_ids[slot] = None;
                                partition.entity_arena.payloads[slot] = None;
                                partition.entity_arena.snapshot_pins[slot] = 0;
                                partition.entity_arena.branch_pins[slot] = 0;
                                partition.entity_arena.replay_pins[slot] = 0;
                                partition.entity_arena.retired_at[slot] = None;
                                partition.entity_arena.free_list.push(slot as u64);
                                outcome.entity_reclaimed += 1;
                            }
                        }
                    }
                }
            }

            let relation_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.relation_arena.lifecycle.len())
                .unwrap_or(0);
            for slot_start in (0..relation_len).step_by(relation_chunk_size) {
                outcome.relation_chunks_scanned += 1;
                let slot_end = (slot_start + relation_chunk_size).min(relation_len);
                for slot in slot_start..slot_end {
                    self.complexity_counters
                        .borrow_mut()
                        .retention_relation_slots_scanned += 1;
                    let retired_at = self
                        .partitions
                        .get(&partition_id)
                        .and_then(|partition| partition.relation_arena.retired_at[slot]);
                    if let Some(version) = retired_at {
                        self.refresh_relation_retention_state(
                            partition_id,
                            slot,
                            Some(version),
                            retention_fence,
                        );
                        if self.partitions.get(&partition_id).is_some_and(|partition| {
                            partition.relation_arena.lifecycle[slot]
                                == RecordLifecycleState::Reclaimable
                        }) {
                            outcome.relation_reclaimable += 1;
                            if self.config.mvcc.auto_reclaim_deleted_records
                                && outcome.relation_reclaimed < self.config.mvcc.reclaim_batch_size
                            {
                                let partition = self
                                    .partitions
                                    .get_mut(&partition_id)
                                    .expect("relation partition for reclaim");
                                partition.relation_arena.lifecycle[slot] =
                                    RecordLifecycleState::Reusable;
                                partition.relation_arena.kind_ids[slot] = None;
                                partition.relation_arena.payloads[slot] = None;
                                partition.relation_arena.payload_history.remove(&slot);
                                partition.relation_arena.snapshot_pins[slot] = 0;
                                partition.relation_arena.endpoints[slot] = None;
                                partition.relation_arena.retired_at[slot] = None;
                                partition.relation_arena.free_list.push(slot as u64);
                                outcome.relation_reclaimed += 1;
                            }
                        }
                    }
                }
            }
        }

        outcome
    }

    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        crate::identity::data::VersionId(self.next_version_id.saturating_sub(1))
    }

    fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        match self.config.retention_policy.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => self
                .snapshots
                .values()
                .map(|state| state.handle.version_id)
                .min()
                .unwrap_or(published_version),
            crate::config::data::RetentionBackend::EpochChunkRetention => self
                .snapshots
                .values()
                .map(|state| state.handle.version_id)
                .min()
                .unwrap_or(published_version),
        }
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

    pub(crate) fn latest_common_ancestor(
        &self,
        left: crate::history::data::CommitId,
        right: crate::history::data::CommitId,
    ) -> Option<crate::history::data::CommitId> {
        let left_ancestors = self.ancestor_set(left);
        let right_ancestors = self.ancestor_set(right);
        left_ancestors
            .intersection(&right_ancestors)
            .copied()
            .max_by_key(|commit_id| commit_id.0)
    }

    fn ancestor_set(
        &self,
        start: crate::history::data::CommitId,
    ) -> std::collections::BTreeSet<crate::history::data::CommitId> {
        let mut seen = std::collections::BTreeSet::new();
        let mut stack = vec![start];
        while let Some(commit_id) = stack.pop() {
            if !seen.insert(commit_id) {
                continue;
            }
            if let Some(node) = self.commit_graph.get(&commit_id) {
                stack.extend(node.commit.parents.iter().copied());
            }
        }
        seen
    }

    fn branch_unique_commits(
        &self,
        head: crate::history::data::CommitId,
        merge_base: Option<crate::history::data::CommitId>,
    ) -> Vec<crate::history::data::CommitId> {
        let mut commits = self.ancestor_set(head).into_iter().collect::<Vec<_>>();
        if let Some(merge_base) = merge_base {
            let base_ancestors = self.ancestor_set(merge_base);
            commits.retain(|commit_id| !base_ancestors.contains(commit_id));
        }
        commits.sort_by_key(|commit_id| commit_id.0);
        commits
    }

    fn merge_conflicts_between(
        &self,
        left_commits: &[crate::history::data::CommitId],
        right_commits: &[crate::history::data::CommitId],
    ) -> Vec<crate::history::data::MergeConflictRecord> {
        let left_records = self.commit_record_set(left_commits);
        let right_records = self.commit_record_set(right_commits);
        let mut conflicts = left_records
            .intersection(&right_records)
            .cloned()
            .collect::<Vec<_>>();
        conflicts.sort();
        conflicts
    }

    fn commit_record_set(
        &self,
        commits: &[crate::history::data::CommitId],
    ) -> std::collections::BTreeSet<crate::history::data::MergeConflictRecord> {
        commits
            .iter()
            .filter_map(|commit_id| self.commit_envelopes.get(commit_id))
            .flat_map(|envelope| envelope.patch.records.iter())
            .filter_map(|record| match (record.entity_id, record.relation_id) {
                (Some(entity_id), None) => {
                    Some(crate::history::data::MergeConflictRecord::Entity(entity_id))
                }
                (None, Some(relation_id)) => Some(
                    crate::history::data::MergeConflictRecord::Relation(relation_id),
                ),
                _ => None,
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        self.commit_envelopes.remove(&commit_id).is_some()
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

    fn current_state(&self) -> BorrowedWorkingState<'_> {
        BorrowedWorkingState::new(&self.partitions)
    }

    pub(crate) fn take_working_state(&mut self) -> WorkingState {
        WorkingState::new(
            std::mem::take(&mut self.partitions),
            self.config.adjacency_policy.clone(),
        )
    }

    pub(crate) fn refresh_unique_field_index_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        version_id: crate::identity::data::VersionId,
    ) {
        let tracked_fields = self.tracked_unique_entity_fields();
        if tracked_fields.is_empty() {
            return;
        }
        let state = self.current_state();
        let mut refreshed_values = Vec::new();
        for record in changed_records {
            let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
                continue;
            };
            for field in &tracked_fields {
                if let Some(payload) = crate::validation::logic::entity_payload_for_state(
                    &state, *entity_id, version_id,
                ) {
                    if let Some(value) = payload
                        .as_json()
                        .and_then(|value| value.get(field))
                        .and_then(|value| value.as_str())
                    {
                        refreshed_values.push((field.clone(), value.to_string(), *entity_id));
                    }
                }
            }
        }
        for record in changed_records {
            let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
                continue;
            };
            for field in &tracked_fields {
                if let Some(values) = self.entity_unique_field_index.get_mut(field) {
                    values.retain(|_, entity_ids| {
                        entity_ids.remove(entity_id);
                        !entity_ids.is_empty()
                    });
                }
            }
        }
        for (field, value, entity_id) in refreshed_values {
            self.entity_unique_field_index
                .entry(field)
                .or_default()
                .entry(value)
                .or_default()
                .insert(entity_id);
        }
    }

    #[allow(dead_code)]
    pub(super) fn rebuild_unique_field_indexes(&mut self) {
        self.entity_unique_field_index.clear();
        let tracked_fields = self.tracked_unique_entity_fields();
        if tracked_fields.is_empty() {
            return;
        }
        let state = self.current_state();
        let version_id = self.current_version_id();
        let mut rebuilt_values = Vec::new();
        for partition_id in state.partition_ids() {
            let partition = state
                .get_partition(partition_id)
                .expect("partition for unique field rebuild");
            for slot in 0..partition.entity_arena.generations.len() {
                if partition.entity_arena.lifecycle[slot] == RecordLifecycleState::Reusable {
                    continue;
                }
                let entity_id = crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    partition.entity_arena.generations[slot],
                );
                if let Some(payload) = crate::validation::logic::entity_payload_for_state(
                    &state, entity_id, version_id,
                ) {
                    for field in &tracked_fields {
                        if let Some(value) = payload
                            .as_json()
                            .and_then(|value| value.get(field))
                            .and_then(|value| value.as_str())
                        {
                            rebuilt_values.push((field.clone(), value.to_string(), entity_id));
                        }
                    }
                }
            }
        }
        for (field, value, entity_id) in rebuilt_values {
            self.entity_unique_field_index
                .entry(field)
                .or_default()
                .entry(value)
                .or_default()
                .insert(entity_id);
        }
    }

    #[cfg(test)]
    pub(crate) fn rebuild_unique_field_indexes_for_test(&mut self) {
        self.rebuild_unique_field_indexes();
    }

    fn tracked_unique_entity_fields(&self) -> BTreeSet<String> {
        let mut fields = BTreeSet::new();
        for rules in [
            &self.config.invariant_catalog.always_on_structural,
            &self.config.invariant_catalog.commit_boundary,
            &self.config.invariant_catalog.snapshot_audit,
            &self.config.invariant_catalog.harness_heavy,
        ] {
            for rule in rules {
                if let InvariantRule::UniqueEntityPayloadField(field) = rule {
                    fields.insert(field.clone());
                }
            }
        }
        fields
    }

    fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, SnapshotState) {
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        let handle = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let current_state = self.current_state();
        let entities = self.visible_entities_from_state(&current_state, version_id);
        let relations = self.visible_relations_from_state(&current_state, version_id);
        let pinned_entities = entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>();
        let pinned_relations = relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>();
        for entity_id in &pinned_entities {
            self.pin_entity(*entity_id);
        }
        for relation_id in &pinned_relations {
            self.pin_relation(*relation_id);
        }
        (
            handle.clone(),
            SnapshotState {
                handle,
                pinned_entities,
                pinned_relations,
            },
        )
    }

    pub(crate) fn pin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let slot = entity_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&entity_id.partition_id) else {
            return;
        };
        if slot >= partition.entity_arena.snapshot_pins.len() {
            return;
        }
        self.complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.entity_arena.snapshot_pins[slot] += 1;
        if partition.entity_arena.retired_at[slot].is_some() {
            partition.entity_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let slot = entity_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&entity_id.partition_id) else {
            return;
        };
        if slot >= partition.entity_arena.snapshot_pins.len()
            || partition.entity_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.entity_arena.snapshot_pins[slot] -= 1;
        let retired_at = partition.entity_arena.retired_at[slot];
        let retention_fence = self.retention_fence_version(self.current_version_id());
        self.refresh_entity_retention_state(
            entity_id.partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }

    pub(crate) fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let slot = relation_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&relation_id.partition_id) else {
            return;
        };
        if slot >= partition.relation_arena.snapshot_pins.len() {
            return;
        }
        self.complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.relation_arena.snapshot_pins[slot] += 1;
        if partition.relation_arena.retired_at[slot].is_some() {
            partition.relation_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let slot = relation_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&relation_id.partition_id) else {
            return;
        };
        if slot >= partition.relation_arena.snapshot_pins.len()
            || partition.relation_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.relation_arena.snapshot_pins[slot] -= 1;
        let retired_at = partition.relation_arena.retired_at[slot];
        let retention_fence = self.retention_fence_version(self.current_version_id());
        self.refresh_relation_retention_state(
            relation_id.partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }

    fn refresh_entity_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        let partition = self
            .partitions
            .get_mut(&partition_id)
            .expect("entity retention partition present");
        partition.entity_arena.lifecycle[slot] = match self.config.retention_policy.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => {
                if partition.entity_arena.snapshot_pins[slot] > 0 {
                    RecordLifecycleState::PinnedBySnapshot
                } else if partition.entity_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.entity_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if partition.entity_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.entity_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else if retired_at.is_some_and(|retired| retired <= retention_fence) {
                    RecordLifecycleState::Reclaimable
                } else {
                    RecordLifecycleState::PinnedBySnapshot
                }
            }
        };
    }

    fn refresh_relation_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        let partition = self
            .partitions
            .get_mut(&partition_id)
            .expect("relation retention partition present");
        partition.relation_arena.lifecycle[slot] = match self.config.retention_policy.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => {
                if partition.relation_arena.snapshot_pins[slot] > 0 {
                    RecordLifecycleState::PinnedBySnapshot
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if retired_at.is_some_and(|retired| retired <= retention_fence) {
                    RecordLifecycleState::Reclaimable
                } else {
                    RecordLifecycleState::PinnedBySnapshot
                }
            }
        };
    }

    pub(crate) fn trim_live_history_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        published_version: crate::identity::data::VersionId,
    ) {
        let oldest_pinned_version = self.retention_fence_version(published_version);

        let mut entity_slots = BTreeMap::new();
        let mut relation_slots = BTreeMap::new();
        for record in changed_records {
            match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    entity_slots
                        .entry(entity_id.partition_id)
                        .or_insert_with(BTreeSet::new)
                        .insert(entity_id.local_slot.0 as usize);
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    relation_slots
                        .entry(relation_id.partition_id)
                        .or_insert_with(BTreeSet::new)
                        .insert(relation_id.local_slot.0 as usize);
                }
            }
        }

        for (partition_id, slots) in entity_slots {
            let Some(partition) = self.partitions.get_mut(&partition_id) else {
                continue;
            };
            for slot in slots {
                if slot >= partition.entity_arena.payload_history.len()
                    || partition.entity_arena.lifecycle[slot] != RecordLifecycleState::Live
                {
                    continue;
                }
                let history = &mut partition.entity_arena.payload_history[slot];
                let original_len = history.len();
                history.retain(|entry| {
                    entry
                        .retired_at
                        .is_none_or(|retired| retired > oldest_pinned_version)
                });
                self.complexity_counters
                    .borrow_mut()
                    .live_entity_history_entries_trimmed +=
                    original_len.saturating_sub(history.len());
            }
        }

        for (partition_id, slots) in relation_slots {
            let Some(partition) = self.partitions.get_mut(&partition_id) else {
                continue;
            };
            for slot in slots {
                if !partition.relation_arena.payload_history.contains_key(&slot)
                    || partition.relation_arena.lifecycle[slot] != RecordLifecycleState::Live
                {
                    continue;
                }
                let history = partition
                    .relation_arena
                    .payload_history
                    .get_mut(&slot)
                    .expect("relation history present after key check");
                let original_len = history.len();
                history.retain(|entry| {
                    entry
                        .retired_at
                        .is_none_or(|retired| retired > oldest_pinned_version)
                });
                self.complexity_counters
                    .borrow_mut()
                    .live_relation_history_entries_trimmed +=
                    original_len.saturating_sub(history.len());
                if history.is_empty() {
                    partition.relation_arena.payload_history.remove(&slot);
                }
            }
        }
    }

    pub fn outgoing_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.adjacency.get(slot))
            .into_iter()
            .flat_map(|relations| relations.ids().into_iter())
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }

    pub fn incoming_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.reverse_adjacency.get(slot))
            .into_iter()
            .flat_map(|relations| relations.ids().into_iter())
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }

    pub(crate) fn compact_durable_log_if_needed(&mut self) {
        use crate::config::data::DurableLogRetentionMode;

        let policy = &self.config.durable_log_policy;
        if self.durable_log.len() <= policy.max_in_memory_envelopes {
            return;
        }

        match policy.retention_mode {
            DurableLogRetentionMode::RetainAllInMemory => {}
            DurableLogRetentionMode::CompactAfterCheckpoint => {
                if let Some(checkpoint) = self.durable_checkpoints.last() {
                    if let Some(commit) = checkpoint.up_to_commit.as_ref() {
                        self.durable_log
                            .retain(|entry| entry.envelope.commit.commit_id > commit.commit_id);
                    }
                }
                if self.durable_log.len() > policy.max_in_memory_envelopes {
                    let overflow = self.durable_log.len() - policy.max_in_memory_envelopes;
                    self.durable_log.drain(0..overflow);
                }
            }
        }
    }
}
