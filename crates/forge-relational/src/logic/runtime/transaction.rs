use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::data::diagnostics::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry};
use crate::data::diff::{PatchOrdering, PatchPublicationMode, PatchRecord, PatchRecordKind, PatchStreamPosition, RelationalPatchRecord};
use crate::data::history::CommitId;
use crate::data::identity::{EntityId, KindId, RelationId, VersionId};
use crate::data::publication::{PublicationError, PublicationStage, PublicationStatus};
use crate::data::schema::RelationalSchemaRegistry;
use crate::data::transaction::{
    AuthoritativeApplyPlan, CommitConflict, CommitOutcome, MergedCommitPlan, RecordRef,
    RelationSpec, RollbackOutcome, SavepointId, TransactionCommitError, TransactionIntent,
    TransactionOptions, WorkerIntentBatch,
};

use super::{
    first_blocking_invariant_error, first_publication_invariant_error, publication_failure_diagnostic,
    schema_error_to_commit_conflict, InvariantExecutionPoint, RecordLifecycleState,
    RelationalDiagnosticArtifact, RelationalRuntime,
};
use super::state::{RelationEndpoints, WorkingState};

#[derive(Debug)]
pub struct RelationalTransaction<'a> {
    pub(super) runtime: &'a mut RelationalRuntime,
    pub(super) transaction_id: crate::data::transaction::TransactionId,
    pub(super) options: TransactionOptions,
    pub(super) batches: Vec<WorkerIntentBatch>,
    pub(super) savepoints: Vec<(SavepointId, usize)>,
    pub(super) last_merged_plan: Option<MergedCommitPlan>,
}

impl<'a> RelationalTransaction<'a> {
    pub fn transaction_id(&self) -> crate::data::transaction::TransactionId {
        self.transaction_id
    }

    pub fn push_batch(&mut self, batch: WorkerIntentBatch) {
        self.batches.push(batch);
    }

    pub fn create_savepoint(&mut self) -> SavepointId {
        assert!(
            self.options.allow_nested_savepoints,
            "nested savepoints are disabled for this transaction"
        );
        let savepoint_id = SavepointId(self.runtime.next_savepoint_id);
        self.runtime.next_savepoint_id += 1;
        self.savepoints.push((savepoint_id, self.batches.len()));
        savepoint_id
    }

