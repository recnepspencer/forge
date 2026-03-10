use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{CommitId, CommitReference};
use crate::identity::data::{EntityId, PartitionId, RelationId, VersionId};
use crate::publication::data::diff::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    RelationalPatchRecord,
};
use crate::publication::data::{PublicationError, PublicationStage, PublicationStatus};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::symbols::data::{InternedString, SymbolPolicy};
use crate::transactions::data::{
    AuthoritativeApplyPlan, CommitConflict, CommitOutcome, MergedCommitPlan, RecordRef,
    RollbackOutcome, SavepointId, TransactionCommitError, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use crate::logic::runtime::apply::apply_plan_to_staged_state;
use crate::logic::runtime::merge::{
    canonical_intent_key, detect_conflicting_updates, validate_intent,
};
use crate::logic::runtime::{
    InvariantExecutionPoint, PartitionAccess, RelationalRuntime, WorkingState,
};
use crate::publication::logic::publication_failure_diagnostic;
use crate::validation::logic::{first_blocking_invariant_error, first_publication_invariant_error};

#[derive(Debug)]
pub struct RelationalTransaction<'a> {
    pub(crate) runtime: &'a mut RelationalRuntime,
    pub(crate) transaction_id: crate::transactions::data::TransactionId,
    pub(crate) options: TransactionOptions,
    pub(crate) batches: Vec<WorkerIntentBatch>,
    pub(crate) savepoints: Vec<(SavepointId, usize)>,
    pub(crate) last_merged_plan: Option<MergedCommitPlan>,
}

