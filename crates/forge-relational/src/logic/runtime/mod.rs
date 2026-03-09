mod state;
mod transaction;
mod types;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::data::diagnostics::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::data::publication::{PublicationBundle, PublicationStatus};
use crate::data::query::QueryWorkPacket;
use crate::data::snapshot::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
use crate::data::transaction::TransactionOptions;

pub use transaction::RelationalTransaction;
pub use types::{
    EntityReadRecord, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule,
    PacketResult, RecordLifecycleState, RelationReadRecord, RelationalDiagnosticsFacade,
    RelationalReadView, RelationalReplayRecord, RelationalRuntimeConfig, ReplaySchemaVersion,
    StorageInvariantReport, StorageStats,
};

use self::state::{EntityArena, RelationArena, SnapshotState, WorkingState};
#[derive(Debug, Clone)]
pub struct RelationalRuntime {
    pub(super) config: RelationalRuntimeConfig,
    pub(super) entity_arena: EntityArena,
    pub(super) relation_arena: RelationArena,
    pub(super) adjacency: Vec<BTreeSet<crate::data::identity::RelationId>>,
    pub(super) snapshots: BTreeMap<SnapshotId, SnapshotState>,
    pub(super) diagnostics: Vec<RelationalDiagnosticArtifact>,
    pub(super) latest_publication_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
    pub(super) next_transaction_id: u64,
    pub(super) next_savepoint_id: u64,
    pub(super) next_commit_id: u64,
    pub(super) next_version_id: u64,
    pub(super) next_snapshot_id: u64,
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
        self.snapshots.get(&handle.snapshot_id).map(|state| RelationalReadView {
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
        self.snapshots.get(&handle.snapshot_id).map(|state| SnapshotInspectionSummary {
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

    pub(super) fn current_version_id(&self) -> crate::data::identity::VersionId {
        crate::data::identity::VersionId(self.next_version_id.saturating_sub(1))
    }

    pub(super) fn current_state(&self) -> WorkingState {
        WorkingState {
            entity_arena: self.entity_arena.clone(),
            relation_arena: self.relation_arena.clone(),
            adjacency: self.adjacency.clone(),
        }
    }

    pub(super) fn snapshot_state_for_current(
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

    pub(super) fn live_entities_from_state(
        &self,
        state: &WorkingState,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.entity_arena.generations.len() {
            if state.entity_arena.lifecycle[slot] != RecordLifecycleState::Live {
                continue;
            }
            let kind_id = state.entity_arena.kind_ids[slot].expect("kind id for live entity");
            let kind = self
                .config
                .schema_registry
                .resolve_entity(kind_id)
                .expect("kind resolution for live entity");
            let payload = state.entity_arena.payloads[slot]
                .clone()
                .expect("payload for live entity");
            records.push(EntityReadRecord {
                entity_id: crate::data::identity::EntityId::new(
                    slot as u64,
                    state.entity_arena.generations[slot],
                ),
                kind,
                lifecycle: state.entity_arena.lifecycle[slot],
                payload,
            });
        }
        records
    }

    pub(super) fn live_relations_from_state(
        &self,
        state: &WorkingState,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.relation_arena.generations.len() {
            if state.relation_arena.lifecycle[slot] != RecordLifecycleState::Live {
                continue;
            }
            let kind_id = state.relation_arena.kind_ids[slot].expect("kind id for live relation");
            let kind = self
                .config
                .schema_registry
                .resolve_relation(kind_id)
                .expect("kind resolution for live relation");
            let payload = state.relation_arena.payloads[slot]
                .clone()
                .expect("payload for live relation");
            let endpoints = state.relation_arena.endpoints[slot]
                .as_ref()
                .expect("endpoints for live relation");
            records.push(RelationReadRecord {
                relation_id: crate::data::identity::RelationId::new(
                    slot as u64,
                    state.relation_arena.generations[slot],
                ),
                kind,
                lifecycle: state.relation_arena.lifecycle[slot],
                source: endpoints.source,
                target: endpoints.target,
                payload,
            });
        }
        records
    }

    pub(super) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        self.diagnostics.push(artifact);
    }

    pub(super) fn push_bounded_diagnostic(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> RelationalDiagnosticArtifact {
        let max_entries = self.config.diagnostics.max_entries_per_artifact;
        let artifact = RelationalDiagnosticArtifact {
            scope,
            kind,
            determinism: DeterminismExpectation::Required,
            entries: entries.into_iter().take(max_entries).collect(),
        };
        self.push_diagnostic_artifact(artifact.clone());
        artifact
    }

    pub(super) fn assemble_publication_bundle(
        &mut self,
        staged: &WorkingState,
        commit_id: crate::data::history::CommitId,
        version_id: crate::data::identity::VersionId,
        patch: crate::data::diff::RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
    ) -> state::PublicationArtifacts {
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        let snapshot = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let replay = types::RelationalReplayRecord {
            schema_version: types::ReplaySchemaVersion(1),
            commit_id,
            version_id,
            snapshot_id,
            patch: patch.clone(),
            schema_registry: self.config.schema_registry.clone(),
        };
        let commit_reference = crate::data::history::CommitReference {
            commit_id,
            version_id,
            branch_id: self.config.main_branch.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot: snapshot.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            patch: patch.clone(),
            replay: replay.clone(),
            status: PublicationStatus::Published,
        };
        let snapshot_state = SnapshotState {
            handle: snapshot.clone(),
            entities: self.live_entities_from_state(staged),
            relations: self.live_relations_from_state(staged),
        };
        self.snapshots.insert(snapshot_id, snapshot_state);
        state::PublicationArtifacts {
            snapshot,
            diagnostics_summary,
            patch,
            replay,
            bundle,
        }
    }

    pub(super) fn run_invariants_for_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
        execution_point: InvariantExecutionPoint,
        include_harness_heavy: bool,
        merged_plan: Option<&crate::data::transaction::MergedCommitPlan>,
    ) -> Vec<InvariantCheckResult> {
        let mut results = Vec::new();
        let groups = match execution_point {
            InvariantExecutionPoint::MutationSensitive => vec![(
                InvariantClass::AlwaysOnStructural,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.always_on_structural,
            )],
            InvariantExecutionPoint::CommitBoundary => vec![(
                InvariantClass::CommitBoundary,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.commit_boundary,
            )],
            InvariantExecutionPoint::SnapshotPublication => vec![(
                InvariantClass::SnapshotAudit,
                InvariantFailureEffect::BlockPublication,
                &self.config.invariant_catalog.snapshot_audit,
            )],
            InvariantExecutionPoint::HarnessAudit => {
                if include_harness_heavy {
                    vec![(
                        InvariantClass::HarnessHeavy,
                        InvariantFailureEffect::AuditOnly,
                        &self.config.invariant_catalog.harness_heavy,
                    )]
                } else {
                    Vec::new()
                }
            }
        };

        let entity_records = self.live_entities_from_state(state);
        let relation_records = self.live_relations_from_state(state);

        for (class, failure_effect, rules) in groups {
            let mut violations = Vec::new();
            for rule in rules {
                match rule {
                    types::InvariantRule::LiveEntityRequiresKind => {
                        for slot in 0..state.entity_arena.generations.len() {
                            if state.entity_arena.lifecycle[slot] == RecordLifecycleState::Live
                                && state.entity_arena.kind_ids[slot].is_none()
                            {
                                violations.push(types::InvariantViolation {
                                    class,
                                    code: crate::data::diagnostics::DiagnosticCode::SidecarConsistencyFailure,
                                    detail: format!("live entity slot {} missing kind id", slot),
                                });
                            }
                        }
                    }
                    types::InvariantRule::LiveRelationRequiresEndpoints => {
                        for slot in 0..state.relation_arena.generations.len() {
                            if state.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
                                && state.relation_arena.endpoints[slot].is_none()
                            {
                                violations.push(types::InvariantViolation {
                                    class,
                                    code: crate::data::diagnostics::DiagnosticCode::SidecarConsistencyFailure,
                                    detail: format!("live relation slot {} missing endpoints", slot),
                                });
                            }
                        }
                    }
                    types::InvariantRule::MaxMergedIntents(limit) => {
                        let merged_len = merged_plan.map(|plan| plan.merged_intents.len()).unwrap_or(0);
                        if merged_len > *limit {
                            violations.push(types::InvariantViolation {
                                class,
                                code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "merged commit plan has {} intents, limit is {}",
                                    merged_len, limit
                                ),
                            });
                        }
                    }
                    types::InvariantRule::MaxSnapshotEntities(limit) => {
                        if entity_records.len() > *limit {
                            violations.push(types::InvariantViolation {
                                class,
                                code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "snapshot at version {} has {} entities, limit is {}",
                                    version_id.0, entity_records.len(), limit
                                ),
                            });
                        }
                    }
                    types::InvariantRule::UniqueEntityPayloadField(field) => {
                        let mut seen = BTreeSet::new();
                        for entity in &entity_records {
                            if let Some(value) = entity.payload.get(field).and_then(|value| value.as_str()) {
                                if !seen.insert(value.to_string()) {
                                    violations.push(types::InvariantViolation {
                                        class,
                                        code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
                                        detail: format!(
                                            "duplicate entity payload field {}={}",
                                            field, value
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if !relation_records.is_empty() || !entity_records.is_empty() || !rules.is_empty() {
                results.push(types::InvariantCheckResult {
                    class,
                    execution_point,
                    failure_effect,
                    violations,
                });
            }
        }

        results
    }
}

pub(super) fn first_blocking_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<crate::data::transaction::CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| crate::data::transaction::CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn first_publication_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<crate::data::transaction::CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockPublication
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| crate::data::transaction::CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn schema_error_to_commit_conflict(
    error: crate::data::schema::SchemaRegistryError,
) -> crate::data::transaction::CommitConflict {
    crate::data::transaction::CommitConflict {
        code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
        detail: format!("{error:?}"),
    }
}

pub(super) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: json!({ "execution_point": "snapshot_publication" }),
    }
}