    pub fn rollback_to_savepoint(
        &mut self,
        savepoint_id: SavepointId,
    ) -> Result<RollbackOutcome, CommitConflict> {
        let Some(index) = self
            .savepoints
            .iter()
            .position(|(candidate, _)| *candidate == savepoint_id)
        else {
            return Err(CommitConflict {
                code: DiagnosticCode::InvalidSavepoint,
                detail: format!("savepoint {:?} does not exist", savepoint_id),
            });
        };
        let (_, batch_len) = self.savepoints[index];
        let drained = self.batches.split_off(batch_len);
        self.savepoints.truncate(index);
        let restored_records = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| match intent {
                TransactionIntent::CreateEntity(_) => RecordRef::Entity(EntityId::new(u64::MAX, 0)),
                TransactionIntent::UpdateEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::DeleteEntity { entity_id } => RecordRef::Entity(entity_id),
                TransactionIntent::CreateRelation(_) => RecordRef::Relation(RelationId::new(u64::MAX, 0)),
                TransactionIntent::DeleteRelation { relation_id } => RecordRef::Relation(relation_id),
            })
            .collect();
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::Rollback,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::SavepointRolledBack,
                message: "rolled back to savepoint".to_string(),
                fields: json!({ "savepoint_id": savepoint_id.0 }),
            }],
        );
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            restored_records,
        })
    }

    pub fn merged_plan(&mut self) -> Result<&MergedCommitPlan, CommitConflict> {
        if self.last_merged_plan.is_none() {
            let plan = self.build_merged_plan()?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub fn commit(self) -> Result<CommitOutcome, TransactionCommitError> {
        let merged_plan = self.build_merged_plan().map_err(TransactionCommitError::Conflict)?;
        let current_state = self.runtime.current_state();
        let commit_boundary_results = self.runtime.run_invariants_for_state(
            &current_state,
            self.runtime.current_version_id(),
            InvariantExecutionPoint::CommitBoundary,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&commit_boundary_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: error.detail.clone(),
                    fields: json!({ "execution_point": "commit_boundary" }),
                }],
            );
            return Err(TransactionCommitError::Conflict(error));
        }

        let version_id = VersionId(self.runtime.next_version_id);
        let apply_plan = AuthoritativeApplyPlan {
            transaction_id: self.transaction_id,
            version_id,
            merged_intents: merged_plan.merged_intents.clone(),
        };
        let mut staged = current_state.clone();
        let (changed_records, patch_records, diagnostics_entries) = apply_plan_to_staged_state(&mut staged, &apply_plan);

        let structural_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::MutationSensitive,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&structural_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: error.detail.clone(),
                    fields: json!({ "execution_point": "mutation_sensitive" }),
                }],
            );
            return Err(TransactionCommitError::Conflict(error));
        }

        let commit_id = CommitId(self.runtime.next_commit_id);
        let patch = RelationalPatchRecord {
            ordering: PatchOrdering::CanonicalCommitOrder,
            publication_mode: PatchPublicationMode::CommitNative,
            position: PatchStreamPosition(commit_id.0),
            records: patch_records,
        };
        let diagnostics_summary = RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            determinism: crate::data::diagnostics::DeterminismExpectation::Required,
            entries: diagnostics_entries
                .into_iter()
                .take(self.runtime.config.diagnostics.max_entries_per_artifact)
                .collect(),
        };

        let snapshot_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::SnapshotPublication,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_publication_invariant_error(&snapshot_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(error.detail.clone())],
            );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::InvariantCheck,
                detail: error.detail,
            }));
        }

        let artifacts = self.runtime.assemble_publication_bundle(
            &staged,
            commit_id,
            version_id,
            patch.clone(),
            diagnostics_summary.clone(),
        );

        self.runtime.entity_arena = staged.entity_arena;
        self.runtime.relation_arena = staged.relation_arena;
        self.runtime.adjacency = staged.adjacency;
        self.runtime.next_commit_id += 1;
        self.runtime.next_version_id += 1;
        self.runtime.latest_publication_bundle = Some(artifacts.bundle.clone());
        self.runtime.push_diagnostic_artifact(artifacts.diagnostics_summary);
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "commit published coherently".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "snapshot_id": artifacts.snapshot.snapshot_id.0
                }),
            }],
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            version_id,
            snapshot: artifacts.snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
        })
    }

    fn build_merged_plan(&self) -> Result<MergedCommitPlan, CommitConflict> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        for intent in &intents {
            validate_intent(&self.runtime.current_state(), &self.runtime.config.schema_registry, intent)?;
        }
        intents.sort_by_key(canonical_intent_key);
        let mut seen_updates = BTreeSet::new();
        for intent in &intents {
            match intent {
                TransactionIntent::UpdateEntity { entity_id, .. }
                | TransactionIntent::DeleteEntity { entity_id } => {
                    if !seen_updates.insert(format!("entity:{}:{}", entity_id.slot.0, entity_id.generation.0)) {
                        return Err(CommitConflict {
                            code: DiagnosticCode::ConflictingIntent,
                            detail: format!("conflicting entity intent for slot {}", entity_id.slot.0),
                        });
                    }
                }
                TransactionIntent::DeleteRelation { relation_id } => {
                    if !seen_updates.insert(format!("relation:{}:{}", relation_id.slot.0, relation_id.generation.0)) {
                        return Err(CommitConflict {
                            code: DiagnosticCode::ConflictingIntent,
                            detail: format!("conflicting relation intent for slot {}", relation_id.slot.0),
                        });
                    }
                }
                TransactionIntent::CreateEntity(_) | TransactionIntent::CreateRelation(_) => {}
            }
        }
        Ok(MergedCommitPlan {
            transaction_id: self.transaction_id,
            merged_intents: intents,
        })
    }
}

fn canonical_intent_key(intent: &TransactionIntent) -> (u8, String) {
    match intent {
        TransactionIntent::CreateEntity(spec) => (0, format!("{:010}:{}", spec.kind_id.0, spec.client_key)),
        TransactionIntent::UpdateEntity { entity_id, .. } => (
            1,
            format!("{:020}:{:010}", entity_id.slot.0, entity_id.generation.0),
        ),
        TransactionIntent::DeleteEntity { entity_id } => (
            2,
            format!("{:020}:{:010}", entity_id.slot.0, entity_id.generation.0),
        ),
        TransactionIntent::CreateRelation(spec) => (
            3,
            format!(
                "{:010}:{:020}:{:020}:{}",
                spec.kind_id.0, spec.source.slot.0, spec.target.slot.0, spec.client_key
            ),
        ),
        TransactionIntent::DeleteRelation { relation_id } => (
            4,
            format!("{:020}:{:010}", relation_id.slot.0, relation_id.generation.0),
        ),
    }
}