impl<'a> RelationalTransaction<'a> {
    pub fn transaction_id(&self) -> crate::transactions::data::TransactionId {
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
                TransactionIntent::CreateEntity(_)
                | TransactionIntent::BulkCreateEntities { .. } => {
                    RecordRef::Entity(EntityId::new(PartitionId::main(), u64::MAX, 0))
                }
                TransactionIntent::UpdateEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::ReplaceEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::DeleteEntity { entity_id } => RecordRef::Entity(entity_id),
                TransactionIntent::CreateRelation(_)
                | TransactionIntent::BulkCreateRelations { .. } => {
                    RecordRef::Relation(RelationId::new(PartitionId::main(), u64::MAX, 0))
                }
                TransactionIntent::DeleteRelation { relation_id } => {
                    RecordRef::Relation(relation_id)
                }
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
            let current_state = WorkingState::new(
                self.runtime.partitions.clone(),
                self.runtime.config.adjacency_policy.clone(),
            );
            let plan = self.build_merged_plan_for_state(&current_state)?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub fn commit(mut self) -> Result<CommitOutcome, TransactionCommitError> {
        let mut staged = self.runtime.take_working_state();
        let merged_plan = self
            .build_merged_plan_for_state(&staged)
            .map_err(TransactionCommitError::Conflict)?;
        {
            let mut counters = self.runtime.complexity_counters.borrow_mut();
            counters.partitions_touched_by_commit = touched_partitions_for_plan(&merged_plan);
            let (bulk_entity_slots_reserved, bulk_relation_slots_reserved) =
                bulk_reservations_for_plan(&staged, &merged_plan);
            counters.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
            counters.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
        }
        let commit_boundary_results = self.runtime.run_invariants_for_state(
            &staged,
            self.runtime.current_version_id(),
            InvariantExecutionPoint::CommitBoundary,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&commit_boundary_results) {
            staged.apply_to_runtime(&mut self.runtime.partitions);
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
        let (changed_records, patch_records, diagnostics_entries) = apply_plan_to_staged_state(
            &mut staged,
            &apply_plan,
            self.runtime.config.publication.patch_surface_policy,
            &self.runtime.config.schema_registry,
            self.runtime.config.cascade_delete_policy,
        );
        {
            let mut counters = self.runtime.complexity_counters.borrow_mut();
            counters.entity_slots_touched_by_commit = staged
                .mutation_journal
                .values()
                .map(|journal| journal.entity_slots.len())
                .sum();
            counters.relation_slots_touched_by_commit = staged
                .mutation_journal
                .values()
                .map(|journal| journal.relation_slots.len())
                .sum();
        }

        let structural_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::MutationSensitive,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&structural_results) {
            staged.apply_to_runtime(&mut self.runtime.partitions);
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
        let branch_id = self
            .options
            .target_branch
            .clone()
            .unwrap_or_else(|| self.runtime.config.main_branch.clone());
        let (parents, merge_base_commits) = match self.resolve_parent_commits(&branch_id) {
            Ok(result) => result,
            Err(conflict) => {
                staged.apply_to_runtime(&mut self.runtime.partitions);
                self.runtime.push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::Failure,
                    vec![RelationalDiagnosticsEntry {
                        code: conflict.code,
                        message: conflict.detail.clone(),
                        fields: json!({
                            "branch_id": branch_id.0,
                            "merge_parent_branches": self.options.merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                        }),
                    }],
                );
                return Err(TransactionCommitError::Conflict(conflict));
            }
        };
        let commit_reference = CommitReference {
            commit_id,
            version_id,
            branch_id: branch_id.clone(),
            parents,
        };
        let patch = RelationalPatchRecord {
            ordering: PatchOrdering::CanonicalCommitOrder,
            publication_mode: PatchPublicationMode::CommitNative,
            position: PatchStreamPosition(commit_id.0),
            compatibility: match self.runtime.config.publication.patch_surface_policy {
                crate::config::data::PatchSurfacePolicy::StructuredPatchSurface => {
                    PatchCompatibilityClass::StructuredCompatible
                }
                crate::config::data::PatchSurfacePolicy::DensePatchSurface => {
                    PatchCompatibilityClass::DenseCompatible
                }
            },
            records: patch_records,
        }
        .canonicalized();
        if patch.records.len() > self.runtime.config.publication.max_patch_records_per_commit {
            staged.apply_to_runtime(&mut self.runtime.partitions);
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::PatchPublication,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::DiagnosticsPublicationFailure,
                    message: "patch record budget exceeded".to_string(),
                    fields: json!({
                        "patch_records": patch.records.len(),
                        "max_patch_records_per_commit": self.runtime.config.publication.max_patch_records_per_commit,
                    }),
                }],
            );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::BundleAssembly,
                detail: "patch record budget exceeded".to_string(),
            }));
        }
        let diagnostics_summary = RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            determinism: crate::diagnostics::data::DeterminismExpectation::Required,
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
            staged.apply_to_runtime(&mut self.runtime.partitions);
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
            commit_reference.clone(),
            version_id,
            patch.clone(),
            diagnostics_summary.clone(),
        );
        let lineage_event_ids = self.runtime.ensure_lineage_for_commit(
            &mut staged,
            &commit_reference,
            &merged_plan.merged_intents,
            &changed_records,
        );
        let canonical_commit_envelope = CanonicalCommitEnvelope {
            commit: commit_reference.clone(),
            branch_context: branch_id.clone(),
            merge_parent_branches: self.options.merge_parent_branches.clone(),
            merge_base_commits: merge_base_commits.clone(),
            schema_version: self.runtime.primary_schema_version(),
            schema_registry: self.runtime.config.schema_registry.clone(),
            merged_plan: merged_plan.clone(),
            patch: patch.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            lineage_event_ids,
            index_generation_ids: Vec::new(),
        };

        staged.apply_to_runtime(&mut self.runtime.partitions);
        self.runtime
            .refresh_unique_field_index_for_records(&changed_records, version_id);
        for entity_id in &artifacts.snapshot_state.pinned_entities {
            self.runtime.pin_entity(*entity_id);
        }
        for relation_id in &artifacts.snapshot_state.pinned_relations {
            self.runtime.pin_relation(*relation_id);
        }
        self.runtime
            .snapshots
            .insert(artifacts.snapshot.snapshot_id, artifacts.snapshot_state);
        self.runtime
            .trim_live_history_for_records(&changed_records, version_id);
        self.runtime.next_commit_id += 1;
        self.runtime.next_version_id += 1;
        self.runtime
            .branch_heads
            .insert(branch_id.clone(), Some(commit_reference.clone()));
        self.runtime.commit_graph.insert(
            commit_id,
            crate::history::data::VersionNode {
                commit: commit_reference.clone(),
            },
        );
        self.runtime
            .commit_envelopes
            .insert(commit_id, canonical_commit_envelope.clone());
        self.runtime
            .durable_log
            .push(crate::durability::data::DurableCommitEnvelope {
                envelope: canonical_commit_envelope,
            });
        self.runtime.compact_durable_log_if_needed();
        self.runtime.latest_publication_bundle = Some(artifacts.bundle.clone());
        self.runtime
            .push_diagnostic_artifact(artifacts.diagnostics_summary);
        let _ = self.runtime.run_retention_pass();
        let publication_code = if commit_reference.parents.len() > 1 {
            DiagnosticCode::MergeCommitPublished
        } else {
            DiagnosticCode::CommitPublished
        };
        let mut publication_entries = Vec::new();
        if commit_reference.parents.len() > 1 {
            publication_entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::MergeBaseResolved,
                message: "merge bases resolved deterministically".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
                }),
            });
        }
        publication_entries.push(RelationalDiagnosticsEntry {
            code: publication_code,
            message: if commit_reference.parents.len() > 1 {
                "merge commit published coherently".to_string()
            } else {
                "commit published coherently".to_string()
            },
            fields: json!({
                "commit_id": commit_id.0,
                "snapshot_id": artifacts.snapshot.snapshot_id.0,
                "branch_id": branch_id.0,
                "parent_commit_ids": commit_reference.parents.iter().map(|parent| parent.0).collect::<Vec<_>>(),
                "merge_parent_branches": self.options.merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
            }),
        });
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::MinimalSummary,
            publication_entries,
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            commit: commit_reference,
            version_id,
            snapshot: artifacts.snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
        })
    }

    fn build_merged_plan_for_state(
        &mut self,
        current_state: &impl PartitionAccess,
    ) -> Result<MergedCommitPlan, CommitConflict> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        self.normalize_intents_for_merge(&mut intents);
        for intent in &intents {
            validate_intent(
                current_state,
                &self.runtime.config.schema_registry,
                self.runtime.config.cross_context_policy,
                &self.runtime.complexity_counters,
                intent,
            )?;
        }
        intents.sort_by_key(canonical_intent_key);
        detect_conflicting_updates(&intents)?;
        Ok(MergedCommitPlan {
            transaction_id: self.transaction_id,
            merged_intents: intents,
        })
    }

    fn resolve_parent_commits(
        &self,
        target_branch: &crate::history::data::BranchId,
    ) -> Result<(Vec<CommitId>, Vec<CommitId>), CommitConflict> {
        let mut parents = Vec::new();
        let mut merge_bases = Vec::new();
        let target_head = self
            .runtime
            .branch_head(target_branch)
            .map(|head| head.commit_id);
        if let Some(head) = self.runtime.branch_head(target_branch) {
            parents.push(head.commit_id);
        }
        let mut merge_branches = self.options.merge_parent_branches.clone();
        merge_branches.sort();
        merge_branches.dedup();
        for merge_branch in merge_branches {
            if &merge_branch == target_branch {
                continue;
            }
            let Some(head) = self.runtime.branch_head(&merge_branch) else {
                return Err(CommitConflict {
                    code: DiagnosticCode::InvalidMergeParent,
                    detail: format!("merge parent branch {:?} has no head", merge_branch),
                });
            };
            if !parents.contains(&head.commit_id) {
                if let Some(target_head) = target_head {
                    let inspection = self.runtime.inspect_merge(&merge_branch, target_branch);
                    if !inspection.conflicting_records.is_empty() {
                        return Err(CommitConflict {
                            code: DiagnosticCode::MergeConflictOverlap,
                            detail: format!(
                                "merge between {:?} and {:?} has overlapping authority on {:?}",
                                merge_branch, target_branch, inspection.conflicting_records
                            ),
                        });
                    }
                    let Some(merge_base) = self
                        .runtime
                        .latest_common_ancestor(target_head, head.commit_id)
                    else {
                        return Err(CommitConflict {
                            code: DiagnosticCode::MissingMergeBase,
                            detail: format!(
                                "merge parent branch {:?} has no common ancestor with target branch {:?}",
                                merge_branch, target_branch
                            ),
                        });
                    };
                    merge_bases.push(merge_base);
                }
                parents.push(head.commit_id);
            }
        }
        merge_bases.sort_by_key(|commit_id| commit_id.0);
        merge_bases.dedup();
        Ok((parents, merge_bases))
    }

    fn normalize_intents_for_merge(&mut self, intents: &mut [TransactionIntent]) {
        if self.runtime.config.symbol_policy == SymbolPolicy::Disabled {
            return;
        }

        let mut interner = self.runtime.symbol_interner.borrow_mut();
        let mut raw_values = Vec::new();
        for intent in intents.iter() {
            match intent {
                TransactionIntent::CreateEntity(spec) => {
                    if let InternedString::Raw(raw) = &spec.client_key {
                        raw_values.push(raw.clone());
                    }
                }
                TransactionIntent::BulkCreateEntities { client_keys, .. }
                | TransactionIntent::BulkCreateRelations { client_keys, .. } => {
                    for client_key in client_keys {
                        if let InternedString::Raw(raw) = client_key {
                            raw_values.push(raw.clone());
                        }
                    }
                }
                TransactionIntent::CreateRelation(spec) => {
                    if let InternedString::Raw(raw) = &spec.client_key {
                        raw_values.push(raw.clone());
                    }
                }
                TransactionIntent::UpdateEntity { .. }
                | TransactionIntent::ReplaceEntity { .. }
                | TransactionIntent::DeleteEntity { .. }
                | TransactionIntent::DeleteRelation { .. } => {}
            }
        }
        raw_values.sort();
        raw_values.dedup();
        for raw in &raw_values {
            interner.intern(raw);
        }

        for intent in intents {
            match intent {
                TransactionIntent::CreateEntity(spec) => {
                    spec.client_key = normalize_interned_string(
                        &mut interner,
                        self.runtime.config.symbol_policy,
                        spec.client_key.clone(),
                    );
                }
                TransactionIntent::BulkCreateEntities { client_keys, .. } => {
                    for client_key in client_keys {
                        *client_key = normalize_interned_string(
                            &mut interner,
                            self.runtime.config.symbol_policy,
                            client_key.clone(),
                        );
                    }
                }
                TransactionIntent::CreateRelation(spec) => {
                    spec.client_key = normalize_interned_string(
                        &mut interner,
                        self.runtime.config.symbol_policy,
                        spec.client_key.clone(),
                    );
                }
                TransactionIntent::BulkCreateRelations { client_keys, .. } => {
                    for client_key in client_keys {
                        *client_key = normalize_interned_string(
                            &mut interner,
                            self.runtime.config.symbol_policy,
                            client_key.clone(),
                        );
                    }
                }
                TransactionIntent::UpdateEntity { .. }
                | TransactionIntent::ReplaceEntity { .. }
                | TransactionIntent::DeleteEntity { .. }
                | TransactionIntent::DeleteRelation { .. } => {}
            }
        }
        self.runtime.config.symbol_table = interner.snapshot();
    }
}

