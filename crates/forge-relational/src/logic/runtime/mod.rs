mod apply;
mod invariants;
mod merge;
mod publication;
mod read;
mod state;
mod transaction;
mod types;

use std::collections::{BTreeMap, BTreeSet};

use crate::data::publication::PublicationBundle;
use crate::data::query::QueryWorkPacket;
use crate::data::snapshot::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
use crate::data::transaction::TransactionOptions;

pub use transaction::RelationalTransaction;
pub use types::{
    EntityReadRecord, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, InvariantViolation,
    PacketResult, RecordLifecycleState, RelationReadRecord, RelationalDiagnosticsFacade,
    RelationalReadView, RelationalReplayRecord, RelationalRuntimeConfig, ReplaySchemaVersion,
    StorageInvariantReport, StorageStats,
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

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        self.snapshots
            .get(&handle.snapshot_id)
            .map(|state| RelationalReadView {
                snapshot: state.handle.clone(),
                entities: state.entities.clone(),
                relations: state.relations.clone(),
            })
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

    pub fn inspect_snapshot(&self, handle: &SnapshotHandle) -> Option<SnapshotInspectionSummary> {
        self.snapshots
            .get(&handle.snapshot_id)
            .map(|state| SnapshotInspectionSummary {
                version_id: state.handle.version_id,
                entity_count: state.entities.len(),
                relation_count: state.relations.len(),
            })
    }

    pub fn storage_stats(&self) -> StorageStats {
        StorageStats {
            entity_slots: self.entity_arena.generations.len(),
            live_entities: self
                .entity_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Live)
                .count(),
            reusable_entity_slots: self
                .entity_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Reusable)
                .count(),
            relation_slots: self.relation_arena.generations.len(),
            live_relations: self
                .relation_arena
                .lifecycle
                .iter()
                .filter(|state| **state == RecordLifecycleState::Live)
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

    fn current_version_id(&self) -> crate::data::identity::VersionId {
        crate::data::identity::VersionId(self.next_version_id.saturating_sub(1))
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
        let entities = self.live_entities_from_state(&current_state);
        let relations = self.live_relations_from_state(&current_state);
        (
            handle.clone(),
            SnapshotState {
                handle,
                entities,
                relations,
            },
        )
    }
}
