use std::collections::BTreeSet;
use std::fs;

use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaSource, SchemaVersionSource,
};
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::checkpoints::images::{partition_from_image, partition_to_image};
use crate::durability::data::{
    CheckpointCoverage, CompactionOutcome, CompactionPlan, DurabilityError, DurabilityMode,
    DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest, DurableIntegrityStatus,
    DurableSegmentId, DurableSegmentManifest, RecoveryCompatibilityMismatch, RecoveryCoverage,
    RecoveryFailureClass, RecoveryPlan, RelationIntegrityContractFamily,
};
use crate::durability::log::local_store::{
    checkpoint_file_path, current_segment_ids, ensure_loaded_store, persist_store_manifest,
    read_json, segment_file_path, write_json, DurableCheckpointFile, DurableSegmentFile,
};
use crate::history::data::VersionNode;
use crate::logic::runtime::{RecoveryOutcome as RuntimeRecoveryOutcome, RelationalRuntime};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::logic::{validate_schema_continuity_bundle, SchemaContinuityBundleIssue};
use crate::transactions::data::{TransactionOptions, WorkerIntentBatch};
use serde_json::json;
use std::sync::Arc;

pub struct DurabilityAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> DurabilityAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = self.build_checkpoint_image();
        if self.runtime.runtime_config().durability.policy.mode
            == DurabilityMode::PersistedSegmentedLocalFs
        {
            let manifest = self.persist_checkpoint_file(&checkpoint)?;
            self.runtime.publication_authority().push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::CheckpointCreated,
                    message: "durable checkpoint created".to_string(),
                    fields: json!({
                        "checkpoint_id": manifest.checkpoint_id.0,
                        "up_to_commit": checkpoint.coverage.up_to_commit.as_ref().map(|commit| commit.commit_id.0),
                        "partition_count": manifest.partition_count,
                    }),
                }],
            );
        } else {
            self.runtime.publication_authority().push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::CheckpointCreated,
                    message: "durable checkpoint created".to_string(),
                    fields: json!({
                        "up_to_commit": checkpoint.coverage.up_to_commit.as_ref().map(|commit| commit.commit_id.0),
                        "partition_count": checkpoint.partition_images.len(),
                    }),
                }],
            );
        }
        self.runtime.durability.push_checkpoint(checkpoint.clone());
        Ok(checkpoint)
    }

    pub fn compact_store(&mut self) -> Result<CompactionOutcome, DurabilityError> {
        if self.runtime.runtime_config().durability.policy.mode
            != DurabilityMode::PersistedSegmentedLocalFs
        {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: Vec::new(),
            });
        }
        let Some(checkpoint) = self.runtime.durability.checkpoints.last() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.runtime.durable_store()),
            });
        };
        let Some(up_to_commit) = checkpoint.coverage.up_to_commit.as_ref() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.runtime.durable_store()),
            });
        };
        let mut store = ensure_loaded_store(self.runtime)?;
        let plan = CompactionPlan {
            checkpoint_id: store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id)
                .unwrap_or(DurableCheckpointId(0)),
            removable_segments: store
                .segments
                .iter()
                .filter(|segment| {
                    segment
                        .last_commit_id
                        .map(|commit_id| commit_id <= up_to_commit.commit_id)
                        .unwrap_or(false)
                })
                .map(|segment| segment.segment_id)
                .collect(),
        };
        let mut retained_segments = Vec::new();
        let mut removed_segments = Vec::new();
        store.segments.retain(|segment| {
            if plan.removable_segments.contains(&segment.segment_id) {
                let _ = fs::remove_file(&segment.path);
                removed_segments.push(segment.segment_id);
                false
            } else {
                retained_segments.push(segment.segment_id);
                true
            }
        });
        persist_store_manifest(&store)?;
        self.runtime.durability.store = Some(store);
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::DurableCompactionCompleted,
                message: "durable store compacted".to_string(),
                fields: json!({
                    "checkpoint_id": plan.checkpoint_id.0,
                    "removed_segments": removed_segments.iter().map(|id| id.0).collect::<Vec<_>>(),
                }),
            }],
        );
        Ok(CompactionOutcome {
            removed_segments,
            retained_segments,
        })
    }

    pub fn recover(
        &mut self,
        plan: RecoveryPlan,
    ) -> Result<RuntimeRecoveryOutcome, DurabilityError> {
        let compatibility_entry = recovery_compatibility_diagnostic(&plan);
        let compatibility_artifact_kind = match &plan.compatibility.verification_outcome {
            crate::durability::data::RecoveryVerificationOutcome::VerifiedAtLayer(_) => {
                DiagnosticsArtifactKind::MinimalSummary
            }
            crate::durability::data::RecoveryVerificationOutcome::Rejected { .. } => {
                DiagnosticsArtifactKind::Failure
            }
        };
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            compatibility_artifact_kind,
            vec![compatibility_entry.clone()],
        );
        if matches!(
            plan.compatibility.verification_outcome,
            crate::durability::data::RecoveryVerificationOutcome::Rejected { .. }
        ) {
            record_recovery_verification_counters(self.runtime, &plan.compatibility);
        }
        validate_recovery_compatibility(self.runtime, &plan)?;
        if !plan.compatibility.schema_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery schema registry mismatch",
            )
            .with_compatibility_mismatch(RecoveryCompatibilityMismatch::SchemaRegistryShape {
                expected_primary_schema_version: plan.config.primary_schema_version_id(),
                found_primary_schema_version: self.runtime.primary_schema_version_id(),
                expected_entity_kind_count: plan.config.schema.registry.entity_kinds.len(),
                found_entity_kind_count: self.runtime.schema_registry().entity_kinds.len(),
                expected_relation_kind_count: plan.config.schema.registry.relation_kinds.len(),
                found_relation_kind_count: self.runtime.schema_registry().relation_kinds.len(),
            }));
        }
        if !plan.compatibility.profile_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::ProfileMismatch,
                "recovery profile mismatch",
            )
            .with_compatibility_mismatch(RecoveryCompatibilityMismatch::RuntimeProfile {
                expected: format!("{:?}", plan.config.profile),
                found: format!("{:?}", self.runtime.runtime_profile()),
            }));
        }
        if !plan.compatibility.runtime_name_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::RuntimeNameMismatch,
                "recovery runtime name mismatch",
            )
            .with_compatibility_mismatch(RecoveryCompatibilityMismatch::RuntimeName {
                expected: plan.config.execution.runtime_name.clone(),
                found: self.runtime.runtime_name().to_string(),
            }));
        }
        if plan.integrity_report.corrupt_segment_id.is_some() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::CorruptSegment,
                "required durable segment is corrupt",
            ));
        }

        let tail_commits = plan.tail_log.len();
        let checkpoint_commits = plan
            .checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.len())
            .unwrap_or(0);
        let mut restored = rebuild_runtime_from_plan(plan.clone())?;
        restored.durability.set_log(plan.tail_log);
        restored.durability.store = plan.store.clone();
        record_recovery_verification_counters(&restored, &plan.compatibility);
        restored.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            compatibility_artifact_kind,
            vec![compatibility_entry],
        );
        restored.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![
                RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RecoveryCheckpointSelected,
                    message: "recovery checkpoint selected".to_string(),
                    fields: json!({
                        "checkpoint_id": plan.cursor.checkpoint_id.map(|id| id.0),
                        "skipped_corrupt_checkpoints": plan.integrity_report.skipped_corrupt_checkpoints.iter().map(|id| id.0).collect::<Vec<_>>(),
                    }),
                },
                RelationalDiagnosticsEntry {
                    code: DiagnosticCode::RecoveryRangeReplayed,
                    message: "durable tail replayed".to_string(),
                    fields: json!({
                        "segment_ids": plan.cursor.segment_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                        "tail_commits": tail_commits,
                    }),
                },
            ],
        );
        let outcome = RuntimeRecoveryOutcome {
            recovered_commits: restored.history.commit_envelopes.len(),
            latest_commit: restored.history_access().latest_commit().cloned(),
            restored_branches: restored.history.branch_heads.len(),
            cursor: plan.cursor,
            coverage: RecoveryCoverage {
                checkpoint_commits,
                replayed_tail_commits: tail_commits,
                recovered_through_commit: restored.history_access().latest_commit().cloned(),
            },
            integrity_report: plan.integrity_report,
        };
        *self.runtime = restored;
        Ok(outcome)
    }

    pub(crate) fn compact_log_if_needed(&mut self) {
        use crate::config::data::DurableLogRetentionMode;

        let policy = self.runtime.runtime_config().durability.policy.log.clone();
        if self.runtime.durability.log.len() <= policy.max_in_memory_envelopes {
            return;
        }

        match policy.retention_mode {
            DurableLogRetentionMode::RetainAllInMemory => {}
            DurableLogRetentionMode::CompactAfterCheckpoint => {
                if let Some(checkpoint) = self.runtime.durability.checkpoints.last() {
                    if let Some(commit) = checkpoint.coverage.up_to_commit.as_ref() {
                        self.runtime
                            .durability
                            .log
                            .retain(|entry| entry.commit.commit_id > commit.commit_id);
                        self.runtime.durability.rebuild_log_commit_index();
                    }
                }
                if self.runtime.durability.log.len() > policy.max_in_memory_envelopes {
                    let overflow =
                        self.runtime.durability.log.len() - policy.max_in_memory_envelopes;
                    self.runtime.durability.trim_log_front(overflow);
                }
            }
        }
        if self.runtime.runtime_config().durability.policy.mode
            == DurabilityMode::PersistedSegmentedLocalFs
            && self
                .runtime
                .runtime_config()
                .durability
                .policy
                .checkpoints
                .compact_after_checkpoint
        {
            let _ = self.compact_store();
        }
    }

    pub(crate) fn append_commit(
        &mut self,
        envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), DurabilityError> {
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => {
                self.runtime.durability.push_log_envelope(envelope.clone());
                Ok(())
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                let mut store = ensure_loaded_store(self.runtime)?;
                let segment_capacity = store.layout.segment_commit_capacity.max(1);
                let segment_id = match store.segments.last() {
                    Some(segment) if segment.commit_count < segment_capacity => segment.segment_id,
                    _ => DurableSegmentId(
                        store
                            .segments
                            .last()
                            .map(|segment| segment.segment_id.0)
                            .unwrap_or(0)
                            + 1,
                    ),
                };
                let segment_path = segment_file_path(&store.layout, segment_id);
                let mut segment_entries = if segment_path.exists() {
                    read_json::<DurableSegmentFile>(&segment_path)?.entries
                } else {
                    Vec::new()
                };
                segment_entries.push(envelope.clone());
                write_json(
                    &segment_path,
                    &DurableSegmentFile {
                        entries: segment_entries.clone(),
                    },
                )?;
                let first_commit_id = segment_entries.first().map(|entry| entry.commit.commit_id);
                let last_commit_id = segment_entries.last().map(|entry| entry.commit.commit_id);
                if let Some(existing) = store
                    .segments
                    .iter_mut()
                    .find(|segment| segment.segment_id == segment_id)
                {
                    existing.first_commit_id = first_commit_id;
                    existing.last_commit_id = last_commit_id;
                    existing.commit_count = segment_entries.len();
                    existing.integrity = DurableIntegrityStatus::Verified;
                } else {
                    store.segments.push(DurableSegmentManifest {
                        segment_id,
                        path: segment_path,
                        first_commit_id,
                        last_commit_id,
                        commit_count: segment_entries.len(),
                        runtime_name: self.runtime.runtime_name().to_string(),
                        profile: self.runtime.runtime_profile(),
                        schema_version: self.runtime.primary_schema_version_id(),
                        integrity: DurableIntegrityStatus::Verified,
                    });
                }
                persist_store_manifest(&store)?;
                self.runtime.durability.store = Some(store);
                self.runtime.durability.push_log_envelope(envelope.clone());
                let latest_commit_id = self
                    .runtime
                    .durability
                    .log
                    .last()
                    .map(|entry| entry.commit.commit_id.0)
                    .or(Some(envelope.commit.commit_id.0));
                self.runtime
                    .publication_authority()
                    .push_bounded_diagnostic(
                        DiagnosticsScope::History,
                        DiagnosticsArtifactKind::MinimalSummary,
                        vec![RelationalDiagnosticsEntry {
                            code: DiagnosticCode::DurableAppendSucceeded,
                            message: "durable segment append succeeded".to_string(),
                            fields: json!({
                                "segment_id": segment_id.0,
                                "commit_id": latest_commit_id,
                            }),
                        }],
                    );
                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_durable_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        self.runtime.durability.remove_log_commit(commit_id)
    }

    fn build_checkpoint_image(&self) -> DurableCheckpoint {
        DurableCheckpoint {
            coverage: CheckpointCoverage {
                up_to_commit: self.runtime.history_access().latest_commit().cloned(),
                up_to_version: self
                    .runtime
                    .history_access()
                    .latest_commit()
                    .map(|commit| commit.version_id),
            },
            branches: self.runtime.history_access().branches(),
            envelopes: self.runtime.history_access().commit_envelopes_snapshot(),
            partition_images: self
                .runtime
                .partitions
                .values()
                .cloned()
                .map(partition_to_image)
                .collect(),
            lineage_nodes: self.runtime.lineage_access().nodes_snapshot(),
            lineage_events: self.runtime.lineage_access().events_snapshot(),
            correspondence_candidates: self
                .runtime
                .lineage_access()
                .correspondence_candidates_snapshot(),
            rejected_lineage_decisions: self.runtime.lineage_access().rejected_decisions_snapshot(),
            index_definitions: self.runtime.index_access().definitions_snapshot(),
            index_generations: self.runtime.index_access().generations_snapshot(),
            symbol_table: self.runtime.services.symbols.snapshot(),
            runtime_name: self.runtime.runtime_name().to_string(),
        }
    }

    fn persist_checkpoint_file(
        &mut self,
        checkpoint: &DurableCheckpoint,
    ) -> Result<DurableCheckpointManifest, DurabilityError> {
        let mut store = ensure_loaded_store(self.runtime)?;
        let checkpoint_id = DurableCheckpointId(
            store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id.0)
                .unwrap_or(0)
                + 1,
        );
        let path = checkpoint_file_path(&store.layout, checkpoint_id);
        write_json(
            &path,
            &DurableCheckpointFile {
                checkpoint: checkpoint.clone(),
            },
        )?;
        let manifest = DurableCheckpointManifest {
            checkpoint_id,
            path,
            coverage: checkpoint.coverage.clone(),
            partition_count: checkpoint.partition_images.len(),
            runtime_name: self.runtime.runtime_name().to_string(),
            profile: self.runtime.runtime_profile(),
            schema_version: self.runtime.primary_schema_version_id(),
            integrity: DurableIntegrityStatus::Verified,
        };
        store.checkpoints.push(manifest.clone());
        persist_store_manifest(&store)?;
        self.runtime.durability.store = Some(store);
        Ok(manifest)
    }
}