fn validate_intent(
    state: &WorkingState,
    schema_registry: &RelationalSchemaRegistry,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    match intent {
        TransactionIntent::CreateEntity(spec) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::UpdateEntity { entity_id, .. } | TransactionIntent::DeleteEntity { entity_id } => {
            if entity_exists_in_state(state, *entity_id) {
                Ok(())
            } else {
                Err(CommitConflict {
                    code: DiagnosticCode::StaleHandle,
                    detail: format!("entity {:?} is stale or absent", entity_id),
                })
            }
        }
        TransactionIntent::CreateRelation(spec) => {
            schema_registry
                .resolve_relation(spec.kind_id)
                .map_err(schema_error_to_commit_conflict)?;
            if !entity_exists_in_state(state, spec.source) || !entity_exists_in_state(state, spec.target) {
                return Err(CommitConflict {
                    code: DiagnosticCode::InvalidRelationEndpoint,
                    detail: "relation endpoints must be live entities".to_string(),
                });
            }
            for slot in 0..state.relation_arena.generations.len() {
                if state.relation_arena.lifecycle[slot] != RecordLifecycleState::Live {
                    continue;
                }
                let Some(endpoints) = state.relation_arena.endpoints[slot].as_ref() else {
                    continue;
                };
                let same_kind = state.relation_arena.kind_ids[slot] == Some(spec.kind_id);
                let same_endpoints = endpoints.source == spec.source && endpoints.target == spec.target;
                if same_kind && same_endpoints {
                    return Err(CommitConflict {
                        code: DiagnosticCode::DuplicateRelationIdentity,
                        detail: "duplicate relation identity".to_string(),
                    });
                }
            }
            Ok(())
        }
        TransactionIntent::DeleteRelation { relation_id } => {
            if relation_exists_in_state(state, *relation_id) {
                Ok(())
            } else {
                Err(CommitConflict {
                    code: DiagnosticCode::StaleHandle,
                    detail: format!("relation {:?} is stale or absent", relation_id),
                })
            }
        }
    }
}

fn apply_plan_to_staged_state(
    staged: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
) -> (Vec<RecordRef>, Vec<PatchRecord>, Vec<RelationalDiagnosticsEntry>) {
    let mut changed_records = Vec::new();
    let mut patch_records = Vec::new();
    let mut diagnostics = Vec::new();

    for intent in &apply_plan.merged_intents {
        match intent.clone() {
            TransactionIntent::CreateEntity(spec) => {
                let entity_id = allocate_entity(staged, spec.kind_id, spec.payload.clone());
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityCreated,
                    message: "entity created".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0, "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityCreated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({ "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
            }
            TransactionIntent::UpdateEntity { entity_id, payload } => {
                let slot = entity_id.slot.0 as usize;
                staged.entity_arena.payloads[slot] = Some(payload);
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityUpdated,
                    message: "entity updated".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0 }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityUpdated,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({}),
                });
            }
            TransactionIntent::DeleteEntity { entity_id } => {
                let slot = entity_id.slot.0 as usize;
                staged.entity_arena.lifecycle[slot] = RecordLifecycleState::Reusable;
                staged.entity_arena.free_list.push(entity_id.slot.0);
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::EntityDeleted,
                    message: "entity deleted".to_string(),
                    fields: json!({ "entity_slot": entity_id.slot.0 }),
                });
                changed_records.push(RecordRef::Entity(entity_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::EntityDeleted,
                    entity_id: Some(entity_id),
                    relation_id: None,
                    detail: json!({}),
                });
            }
            TransactionIntent::CreateRelation(spec) => {
                let relation_id = allocate_relation(staged, &spec);
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationCreated,
                    message: "relation created".to_string(),
                    fields: json!({ "relation_slot": relation_id.slot.0, "source_slot": spec.source.slot.0, "target_slot": spec.target.slot.0, "kind_id": spec.kind_id.0 }),
                });
                changed_records.push(RecordRef::Relation(relation_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RelationCreated,
                    entity_id: None,
                    relation_id: Some(relation_id),
                    detail: json!({ "kind_id": spec.kind_id.0, "client_key": spec.client_key }),
                });
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                let slot = relation_id.slot.0 as usize;
                staged.relation_arena.lifecycle[slot] = RecordLifecycleState::Reusable;
                staged.relation_arena.free_list.push(relation_id.slot.0);
                if let Some(endpoints) = staged.relation_arena.endpoints[slot].as_ref() {
                    staged.adjacency[endpoints.source.slot.0 as usize].remove(&relation_id);
                }
                diagnostics.push(RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RelationDeleted,
                    message: "relation deleted".to_string(),
                    fields: json!({ "relation_slot": relation_id.slot.0 }),
                });
                changed_records.push(RecordRef::Relation(relation_id));
                patch_records.push(PatchRecord {
                    kind: PatchRecordKind::RelationDeleted,
                    entity_id: None,
                    relation_id: Some(relation_id),
                    detail: json!({}),
                });
            }
        }
    }

    (changed_records, patch_records, diagnostics)
}