fn touched_partitions_for_plan(plan: &MergedCommitPlan) -> usize {
    let mut touched = BTreeSet::new();
    for intent in &plan.merged_intents {
        match intent {
            TransactionIntent::CreateEntity(spec) => {
                touched.insert(spec.partition_id);
            }
            TransactionIntent::BulkCreateEntities { partition_id, .. } => {
                touched.insert(*partition_id);
            }
            TransactionIntent::UpdateEntity { entity_id, .. }
            | TransactionIntent::DeleteEntity { entity_id }
            | TransactionIntent::ReplaceEntity { entity_id, .. } => {
                touched.insert(entity_id.partition_id);
                if let TransactionIntent::ReplaceEntity { replacement, .. } = intent {
                    touched.insert(replacement.partition_id);
                }
            }
            TransactionIntent::CreateRelation(spec) => {
                touched.insert(spec.partition_id);
                touched.insert(spec.source.partition_id);
                touched.insert(spec.target.partition_id);
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => {
                touched.insert(*partition_id);
                for (source, target) in endpoints {
                    touched.insert(source.partition_id);
                    touched.insert(target.partition_id);
                }
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                touched.insert(relation_id.partition_id);
            }
        }
    }
    touched.len()
}

fn bulk_reservations_for_plan(
    state: &impl PartitionAccess,
    plan: &MergedCommitPlan,
) -> (usize, usize) {
    let mut entity_requests = BTreeMap::new();
    let mut relation_requests = BTreeMap::new();
    for intent in &plan.merged_intents {
        match intent {
            TransactionIntent::BulkCreateEntities {
                partition_id,
                payloads,
                ..
            } => {
                *entity_requests.entry(*partition_id).or_insert(0usize) += payloads.len();
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => {
                *relation_requests.entry(*partition_id).or_insert(0usize) += endpoints.len();
            }
            _ => {}
        }
    }

    let entity_reserved = entity_requests
        .into_iter()
        .map(|(partition_id, requested): (crate::identity::data::PartitionId, usize)| {
            let reusable = state
                .get_partition(partition_id)
                .map(|partition| partition.entity_arena.free_list.len())
                .unwrap_or(0);
            requested.saturating_sub(reusable)
        })
        .sum();
    let relation_reserved = relation_requests
        .into_iter()
        .map(|(partition_id, requested): (crate::identity::data::PartitionId, usize)| {
            let reusable = state
                .get_partition(partition_id)
                .map(|partition| partition.relation_arena.free_list.len())
                .unwrap_or(0);
            requested.saturating_sub(reusable)
        })
        .sum();
    (entity_reserved, relation_reserved)
}

fn normalize_interned_string(
    interner: &mut crate::symbols::data::StringInterner,
    policy: SymbolPolicy,
    value: InternedString,
) -> InternedString {
    match policy {
        SymbolPolicy::Disabled => value,
        SymbolPolicy::PreferInterned | SymbolPolicy::RequireInterned => interner.normalize(value),
    }
}