impl RelationalRuntime {
    pub fn durability_authority(&mut self) -> DurabilityAuthority<'_> {
        DurabilityAuthority::new(self)
    }

    pub(crate) fn rebuild_runtime_from_plan(
        plan: RecoveryPlan,
    ) -> Result<RelationalRuntime, DurabilityError> {
        rebuild_runtime_from_plan(plan)
    }
}

fn rebuild_runtime_from_plan(plan: RecoveryPlan) -> Result<RelationalRuntime, DurabilityError> {
    let mut restored = RelationalRuntime::new(plan.config.clone());
    let original_durability_mode = restored.config.durability.policy.mode;
    restored.config.durability.policy.mode = DurabilityMode::InMemoryCanonical;
    restored.durability.store = None;
    if let Some(first_envelope) = plan.tail_log.first() {
        restored.config.schema.registry = first_envelope.schema_registry.clone();
    }

    if let Some(checkpoint) = &plan.checkpoint {
        restored.partitions = checkpoint
            .partition_images
            .iter()
            .cloned()
            .map(|image| (image.partition_id, partition_from_image(image)))
            .collect();
        restored.history.branch_heads = checkpoint
            .branches
            .iter()
            .cloned()
            .map(|head| (head.branch_id, head.head))
            .collect();
        if !restored
            .history
            .branch_heads
            .contains_key(&restored.config.history.main_branch)
        {
            restored
                .history
                .branch_heads
                .insert(restored.config.history.main_branch.clone(), None);
        }
        restored.history.commit_envelopes = checkpoint
            .envelopes
            .iter()
            .cloned()
            .map(|envelope| (envelope.commit.commit_id, Arc::new(envelope)))
            .collect();
        restored.history.patch_stream_index = checkpoint
            .envelopes
            .iter()
            .map(|envelope| (envelope.patch.position, envelope.commit.commit_id))
            .collect();
        restored.history.commit_graph = checkpoint
            .envelopes
            .iter()
            .cloned()
            .map(|envelope| {
                (
                    envelope.commit.commit_id,
                    VersionNode {
                        commit: envelope.commit,
                    },
                )
            })
            .collect();
        restored.lineage.nodes = checkpoint
            .lineage_nodes
            .iter()
            .cloned()
            .map(|node| (node.lineage_id, node))
            .collect();
        restored.lineage.events = checkpoint.lineage_events.clone();
        restored.lineage.rebuild_branch_event_positions();
        restored.lineage.correspondence_candidates = checkpoint.correspondence_candidates.clone();
        restored.lineage.rejected_decisions = checkpoint.rejected_lineage_decisions.clone();
        restored.indexes.definitions = checkpoint
            .index_definitions
            .iter()
            .cloned()
            .map(|definition| (definition.index_id, definition))
            .collect();
        for generation in &checkpoint.index_generations {
            restored
                .indexes
                .generations
                .entry(generation.index_id)
                .or_default()
                .push(generation.clone());
        }
        restored
            .services
            .symbols
            .restore_snapshot(checkpoint.symbol_table.clone());
        restored.durability.push_checkpoint(checkpoint.clone());
    }

    restored.history.next_commit_id = restored
        .history
        .commit_envelopes
        .keys()
        .map(|id| id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.history.next_version_id = restored
        .history
        .commit_envelopes
        .values()
        .map(|envelope| envelope.commit.version_id.0)
        .max()
        .unwrap_or(0)
        + 1;

    for partition in restored.partitions.values_mut() {
        partition.entity_arena.clear_all_pins();
        partition.relation_arena.clear_all_pins();
    }

    let available_commit_ids = restored
        .history
        .commit_envelopes
        .keys()
        .copied()
        .chain(plan.tail_log.iter().map(|entry| entry.commit.commit_id))
        .collect::<BTreeSet<_>>();

    for envelope in &plan.tail_log {
        restored.config.schema.registry = envelope.schema_registry.clone();
        if envelope
            .commit
            .parents
            .iter()
            .any(|parent| !available_commit_ids.contains(parent))
        {
            return Err(DurabilityError::new(
                RecoveryFailureClass::MissingParentChain,
                format!(
                    "missing parent chain for commit {}",
                    envelope.commit.commit_id.0
                ),
            ));
        }
        if envelope
            .commit
            .parents
            .iter()
            .any(|parent| !restored.history.commit_envelopes.contains_key(parent))
        {
            return Err(DurabilityError::new(
                RecoveryFailureClass::MissingParentChain,
                format!(
                    "parent commit not recoverable before child {}",
                    envelope.commit.commit_id.0
                ),
            ));
        }
        if !restored
            .history
            .branch_heads
            .contains_key(&envelope.branch_context)
        {
            let parent_branch = envelope
                .commit
                .parents
                .first()
                .and_then(|parent| restored.history.commit_envelopes.get(parent))
                .map(|parent| parent.branch_context.clone())
                .unwrap_or_else(|| restored.config.history.main_branch.clone());
            let _ = restored
                .history_authority()
                .create_branch(envelope.branch_context.clone(), &parent_branch);
        }
        if is_metadata_only_lineage_commit(envelope) {
            restored.history_authority().publish_metadata_only_commit(
                envelope.commit.commit_id,
                envelope.commit.clone(),
                envelope.branch_context.clone(),
                envelope.patch.position,
                Arc::new(envelope.clone()),
            );
            apply_authoritative_commit_artifacts(&mut restored, envelope);
        } else {
            let mut txn = restored.begin_transaction(TransactionOptions {
                target_branch: Some(envelope.branch_context.clone()),
                merge_parent_branches: envelope.merge_parent_branches.clone(),
                ..schema_transition_options_for_replay(envelope)
            });
            txn.push_batch(WorkerIntentBatch {
                name: format!("recovery-commit-{}", envelope.commit.commit_id.0),
                partition_key: None,
                worker_local_only: true,
                intents: envelope
                    .merged_plan
                    .merged_intents
                    .clone()
                    .into_iter()
                    .map(crate::transactions::data::MutationIntent::from)
                    .collect(),
            });
            txn.commit().map_err(|_| {
                DurabilityError::new(
                    RecoveryFailureClass::ReplayFailure,
                    format!(
                        "failed to replay durable commit {}",
                        envelope.commit.commit_id.0
                    ),
                )
            })?;
            apply_authoritative_commit_artifacts(&mut restored, envelope);
        }
    }

    restored.indexes.next_index_id = restored
        .indexes
        .definitions
        .keys()
        .map(|id| id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.indexes.next_generation_id = restored
        .indexes
        .generations
        .values()
        .flat_map(|generations| {
            generations
                .iter()
                .map(|generation| generation.generation_id.0)
        })
        .max()
        .unwrap_or(0)
        + 1;
    restored.lineage.next_lineage_id = restored
        .lineage
        .nodes
        .keys()
        .map(|id| id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.lineage.next_event_id = restored
        .lineage
        .events
        .iter()
        .map(|event| event.event_id)
        .max()
        .unwrap_or(0)
        + 1;
    restored.lineage.next_candidate_id = restored
        .lineage
        .correspondence_candidates
        .iter()
        .map(|candidate| candidate.candidate_id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.config.durability.policy.mode = original_durability_mode;
    restored.index_authority().rebuild_unique_field_indexes();
    restored.visibility_pins().rebuild_branch_pins_from_heads();
    restored.visibility.cache.clear();
    restored
        .visibility_pins()
        .rebuild_branch_head_visibility_residency();

    Ok(restored)
}

fn validate_recovery_compatibility(
    runtime: &(
        impl SchemaSource + RuntimeIdentitySource + SchemaVersionSource + RuntimeConfigSource
    ),
    plan: &RecoveryPlan,
) -> Result<(), DurabilityError> {
    if plan.config.schema.registry != *runtime.schema_registry() {
        return Err(DurabilityError::new(
            RecoveryFailureClass::SchemaMismatch,
            "recovery schema registry mismatch",
        )
        .with_compatibility_mismatch(schema_registry_mismatch(
            &plan.config.schema.registry,
            runtime.schema_registry(),
            plan.config.primary_schema_version_id(),
            runtime.primary_schema_version_id(),
        )));
    }
    if plan.config.profile != runtime.runtime_profile() {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ProfileMismatch,
            "recovery profile mismatch",
        )
        .with_compatibility_mismatch(RecoveryCompatibilityMismatch::RuntimeProfile {
            expected: format!("{:?}", plan.config.profile),
            found: format!("{:?}", runtime.runtime_profile()),
        }));
    }
    if plan.config.execution.runtime_name != runtime.runtime_name() {
        return Err(DurabilityError::new(
            RecoveryFailureClass::RuntimeNameMismatch,
            "recovery runtime name mismatch",
        )
        .with_compatibility_mismatch(RecoveryCompatibilityMismatch::RuntimeName {
            expected: plan.config.execution.runtime_name.clone(),
            found: runtime.runtime_name().to_string(),
        }));
    }
    validate_schema_continuity_compatibility(runtime, plan)?;
    Ok(())
}

fn recovery_compatibility_diagnostic(
    plan: &RecoveryPlan,
) -> RelationalDiagnosticsEntry {
    let (verification_layer, verification_detail, rejected) = match &plan.compatibility.verification_outcome {
        crate::durability::data::RecoveryVerificationOutcome::VerifiedAtLayer(layer) => (
            format!("{layer:?}"),
            None,
            false,
        ),
        crate::durability::data::RecoveryVerificationOutcome::Rejected { layer, detail } => (
            format!("{layer:?}"),
            Some(detail.clone()),
            true,
        ),
    };
    RelationalDiagnosticsEntry {
        code: DiagnosticCode::DurableRecoveryCompatibilityEvaluated,
        message: "durable recovery compatibility evaluated before recovery execution".to_string(),
        fields: json!({
            "verification_mode": format!("{:?}", plan.verification_mode()),
            "verification_layer": verification_layer,
            "verification_rejected": rejected,
            "verification_detail": verification_detail,
            "descriptor_semantics_version": plan.descriptor_semantics_version.0,
            "first_mismatch": plan.compatibility.first_mismatch.as_ref().map(|mismatch| format!("{:?}", mismatch)),
            "schema_parity": format!("{:?}", plan.compatibility.schema_parity),
            "descriptor_version_parity": format!("{:?}", plan.compatibility.descriptor_version_parity),
            "schema_transition_parity": format!("{:?}", plan.compatibility.schema_transition_parity),
            "continuation_descriptor_parity": format!("{:?}", plan.compatibility.continuation_descriptor_parity),
            "reconciliation_descriptor_parity": format!("{:?}", plan.compatibility.reconciliation_descriptor_parity),
            "schema_lineage_parity": format!("{:?}", plan.compatibility.schema_lineage_parity),
        }),
    }
}

fn record_recovery_verification_counters(
    runtime: &RelationalRuntime,
    compatibility: &crate::durability::data::RecoveryCompatibilityCheck,
) {
    let layer = match compatibility.verification_outcome {
        crate::durability::data::RecoveryVerificationOutcome::VerifiedAtLayer(layer) => layer,
        crate::durability::data::RecoveryVerificationOutcome::Rejected { layer, .. } => layer,
    };
    runtime
        .performance_access()
        .count_replay_verification_layer(layer);
    if matches!(
        compatibility.first_mismatch,
        Some(RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { .. })
            | Some(RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion { .. })
    ) {
        runtime.performance_access().count_descriptor_version_mismatch();
    }
}

fn apply_authoritative_commit_artifacts(
    runtime: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) {
    if !envelope.lineage_events().is_empty() {
        for event in envelope.lineage_events() {
            if let Some(existing) = runtime
                .lineage
                .events
                .iter_mut()
                .find(|candidate| candidate.event_id == event.event_id)
            {
                *existing = event.clone();
            } else {
                runtime.lineage.events.push(event.clone());
            }
        }
        runtime
            .lineage
            .events
            .sort_by_key(|event| event.event_id);
    }

    if !envelope.index_generations.is_empty() {
        for generation in &envelope.index_generations {
            let generations = runtime
                .indexes
                .generations
                .entry(generation.index_id)
                .or_default();
            if let Some(existing) = generations
                .iter_mut()
                .find(|candidate| candidate.generation_id == generation.generation_id)
            {
                *existing = generation.clone();
            } else {
                generations.push(generation.clone());
                generations.sort_by_key(|candidate| candidate.generation_id);
            }
        }
        if let Some(commit_envelope) = runtime
            .history
            .commit_envelopes
            .get_mut(&envelope.commit.commit_id)
        {
            let commit_envelope = Arc::make_mut(commit_envelope);
            commit_envelope.append_index_generations_canonical(&envelope.index_generations);
        }
    }
}

fn is_metadata_only_lineage_commit(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind()
        == crate::replay::data::CanonicalCommitAuthorityKind::MetadataOnlyLineage
}

fn schema_transition_options_for_replay(
    envelope: &CanonicalCommitEnvelope,
) -> TransactionOptions {
    let options = TransactionOptions::default();
    let Some(transition) = envelope.schema_transition.as_ref() else {
        return options;
    };
    options.with_schema_transition(
        crate::schema::data::ProposedSchemaTransition {
            source_schema_id: transition.source_schema_id.clone(),
            source_schema_version_id: transition.source_schema_version_id,
            target_schema_id: transition.target_schema_id.clone(),
            target_schema_version_id: transition.target_schema_version_id,
            diff_atoms: transition.diff_atoms.clone(),
        },
        Some(transition.reconciliation_descriptor.policy),
    )
}

fn validate_schema_continuity_compatibility(
    runtime: &(
        impl SchemaSource + RuntimeIdentitySource + SchemaVersionSource + RuntimeConfigSource
    ),
    plan: &RecoveryPlan,
) -> Result<(), DurabilityError> {
    let descriptor_policy = runtime
        .runtime_config()
        .schema
        .descriptor_semantics_policy
        .clone();
    let canonicalization_policy = runtime
        .runtime_config()
        .schema
        .descriptor_canonicalization_policy
        .clone();
    let runtime_descriptor_version = descriptor_policy.current_write_version();
    let runtime_canonicalization_version = canonicalization_policy.current_write_version();
    if !descriptor_policy.supports(plan.descriptor_semantics_version) {
        return Err(DurabilityError::new(
            RecoveryFailureClass::SchemaMismatch,
            "recovery descriptor semantics version mismatch",
        )
        .with_compatibility_mismatch(
            RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
                expected: plan.descriptor_semantics_version,
                found: runtime_descriptor_version,
            },
        ));
    }

    let checkpoint_envelopes = plan
        .checkpoint
        .as_ref()
        .map(|checkpoint| checkpoint.envelopes.as_slice())
        .unwrap_or(&[]);
    for envelope in checkpoint_envelopes.iter().chain(plan.tail_log.iter()) {
        if envelope.descriptor_semantics_version != plan.descriptor_semantics_version {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery envelope descriptor semantics version mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
                    expected: plan.descriptor_semantics_version,
                    found: envelope.descriptor_semantics_version,
                },
            ));
        }

        if let Some(found) = envelope
            .schema_continuation_descriptor
            .as_ref()
            .map(|descriptor| descriptor.bridge.canonicalization_version)
            .into_iter()
            .chain(
                envelope
                    .schema_reconciliation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.canonicalization_version),
            )
            .find(|version| !canonicalization_policy.supports(*version))
        {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery envelope descriptor canonicalization version mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
                    expected: runtime_canonicalization_version,
                    found,
                },
            ));
        }

        let validated_bundle = validate_schema_continuity_bundle(envelope)
            .map_err(|issue| schema_continuity_recovery_error(envelope, issue))?;
        let _ = (
            validated_bundle.envelope(),
            validated_bundle.transition(),
            validated_bundle.continuation(),
            validated_bundle.reconciliation(),
        );
    }

    let _ = runtime;
    Ok(())
}

