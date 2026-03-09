mod apply;
mod chunks;
mod durability;
mod indexes;
mod invariants;
mod lineage;
mod merge;
mod publication;
mod read;
mod replay;
mod state;
mod transaction;
mod types;

use std::collections::{BTreeMap, BTreeSet};

use crate::data::durability::{DurableCheckpoint, DurableCommitEnvelope};
use crate::data::history::{
    BranchCreateError, BranchHead, BranchId, VersionGraphSnapshot, VersionNode,
};
use crate::data::index::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::data::lineage::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::data::publication::PublicationBundle;
use crate::data::query::QueryWorkPacket;
use crate::data::replay::CanonicalCommitEnvelope;
use crate::data::snapshot::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
use crate::data::transaction::TransactionOptions;

pub use transaction::RelationalTransaction;
pub use types::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, EntityReadRecord,
    IndexedReadOutcome, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, InvariantViolation,
    PacketResult, RecordLifecycleState, RecoveryOutcome, RelationReadRecord,
    RelationalDiagnosticsFacade, RelationalReadView, RelationalReplayRecord,
    RelationalRuntimeConfig, ReplaySchemaVersion, RetentionPassOutcome, StorageInvariantReport,
    StorageStats,
};

use self::state::{EntityArena, RelationArena, SnapshotState, WorkingState};
#[derive(Debug, Clone)]
pub struct RelationalRuntime {
    config: RelationalRuntimeConfig,
    entity_arena: EntityArena,
    relation_arena: RelationArena,
    adjacency: Vec<BTreeSet<crate::data::identity::RelationId>>,
    snapshots: BTreeMap<SnapshotId, SnapshotState>,
    diagnostics: Vec<crate::data::diagnostics::RelationalDiagnosticArtifact>,
    latest_publication_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
    branch_heads: BTreeMap<BranchId, Option<crate::data::history::CommitReference>>,
    commit_graph: BTreeMap<crate::data::history::CommitId, VersionNode>,
    commit_envelopes: BTreeMap<crate::data::history::CommitId, CanonicalCommitEnvelope>,
    index_definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    index_generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    lineage_nodes: BTreeMap<crate::data::identity::LineageId, LineageNode>,
    lineage_events: Vec<LineageEventRecord>,
    correspondence_candidates: Vec<CorrespondenceCandidate>,
    durable_log: Vec<DurableCommitEnvelope>,
    durable_checkpoints: Vec<DurableCheckpoint>,
    next_index_id: u64,
    next_index_generation_id: u64,
    next_lineage_id: u64,
    next_lineage_event_id: u64,
    next_transaction_id: u64,
    next_savepoint_id: u64,
    next_commit_id: u64,
    next_version_id: u64,
    next_snapshot_id: u64,
}