fn allocate_entity(staged: &mut WorkingState, kind_id: KindId, payload: serde_json::Value) -> EntityId {
    if let Some(slot) = staged.entity_arena.free_list.pop() {
        let idx = slot as usize;
        staged.entity_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.entity_arena.kind_ids[idx] = Some(kind_id);
        staged.entity_arena.payloads[idx] = Some(payload);
        staged.entity_arena.generations[idx] += 1;
        return EntityId::new(slot, staged.entity_arena.generations[idx]);
    }
    let slot = staged.entity_arena.generations.len() as u64;
    staged.entity_arena.generations.push(1);
    staged.entity_arena.lifecycle.push(RecordLifecycleState::Live);
    staged.entity_arena.kind_ids.push(Some(kind_id));
    staged.entity_arena.payloads.push(Some(payload));
    staged.entity_arena.aspect_versions.push(BTreeMap::new());
    staged.entity_arena.structural_fingerprints.push(None);
    staged.entity_arena.lineage_ids.push(None);
    staged.entity_arena.diagnostics_enrichment.push(BTreeMap::new());
    staged.entity_arena.branch_pins.push(0);
    staged.entity_arena.replay_pins.push(0);
    staged.entity_arena.snapshot_pins.push(0);
    staged.adjacency.push(BTreeSet::new());
    EntityId::new(slot, 1)
}

fn allocate_relation(staged: &mut WorkingState, spec: &RelationSpec) -> RelationId {
    if let Some(slot) = staged.relation_arena.free_list.pop() {
        let idx = slot as usize;
        staged.relation_arena.lifecycle[idx] = RecordLifecycleState::Live;
        staged.relation_arena.kind_ids[idx] = Some(spec.kind_id);
        staged.relation_arena.payloads[idx] = Some(spec.payload.clone());
        staged.relation_arena.endpoints[idx] = Some(RelationEndpoints {
            source: spec.source,
            target: spec.target,
        });
        staged.relation_arena.generations[idx] += 1;
        let relation_id = RelationId::new(slot, staged.relation_arena.generations[idx]);
        staged.adjacency[spec.source.slot.0 as usize].insert(relation_id);
        return relation_id;
    }
    let slot = staged.relation_arena.generations.len() as u64;
    staged.relation_arena.generations.push(1);
    staged.relation_arena.lifecycle.push(RecordLifecycleState::Live);
    staged.relation_arena.kind_ids.push(Some(spec.kind_id));
    staged.relation_arena.payloads.push(Some(spec.payload.clone()));
    staged.relation_arena.endpoints.push(Some(RelationEndpoints {
        source: spec.source,
        target: spec.target,
    }));
    staged.relation_arena.diagnostics_enrichment.push(BTreeMap::new());
    let relation_id = RelationId::new(slot, 1);
    staged.adjacency[spec.source.slot.0 as usize].insert(relation_id);
    relation_id
}

fn entity_exists_in_state(state: &WorkingState, entity_id: EntityId) -> bool {
    let slot = entity_id.slot.0 as usize;
    state.entity_arena.generations.get(slot) == Some(&entity_id.generation.0)
        && state.entity_arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
}

fn relation_exists_in_state(state: &WorkingState, relation_id: RelationId) -> bool {
    let slot = relation_id.slot.0 as usize;
    state.relation_arena.generations.get(slot) == Some(&relation_id.generation.0)
        && state.relation_arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
}