fn schema_continuity_recovery_error(
    envelope: &CanonicalCommitEnvelope,
    issue: SchemaContinuityBundleIssue,
) -> DurabilityError {
    let detail = issue.detail();
    let mismatch = match issue {
        SchemaContinuityBundleIssue::IncompleteBundle => {
            RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::ContinuationDescriptorDrift {
            boundary_fingerprint,
        } => RecoveryCompatibilityMismatch::ContinuationDescriptor {
            commit_id: envelope.commit.commit_id.0,
            boundary_fingerprint,
            detail,
        },
        SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => {
            RecoveryCompatibilityMismatch::ReconciliationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch {
            boundary_fingerprint,
        } => RecoveryCompatibilityMismatch::ContinuationDescriptor {
            commit_id: envelope.commit.commit_id.0,
            boundary_fingerprint: Some(boundary_fingerprint),
            detail,
        },
        SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { expected, found } => {
            RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { expected, found }
        }
        SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch { expected, found } => {
            RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
                expected,
                found,
            }
        }
        SchemaContinuityBundleIssue::VisibleBridgeProofMismatch => {
            RecoveryCompatibilityMismatch::ContinuationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                boundary_fingerprint: envelope
                    .schema_continuation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.boundary_fingerprint),
                detail,
            }
        }
        SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => {
            RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => {
            RecoveryCompatibilityMismatch::SchemaLineage {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => {
            RecoveryCompatibilityMismatch::ContinuationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                boundary_fingerprint: envelope
                    .schema_continuation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.boundary_fingerprint),
                detail,
            }
        }
    };
    DurabilityError::new(
        RecoveryFailureClass::SchemaMismatch,
        "recovery schema continuity compatibility failure",
    )
    .with_compatibility_mismatch(mismatch)
}