impl RelationalRuntime {
    pub fn new(config: RelationalRuntimeConfig) -> Self {
        Self {
            entity_arena: EntityArena::with_capacity(config.initial_entity_capacity),
            relation_arena: RelationArena::with_capacity(config.initial_relation_capacity),
            adjacency: Vec::with_capacity(config.initial_entity_capacity),
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
            config,
        }
    }

    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id = crate::data::transaction::TransactionId(self.next_transaction_id);
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
        self.restore_snapshot_pin_counters();
        if self.config.mvcc.snapshot_release_policy
            == crate::data::config::SnapshotReleasePolicy::ReleaseOnRetentionPass
        {
            self.run_retention_pass();
        }
        true
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        self.snapshots.get(&handle.snapshot_id).map(|state| {
            let current_state = self.current_state();
            RelationalReadView {
                snapshot: state.handle.clone(),
                entities: self.visible_entities_from_state(&current_state, state.handle.version_id),
                relations: self
                    .visible_relations_from_state(&current_state, state.handle.version_id),
            }
        })
    }

    pub fn read_version(&self, version_id: crate::data::identity::VersionId) -> RelationalReadView {
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

    pub fn latest_patch(&self) -> Option<&crate::data::diff::RelationalPatchRecord> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.replay)
    }

    pub fn latest_commit(&self) -> Option<&crate::data::history::CommitReference> {
        self.latest_publication_bundle
            .as_ref()
            .map(|bundle| &bundle.commit)
    }

    pub fn branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Option<&crate::data::history::CommitReference> {
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

    pub fn version_graph(&self) -> VersionGraphSnapshot {
        VersionGraphSnapshot {
            branches: self.branches(),
            commits: self.commit_graph.values().cloned().collect(),
        }
    }

    pub fn ancestor_chain(
        &self,
        commit_id: crate::data::history::CommitId,
    ) -> Vec<crate::data::history::CommitId> {
        let mut ordered = self.ancestor_set(commit_id).into_iter().collect::<Vec<_>>();
        ordered.sort_by_key(|id| id.0);
        ordered
    }

    pub fn latest_common_ancestor_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> Option<crate::data::history::CommitId> {
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
            entity_slots: self.entity_arena.generations.len(),
            entity_chunks: chunked_summary.entity_chunks.len(),
            live_entities: self
                .entity_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Live)
                .count(),
            deleted_entities: self
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
            reusable_entity_slots: self
                .entity_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Reusable)
                .count(),
            relation_slots: self.relation_arena.generations.len(),
            relation_chunks: chunked_summary.relation_chunks.len(),
            live_relations: self
                .relation_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Live)
                .count(),
            deleted_relations: self
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
            reusable_relation_slots: self
                .relation_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Reusable)
                .count(),
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
        let chunk_summary = self.chunked_storage_summary(self.current_version_id());
        let mut outcome = RetentionPassOutcome {
            entity_reclaimable: 0,
            entity_reclaimed: 0,
            entity_chunks_scanned: 0,
            relation_reclaimable: 0,
            relation_reclaimed: 0,
            relation_chunks_scanned: 0,
        };

        for chunk in chunk_summary
            .entity_chunks
            .iter()
            .filter(|chunk| chunk.retained_records > 0 || chunk.reclaimable_records > 0)
        {
            outcome.entity_chunks_scanned += 1;
            let slot_end =
                (chunk.slot_start + chunk.slot_len).min(self.entity_arena.lifecycle.len());
            for slot in chunk.slot_start..slot_end {
                if let Some(version) = self.entity_arena.retired_at[slot] {
                    self.refresh_entity_retention_state(slot, Some(version));
                    if self.entity_arena.lifecycle[slot] == RecordLifecycleState::Reclaimable {
                        outcome.entity_reclaimable += 1;
                        if self.config.mvcc.auto_reclaim_deleted_records
                            && outcome.entity_reclaimed < self.config.mvcc.reclaim_batch_size
                        {
                            self.entity_arena.lifecycle[slot] = RecordLifecycleState::Reusable;
                            self.entity_arena.kind_ids[slot] = None;
                            self.entity_arena.payloads[slot] = None;
                            self.entity_arena.snapshot_pins[slot] = 0;
                            self.entity_arena.branch_pins[slot] = 0;
                            self.entity_arena.replay_pins[slot] = 0;
                            self.entity_arena.retired_at[slot] = None;
                            self.entity_arena.free_list.push(slot as u64);
                            outcome.entity_reclaimed += 1;
                        }
                    }
                }
            }
        }

        for chunk in chunk_summary
            .relation_chunks
            .iter()
            .filter(|chunk| chunk.retained_records > 0 || chunk.reclaimable_records > 0)
        {
            outcome.relation_chunks_scanned += 1;
            let slot_end =
                (chunk.slot_start + chunk.slot_len).min(self.relation_arena.lifecycle.len());
            for slot in chunk.slot_start..slot_end {
                if let Some(version) = self.relation_arena.retired_at[slot] {
                    self.refresh_relation_retention_state(slot, Some(version));
                    if self.relation_arena.lifecycle[slot] == RecordLifecycleState::Reclaimable {
                        outcome.relation_reclaimable += 1;
                        if self.config.mvcc.auto_reclaim_deleted_records
                            && outcome.relation_reclaimed < self.config.mvcc.reclaim_batch_size
                        {
                            self.relation_arena.lifecycle[slot] = RecordLifecycleState::Reusable;
                            self.relation_arena.kind_ids[slot] = None;
                            self.relation_arena.payloads[slot] = None;
                            self.relation_arena.snapshot_pins[slot] = 0;
                            self.relation_arena.endpoints[slot] = None;
                            self.relation_arena.retired_at[slot] = None;
                            self.relation_arena.free_list.push(slot as u64);
                            outcome.relation_reclaimed += 1;
                        }
                    }
                }
            }
        }

        outcome
    }

    fn current_version_id(&self) -> crate::data::identity::VersionId {
        crate::data::identity::VersionId(self.next_version_id.saturating_sub(1))
    }

    pub(super) fn primary_schema_version(&self) -> crate::data::schema::SchemaVersionId {
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
            .unwrap_or(crate::data::schema::SchemaVersionId(0))
    }

    pub(super) fn latest_common_ancestor(
        &self,
        left: crate::data::history::CommitId,
        right: crate::data::history::CommitId,
    ) -> Option<crate::data::history::CommitId> {
        let left_ancestors = self.ancestor_set(left);
        let right_ancestors = self.ancestor_set(right);
        left_ancestors
            .intersection(&right_ancestors)
            .copied()
            .max_by_key(|commit_id| commit_id.0)
    }

    fn ancestor_set(
        &self,
        start: crate::data::history::CommitId,
    ) -> std::collections::BTreeSet<crate::data::history::CommitId> {
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

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::data::history::CommitId,
    ) -> bool {
        self.commit_envelopes.remove(&commit_id).is_some()
    }

    fn current_state(&self) -> WorkingState {
        WorkingState {
            entity_arena: self.entity_arena.clone(),
            relation_arena: self.relation_arena.clone(),
            adjacency: self.adjacency.clone(),
        }
    }

    fn snapshot_state_for_current(
        &mut self,
        version_id: crate::data::identity::VersionId,
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

    fn pin_entity(&mut self, entity_id: crate::data::identity::EntityId) {
        let slot = entity_id.slot.0 as usize;
        if slot >= self.entity_arena.snapshot_pins.len() {
            return;
        }
        self.entity_arena.snapshot_pins[slot] += 1;
        if self.entity_arena.retired_at[slot].is_some() {
            self.entity_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    fn unpin_entity(&mut self, entity_id: crate::data::identity::EntityId) {
        let slot = entity_id.slot.0 as usize;
        if slot >= self.entity_arena.snapshot_pins.len()
            || self.entity_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.entity_arena.snapshot_pins[slot] -= 1;
        self.refresh_entity_retention_state(slot, self.entity_arena.retired_at[slot]);
    }

    fn pin_relation(&mut self, relation_id: crate::data::identity::RelationId) {
        let slot = relation_id.slot.0 as usize;
        if slot >= self.relation_arena.snapshot_pins.len() {
            return;
        }
        self.relation_arena.snapshot_pins[slot] += 1;
        if self.relation_arena.retired_at[slot].is_some() {
            self.relation_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    fn unpin_relation(&mut self, relation_id: crate::data::identity::RelationId) {
        let slot = relation_id.slot.0 as usize;
        if slot >= self.relation_arena.snapshot_pins.len()
            || self.relation_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.relation_arena.snapshot_pins[slot] -= 1;
        self.refresh_relation_retention_state(slot, self.relation_arena.retired_at[slot]);
    }

    fn refresh_entity_retention_state(
        &mut self,
        slot: usize,
        retired_at: Option<crate::data::identity::VersionId>,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        self.entity_arena.lifecycle[slot] = if self.entity_arena.snapshot_pins[slot] > 0 {
            RecordLifecycleState::PinnedBySnapshot
        } else if self.entity_arena.branch_pins[slot] > 0 {
            RecordLifecycleState::PinnedByBranch
        } else if self.entity_arena.replay_pins[slot] > 0 {
            RecordLifecycleState::PinnedByReplayRetention
        } else {
            RecordLifecycleState::Reclaimable
        };
    }

    fn refresh_relation_retention_state(
        &mut self,
        slot: usize,
        retired_at: Option<crate::data::identity::VersionId>,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        self.relation_arena.lifecycle[slot] = if self.relation_arena.snapshot_pins[slot] > 0 {
            RecordLifecycleState::PinnedBySnapshot
        } else {
            RecordLifecycleState::Reclaimable
        };
    }

    fn restore_snapshot_pin_counters(&mut self) {
        for pin in &mut self.entity_arena.snapshot_pins {
            *pin = 0;
        }
        for pin in &mut self.relation_arena.snapshot_pins {
            *pin = 0;
        }

        let entity_pins = self
            .snapshots
            .values()
            .flat_map(|state| state.pinned_entities.iter().copied())
            .collect::<Vec<_>>();
        let relation_pins = self
            .snapshots
            .values()
            .flat_map(|state| state.pinned_relations.iter().copied())
            .collect::<Vec<_>>();

        for entity_id in entity_pins {
            self.pin_entity(entity_id);
        }
        for relation_id in relation_pins {
            self.pin_relation(relation_id);
        }
    }
}