fn schema_registry_mismatch(
    expected: &crate::schema::data::RelationalSchemaRegistry,
    found: &crate::schema::data::RelationalSchemaRegistry,
    expected_primary_schema_version: crate::schema::data::SchemaVersionId,
    found_primary_schema_version: crate::schema::data::SchemaVersionId,
) -> RecoveryCompatibilityMismatch {
    for (kind_id, expected_registration) in &expected.entity_kinds {
        let Some(found_registration) = found.entity_kinds.get(kind_id) else {
            break;
        };
        if expected_registration.kind_name == found_registration.kind_name
            && expected_registration.schema_id == found_registration.schema_id
            && expected_registration.schema_version_id == found_registration.schema_version_id
            && expected_registration.aspect_declarations.plan_revision
                != found_registration.aspect_declarations.plan_revision
        {
            return RecoveryCompatibilityMismatch::EntityAspectPlanRevision {
                kind_id: *kind_id,
                kind_name: expected_registration.kind_name.clone(),
                expected_revision: expected_registration.aspect_declarations.plan_revision.0,
                found_revision: found_registration.aspect_declarations.plan_revision.0,
            };
        }
    }
    for (kind_id, expected_registration) in &expected.relation_kinds {
        let Some(found_registration) = found.relation_kinds.get(kind_id) else {
            break;
        };
        if expected_registration.kind_name == found_registration.kind_name
            && expected_registration.schema_id == found_registration.schema_id
            && expected_registration.schema_version_id == found_registration.schema_version_id
            && expected_registration.aspect_declarations.plan_revision
                != found_registration.aspect_declarations.plan_revision
        {
            return RecoveryCompatibilityMismatch::RelationAspectPlanRevision {
                kind_id: *kind_id,
                kind_name: expected_registration.kind_name.clone(),
                expected_revision: expected_registration.aspect_declarations.plan_revision.0,
                found_revision: found_registration.aspect_declarations.plan_revision.0,
            };
        }
        if expected_registration.kind_name == found_registration.kind_name
            && expected_registration.schema_id == found_registration.schema_id
            && expected_registration.schema_version_id == found_registration.schema_version_id
            && expected_registration.relation_integrity.plan_revision
                != found_registration.relation_integrity.plan_revision
        {
            let (contract_family, expected_contract_ids, found_contract_ids) =
                relation_integrity_contract_mismatch(
                    &expected_registration.relation_integrity,
                    &found_registration.relation_integrity,
                );
            return RecoveryCompatibilityMismatch::RelationIntegrityPlanRevision {
                kind_id: *kind_id,
                kind_name: expected_registration.kind_name.clone(),
                contract_family,
                expected_revision: expected_registration.relation_integrity.plan_revision.0,
                found_revision: found_registration.relation_integrity.plan_revision.0,
                expected_contract_ids,
                found_contract_ids,
            };
        }
    }
    RecoveryCompatibilityMismatch::SchemaRegistryShape {
        expected_primary_schema_version,
        found_primary_schema_version,
        expected_entity_kind_count: expected.entity_kinds.len(),
        found_entity_kind_count: found.entity_kinds.len(),
        expected_relation_kind_count: expected.relation_kinds.len(),
        found_relation_kind_count: found.relation_kinds.len(),
    }
}

fn relation_integrity_contract_mismatch(
    expected: &crate::schema::data::RelationIntegrityDeclarations,
    found: &crate::schema::data::RelationIntegrityDeclarations,
) -> (RelationIntegrityContractFamily, Vec<String>, Vec<String>) {
    if expected.endpoint_kind_contracts != found.endpoint_kind_contracts {
        return (
            RelationIntegrityContractFamily::EndpointKind,
            expected
                .endpoint_kind_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
            found
                .endpoint_kind_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
        );
    }
    if expected.cardinality_contracts != found.cardinality_contracts {
        return (
            RelationIntegrityContractFamily::Cardinality,
            expected
                .cardinality_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
            found
                .cardinality_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
        );
    }
    if expected.uniqueness_contracts != found.uniqueness_contracts {
        return (
            RelationIntegrityContractFamily::Uniqueness,
            expected
                .uniqueness_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
            found
                .uniqueness_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
        );
    }
    if expected.symmetry_contracts != found.symmetry_contracts {
        return (
            RelationIntegrityContractFamily::Symmetry,
            expected
                .symmetry_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
            found
                .symmetry_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
        );
    }
    if expected.endpoint_deletion_integrity_contracts != found.endpoint_deletion_integrity_contracts
    {
        return (
            RelationIntegrityContractFamily::EndpointDeletionIntegrity,
            expected
                .endpoint_deletion_integrity_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
            found
                .endpoint_deletion_integrity_contracts
                .iter()
                .map(|contract| contract.contract_id.clone())
                .collect(),
        );
    }
    (
        RelationIntegrityContractFamily::Aggregate,
        Vec::new(),
        Vec::new(),
    )
}
