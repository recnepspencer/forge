use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::data::{
    CheckpointCoverage, CompactionOutcome, CompactionPlan, DurabilityError, DurabilityMode,
    DurableBitSet, DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest,
    DurableCommitEnvelope, DurableIntegrityStatus, DurableSegmentId, DurableSegmentManifest,
    DurableStore, DurableStoreLayout, EntityArenaCheckpointImage, PartitionCheckpointImage,
    RecoveryCompatibilityCheck, RecoveryCoverage, RecoveryCursor, RecoveryFailureClass,
    RecoveryIntegrityReport, RecoveryPlan, RelationArenaCheckpointImage, RelationEndpointsImage,
    VersionedPayloadImage,
};
use crate::history::data::BranchHead;
use crate::logic::runtime::{RecoveryOutcome as RuntimeRecoveryOutcome, RelationalRuntime};
use crate::storage::logic::state::{
    AdjacencySet, DenseSlotBitSet, EntityArena, PartitionState, RelationArena, RelationEndpoints,
};
use crate::transactions::data::{TransactionOptions, WorkerIntentBatch};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct DurableStoreManifestFile {
    store: DurableStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct DurableSegmentFile {
    entries: Vec<DurableCommitEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct DurableCheckpointFile {
    checkpoint: DurableCheckpoint,
}

impl RelationalRuntime {
    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = self.build_checkpoint_image();
        if self.config.durability_mode == DurabilityMode::PersistedSegmentedLocalFs {
            let manifest = self.persist_checkpoint_file(&checkpoint)?;
            self.push_bounded_diagnostic(
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
            self.push_bounded_diagnostic(
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
        self.durable_checkpoints.push(checkpoint.clone());
        Ok(checkpoint)
    }

    pub fn recover(&mut self, plan: RecoveryPlan) -> Result<RuntimeRecoveryOutcome, DurabilityError> {
        if plan.config.schema_registry != self.config.schema_registry {
            return Err(DurabilityError {
                class: RecoveryFailureClass::SchemaMismatch,
                detail: "recovery schema registry mismatch".to_string(),
            });
        }
        if plan.config.profile != self.config.profile {
            return Err(DurabilityError {
                class: RecoveryFailureClass::ProfileMismatch,
                detail: "recovery profile mismatch".to_string(),
            });
        }
        if plan.config.runtime_name != self.config.runtime_name {
            return Err(DurabilityError {
                class: RecoveryFailureClass::RuntimeNameMismatch,
                detail: "recovery runtime name mismatch".to_string(),
            });
        }
        if !plan.compatibility.schema_match {
            return Err(DurabilityError {
                class: RecoveryFailureClass::SchemaMismatch,
                detail: "recovery schema registry mismatch".to_string(),
            });
        }
        if !plan.compatibility.profile_match {
            return Err(DurabilityError {
                class: RecoveryFailureClass::ProfileMismatch,
                detail: "recovery profile mismatch".to_string(),
            });
        }
        if !plan.compatibility.runtime_name_match {
            return Err(DurabilityError {
                class: RecoveryFailureClass::RuntimeNameMismatch,
                detail: "recovery runtime name mismatch".to_string(),
            });
        }
        if plan.integrity_report.corrupt_segment_id.is_some() {
            return Err(DurabilityError {
                class: RecoveryFailureClass::CorruptSegment,
                detail: "required durable segment is corrupt".to_string(),
            });
        }

        let tail_commits = plan.tail_log.len();
        let checkpoint_commits = plan
            .checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.len())
            .unwrap_or(0);
        let mut restored = Self::rebuild_runtime_from_plan(plan.clone())?;
        restored.durable_log = plan.tail_log;
        restored.durable_store = plan.store.clone();
        restored.push_bounded_diagnostic(
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
            recovered_commits: restored.commit_envelopes.len(),
            latest_commit: restored.latest_commit().cloned(),
            restored_branches: restored.branch_heads.len(),
            cursor: plan.cursor,
            coverage: RecoveryCoverage {
                checkpoint_commits,
                replayed_tail_commits: tail_commits,
                recovered_through_commit: restored.latest_commit().cloned(),
            },
            integrity_report: plan.integrity_report,
        };
        *self = restored;
        Ok(outcome)
    }

    pub(crate) fn rebuild_runtime_from_plan(
        plan: RecoveryPlan,
    ) -> Result<RelationalRuntime, DurabilityError> {
        let mut restored = RelationalRuntime::new(plan.config.clone());
        let original_durability_mode = restored.config.durability_mode;
        restored.config.durability_mode = DurabilityMode::InMemoryCanonical;
        restored.durable_store = None;

        if let Some(checkpoint) = &plan.checkpoint {
            restored.partitions = checkpoint
                .partition_images
                .iter()
                .cloned()
                .map(|image| (image.partition_id, partition_from_image(image)))
                .collect();
            restored.branch_heads = checkpoint
                .branches
                .iter()
                .cloned()
                .map(|head| (head.branch_id, head.head))
                .collect();
            if !restored.branch_heads.contains_key(&restored.config.main_branch) {
                restored
                    .branch_heads
                    .insert(restored.config.main_branch.clone(), None);
            }
            restored.commit_envelopes = checkpoint
                .envelopes
                .iter()
                .cloned()
                .map(|envelope| (envelope.commit.commit_id, envelope))
                .collect();
            restored.commit_graph = checkpoint
                .envelopes
                .iter()
                .cloned()
                .map(|envelope| {
                    (
                        envelope.commit.commit_id,
                        crate::history::data::VersionNode {
                            commit: envelope.commit,
                        },
                    )
                })
                .collect();
            restored.lineage_nodes = checkpoint
                .lineage_nodes
                .iter()
                .cloned()
                .map(|node| (node.lineage_id, node))
                .collect();
            restored.lineage_events = checkpoint.lineage_events.clone();
            restored.correspondence_candidates = checkpoint.correspondence_candidates.clone();
            restored.index_definitions = checkpoint
                .index_definitions
                .iter()
                .cloned()
                .map(|definition| (definition.index_id, definition))
                .collect();
            for generation in &checkpoint.index_generations {
                restored
                    .index_generations
                    .entry(generation.index_id)
                    .or_default()
                    .push(generation.clone());
            }
            restored.symbol_interner.borrow_mut().restore_snapshot(checkpoint.symbol_table.clone());
            restored.durable_checkpoints.push(checkpoint.clone());
        }

        restored.next_commit_id = restored
            .commit_envelopes
            .keys()
            .map(|id| id.0)
            .max()
            .unwrap_or(0)
            + 1;
        restored.next_version_id = restored
            .commit_envelopes
            .values()
            .map(|envelope| envelope.commit.version_id.0)
            .max()
            .unwrap_or(0)
            + 1;

        let available_commit_ids = restored
            .commit_envelopes
            .keys()
            .copied()
            .chain(plan.tail_log.iter().map(|entry| entry.envelope.commit.commit_id))
            .collect::<BTreeSet<_>>();

        for envelope in plan.tail_log.iter().map(|entry| &entry.envelope) {
            if envelope
                .commit
                .parents
                .iter()
                .any(|parent| !available_commit_ids.contains(parent))
            {
                return Err(DurabilityError {
                    class: RecoveryFailureClass::MissingParentChain,
                    detail: format!(
                        "missing parent chain for commit {}",
                        envelope.commit.commit_id.0
                    ),
                });
            }
            if envelope
                .commit
                .parents
                .iter()
                .any(|parent| !restored.commit_envelopes.contains_key(parent))
            {
                return Err(DurabilityError {
                    class: RecoveryFailureClass::MissingParentChain,
                    detail: format!(
                        "parent commit not recoverable before child {}",
                        envelope.commit.commit_id.0
                    ),
                });
            }
            if !restored.branch_heads.contains_key(&envelope.branch_context) {
                let parent_branch = envelope
                    .commit
                    .parents
                    .first()
                    .and_then(|parent| restored.commit_envelopes.get(parent))
                    .map(|parent| parent.branch_context.clone())
                    .unwrap_or_else(|| restored.config.main_branch.clone());
                let _ = restored.create_branch(envelope.branch_context.clone(), &parent_branch);
            }
            let mut txn = restored.begin_transaction(TransactionOptions {
                target_branch: Some(envelope.branch_context.clone()),
                merge_parent_branches: envelope.merge_parent_branches.clone(),
                ..TransactionOptions::default()
            });
            txn.push_batch(WorkerIntentBatch {
                name: format!("recovery-commit-{}", envelope.commit.commit_id.0),
                partition_key: None,
                worker_local_only: true,
                intents: envelope.merged_plan.merged_intents.clone(),
            });
            txn.commit().map_err(|_| DurabilityError {
                class: RecoveryFailureClass::ReplayFailure,
                detail: format!(
                    "failed to replay durable commit {}",
                    envelope.commit.commit_id.0
                ),
            })?;
        }

        restored.next_index_id = restored
            .index_definitions
            .keys()
            .map(|id| id.0)
            .max()
            .unwrap_or(0)
            + 1;
        restored.next_index_generation_id = restored
            .index_generations
            .values()
            .flat_map(|generations| generations.iter().map(|generation| generation.generation_id.0))
            .max()
            .unwrap_or(0)
            + 1;
        restored.next_lineage_id = restored
            .lineage_nodes
            .keys()
            .map(|id| id.0)
            .max()
            .unwrap_or(0)
            + 1;
        restored.next_lineage_event_id = restored
            .lineage_events
            .iter()
            .map(|event| event.event_id)
            .chain(
                restored
                    .correspondence_candidates
                    .iter()
                    .map(|candidate| candidate.candidate_id),
            )
            .max()
            .unwrap_or(0)
            + 1;
        restored.config.durability_mode = original_durability_mode;
        restored.rebuild_unique_field_indexes();

        Ok(restored)
    }

    pub fn recovery_plan(&self) -> RecoveryPlan {
        match self.config.durability_mode {
            DurabilityMode::InMemoryCanonical => {
                let checkpoint = self.durable_checkpoints.last().cloned();
                let tail_log = match checkpoint.as_ref().and_then(|c| c.coverage.up_to_commit.as_ref())
                {
                    Some(up_to_commit) => self
                        .durable_log
                        .iter()
                        .filter(|entry| entry.envelope.commit.commit_id > up_to_commit.commit_id)
                        .cloned()
                        .collect(),
                    None => self.durable_log.clone(),
                };
                RecoveryPlan {
                    config: self.config.clone(),
                    store: self.durable_store.clone(),
                    checkpoint_manifest: None,
                    checkpoint,
                    cursor: RecoveryCursor {
                        checkpoint_id: None,
                        segment_ids: Vec::new(),
                    },
                    integrity_report: RecoveryIntegrityReport {
                        selected_checkpoint_id: None,
                        skipped_corrupt_checkpoints: Vec::new(),
                        verified_segment_ids: Vec::new(),
                        corrupt_segment_id: None,
                    },
                    compatibility: RecoveryCompatibilityCheck {
                        schema_match: true,
                        profile_match: true,
                        runtime_name_match: true,
                    },
                    tail_log,
                }
            }
            DurabilityMode::PersistedSegmentedLocalFs => self.persisted_recovery_plan(),
        }
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.branches()
    }

    pub fn durable_log(&self) -> &[DurableCommitEnvelope] {
        &self.durable_log
    }

    pub fn compact_store(&mut self) -> Result<CompactionOutcome, DurabilityError> {
        if self.config.durability_mode != DurabilityMode::PersistedSegmentedLocalFs {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: Vec::new(),
            });
        }
        let Some(checkpoint) = self.durable_checkpoints.last() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.durable_store.as_ref()),
            });
        };
        let Some(up_to_commit) = checkpoint.coverage.up_to_commit.as_ref() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.durable_store.as_ref()),
            });
        };
        let mut store = self.ensure_loaded_store()?;
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
        self.persist_store_manifest(&store)?;
        self.durable_store = Some(store);
        self.push_bounded_diagnostic(
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

    pub(crate) fn append_durable_commit(
        &mut self,
        envelope: DurableCommitEnvelope,
    ) -> Result<(), DurabilityError> {
        match self.config.durability_mode {
            DurabilityMode::InMemoryCanonical => {
                self.durable_log.push(envelope);
                Ok(())
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                let mut store = self.ensure_loaded_store()?;
                let segment_capacity = store.layout.segment_commit_capacity.max(1);
                let segment_id = match store.segments.last() {
                    Some(segment) if segment.commit_count < segment_capacity => segment.segment_id,
                    _ => DurableSegmentId(store.segments.last().map(|segment| segment.segment_id.0).unwrap_or(0) + 1),
                };
                let segment_path = segment_file_path(&store.layout, segment_id);
                let mut segment_entries = if segment_path.exists() {
                    read_json::<DurableSegmentFile>(&segment_path)?.entries
                } else {
                    Vec::new()
                };
                segment_entries.push(envelope.clone());
                write_json(&segment_path, &DurableSegmentFile { entries: segment_entries.clone() })?;
                let first_commit_id = segment_entries.first().map(|entry| entry.envelope.commit.commit_id);
                let last_commit_id = segment_entries.last().map(|entry| entry.envelope.commit.commit_id);
                if let Some(existing) = store.segments.iter_mut().find(|segment| segment.segment_id == segment_id) {
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
                        runtime_name: self.config.runtime_name.clone(),
                        profile: self.config.profile,
                        schema_version: self.primary_schema_version(),
                        integrity: DurableIntegrityStatus::Verified,
                    });
                }
                self.persist_store_manifest(&store)?;
                self.durable_store = Some(store);
                self.durable_log.push(envelope);
                self.push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::MinimalSummary,
                    vec![RelationalDiagnosticsEntry {
                        code: DiagnosticCode::DurableAppendSucceeded,
                        message: "durable segment append succeeded".to_string(),
                        fields: json!({
                            "segment_id": segment_id.0,
                            "commit_id": self.durable_log.last().map(|entry| entry.envelope.commit.commit_id.0),
                        }),
                    }],
                );
                Ok(())
            }
        }
    }

    fn build_checkpoint_image(&self) -> DurableCheckpoint {
        DurableCheckpoint {
            coverage: CheckpointCoverage {
                up_to_commit: self.latest_commit().cloned(),
                up_to_version: self.latest_commit().map(|commit| commit.version_id),
            },
            branches: self.branches(),
            envelopes: self.commit_envelopes.values().cloned().collect(),
            partition_images: self
                .partitions
                .values()
                .cloned()
                .map(partition_to_image)
                .collect(),
            lineage_nodes: self.lineage_nodes.values().cloned().collect(),
            lineage_events: self.lineage_events.clone(),
            correspondence_candidates: self.correspondence_candidates.clone(),
            index_definitions: self.index_definitions.values().cloned().collect(),
            index_generations: self
                .index_generations
                .values()
                .flat_map(|generations| generations.iter().cloned())
                .collect(),
            symbol_table: self.symbol_interner.borrow().snapshot(),
            runtime_name: self.config.runtime_name.clone(),
        }
    }

    fn persisted_recovery_plan(&self) -> RecoveryPlan {
        let Ok(store) = self.load_store_from_disk() else {
            return RecoveryPlan {
                config: self.config.clone(),
                store: self.durable_store.clone(),
                checkpoint_manifest: None,
                checkpoint: None,
                tail_log: Vec::new(),
                cursor: RecoveryCursor {
                    checkpoint_id: None,
                    segment_ids: Vec::new(),
                },
                integrity_report: RecoveryIntegrityReport {
                    selected_checkpoint_id: None,
                    skipped_corrupt_checkpoints: Vec::new(),
                    verified_segment_ids: Vec::new(),
                    corrupt_segment_id: None,
                },
                compatibility: RecoveryCompatibilityCheck {
                    schema_match: true,
                    profile_match: true,
                    runtime_name_match: true,
                },
            };
        };
        let mut skipped_corrupt_checkpoints = Vec::new();
        let mut selected_checkpoint = None;
        let mut selected_checkpoint_manifest = None;
        for manifest in store.checkpoints.iter().rev() {
            match read_json::<DurableCheckpointFile>(&manifest.path) {
                Ok(file) => {
                    selected_checkpoint = Some(file.checkpoint);
                    selected_checkpoint_manifest = Some(manifest.clone());
                    break;
                }
                Err(_) => skipped_corrupt_checkpoints.push(manifest.checkpoint_id),
            }
        }
        let checkpoint_commit = selected_checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
            .map(|commit| commit.commit_id);
        let mut tail_log = Vec::new();
        let mut verified_segment_ids = Vec::new();
        let mut corrupt_segment_id = None;
        for manifest in &store.segments {
            if checkpoint_commit.is_some_and(|covered| manifest.last_commit_id.is_some_and(|last| last <= covered)) {
                continue;
            }
            match read_json::<DurableSegmentFile>(&manifest.path) {
                Ok(file) => {
                    verified_segment_ids.push(manifest.segment_id);
                    tail_log.extend(
                        file.entries
                            .into_iter()
                            .filter(|entry| checkpoint_commit.is_none_or(|covered| entry.envelope.commit.commit_id > covered)),
                    );
                }
                Err(_) => {
                    corrupt_segment_id = Some(manifest.segment_id);
                    break;
                }
            }
        }
        RecoveryPlan {
            config: self.config.clone(),
            store: Some(store.clone()),
            checkpoint_manifest: selected_checkpoint_manifest.clone(),
            checkpoint: selected_checkpoint.clone(),
            cursor: RecoveryCursor {
                checkpoint_id: selected_checkpoint_manifest.as_ref().map(|manifest| manifest.checkpoint_id),
                segment_ids: verified_segment_ids.clone(),
            },
            integrity_report: RecoveryIntegrityReport {
                selected_checkpoint_id: selected_checkpoint_manifest.as_ref().map(|manifest| manifest.checkpoint_id),
                skipped_corrupt_checkpoints,
                verified_segment_ids,
                corrupt_segment_id,
            },
            compatibility: RecoveryCompatibilityCheck {
                schema_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.schema_version == self.primary_schema_version())
                    .unwrap_or(true),
                profile_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.profile == self.config.profile)
                    .unwrap_or(true),
                runtime_name_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.runtime_name == self.config.runtime_name)
                    .unwrap_or(true),
            },
            tail_log,
        }
    }

    fn persist_checkpoint_file(
        &mut self,
        checkpoint: &DurableCheckpoint,
    ) -> Result<DurableCheckpointManifest, DurabilityError> {
        let mut store = self.ensure_loaded_store()?;
        let checkpoint_id = DurableCheckpointId(
            store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id.0)
                .unwrap_or(0)
                + 1,
        );
        let path = checkpoint_file_path(&store.layout, checkpoint_id);
        write_json(&path, &DurableCheckpointFile {
            checkpoint: checkpoint.clone(),
        })?;
        let manifest = DurableCheckpointManifest {
            checkpoint_id,
            path,
            coverage: checkpoint.coverage.clone(),
            partition_count: checkpoint.partition_images.len(),
            runtime_name: self.config.runtime_name.clone(),
            profile: self.config.profile,
            schema_version: self.primary_schema_version(),
            integrity: DurableIntegrityStatus::Verified,
        };
        store.checkpoints.push(manifest.clone());
        self.persist_store_manifest(&store)?;
        self.durable_store = Some(store);
        Ok(manifest)
    }

    fn ensure_loaded_store(&self) -> Result<DurableStore, DurabilityError> {
        if let Some(store) = &self.durable_store {
            return load_or_initialize_store(store.layout.clone());
        }
        let Some(layout) = self.config.durable_store_layout.clone() else {
            return Err(DurabilityError {
                class: RecoveryFailureClass::DurableIoFailure,
                detail: "persisted durability mode requires a durable store layout".to_string(),
            });
        };
        load_or_initialize_store(layout)
    }

    fn load_store_from_disk(&self) -> Result<DurableStore, DurabilityError> {
        let Some(layout) = self.config.durable_store_layout.clone() else {
            return Err(DurabilityError {
                class: RecoveryFailureClass::DurableIoFailure,
                detail: "persisted durability mode requires a durable store layout".to_string(),
            });
        };
        load_or_initialize_store(layout)
    }

    fn persist_store_manifest(&self, store: &DurableStore) -> Result<(), DurabilityError> {
        ensure_store_dirs(&store.layout)?;
        write_json(&manifest_path(&store.layout), &DurableStoreManifestFile { store: store.clone() })
    }
}

fn current_segment_ids(store: Option<&DurableStore>) -> Vec<DurableSegmentId> {
    store
        .map(|store| store.segments.iter().map(|segment| segment.segment_id).collect())
        .unwrap_or_default()
}

fn manifest_path(layout: &DurableStoreLayout) -> PathBuf {
    layout.root_path.join("manifest.json")
}

fn segment_file_path(layout: &DurableStoreLayout, segment_id: DurableSegmentId) -> PathBuf {
    layout
        .root_path
        .join("segments")
        .join(format!("segment-{}.json", segment_id.0))
}

fn checkpoint_file_path(
    layout: &DurableStoreLayout,
    checkpoint_id: DurableCheckpointId,
) -> PathBuf {
    layout
        .root_path
        .join("checkpoints")
        .join(format!("checkpoint-{}.json", checkpoint_id.0))
}

fn ensure_store_dirs(layout: &DurableStoreLayout) -> Result<(), DurabilityError> {
    fs::create_dir_all(layout.root_path.join("segments")).map_err(io_error)?;
    fs::create_dir_all(layout.root_path.join("checkpoints")).map_err(io_error)?;
    Ok(())
}

fn load_or_initialize_store(layout: DurableStoreLayout) -> Result<DurableStore, DurabilityError> {
    ensure_store_dirs(&layout)?;
    let manifest = manifest_path(&layout);
    if manifest.exists() {
        return Ok(read_json::<DurableStoreManifestFile>(&manifest)?.store);
    }
    let store = DurableStore {
        layout: layout.clone(),
        segments: Vec::new(),
        checkpoints: Vec::new(),
    };
    write_json(&manifest, &DurableStoreManifestFile { store: store.clone() })?;
    Ok(store)
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, DurabilityError> {
    let bytes = fs::read(path).map_err(io_error)?;
    serde_json::from_slice(&bytes).map_err(|error| DurabilityError {
        class: RecoveryFailureClass::CorruptCheckpoint,
        detail: format!("failed to deserialize {}: {error}", path.display()),
    })
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), DurabilityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let temp_path = path.with_extension("tmp");
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| DurabilityError {
        class: RecoveryFailureClass::DurableIoFailure,
        detail: format!("failed to serialize {}: {error}", path.display()),
    })?;
    fs::write(&temp_path, bytes).map_err(io_error)?;
    fs::rename(&temp_path, path).map_err(io_error)?;
    Ok(())
}

fn io_error(error: std::io::Error) -> DurabilityError {
    DurabilityError {
        class: RecoveryFailureClass::DurableIoFailure,
        detail: error.to_string(),
    }
}

fn partition_to_image(partition: PartitionState) -> PartitionCheckpointImage {
    PartitionCheckpointImage {
        partition_id: partition.partition_id,
        entity_arena: EntityArenaCheckpointImage {
            generations: partition.entity_arena.generations,
            lifecycle: partition.entity_arena.lifecycle,
            kind_ids: partition.entity_arena.kind_ids,
            payloads: partition.entity_arena.payloads,
            payload_history: partition
                .entity_arena
                .payload_history
                .into_iter()
                .map(|entries| {
                    entries
                        .into_iter()
                        .map(|entry| VersionedPayloadImage {
                            effective_at: entry.effective_at,
                            retired_at: entry.retired_at,
                            value: entry.value,
                        })
                        .collect()
                })
                .collect(),
            created_at: partition.entity_arena.created_at,
            retired_at: partition.entity_arena.retired_at,
            aspect_versions: partition.entity_arena.aspect_versions,
            structural_fingerprints: partition.entity_arena.structural_fingerprints,
            lineage_ids: partition.entity_arena.lineage_ids,
            diagnostics_enrichment: partition.entity_arena.diagnostics_enrichment,
            branch_pins: partition.entity_arena.branch_pins,
            replay_pins: partition.entity_arena.replay_pins,
            snapshot_pins: partition.entity_arena.snapshot_pins,
            live_bitset: DurableBitSet {
                words: partition.entity_arena.live_bitset.words().to_vec(),
            },
            reclaimable_bitset: DurableBitSet {
                words: partition.entity_arena.reclaimable_bitset.words().to_vec(),
            },
            free_list: partition.entity_arena.free_list,
        },
        relation_arena: RelationArenaCheckpointImage {
            generations: partition.relation_arena.generations,
            lifecycle: partition.relation_arena.lifecycle,
            kind_ids: partition.relation_arena.kind_ids,
            payloads: partition.relation_arena.payloads,
            payload_history: partition
                .relation_arena
                .payload_history
                .into_iter()
                .map(|(slot, entries)| {
                    (
                        slot,
                        entries
                            .into_iter()
                            .map(|entry| VersionedPayloadImage {
                                effective_at: entry.effective_at,
                                retired_at: entry.retired_at,
                                value: entry.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
            created_at: partition.relation_arena.created_at,
            retired_at: partition.relation_arena.retired_at,
            endpoints: partition
                .relation_arena
                .endpoints
                .into_iter()
                .map(|endpoints| {
                    endpoints.map(|endpoints| RelationEndpointsImage {
                        source: endpoints.source,
                        target: endpoints.target,
                    })
                })
                .collect(),
            diagnostics_enrichment: partition.relation_arena.diagnostics_enrichment,
            snapshot_pins: partition.relation_arena.snapshot_pins,
            live_bitset: DurableBitSet {
                words: partition.relation_arena.live_bitset.words().to_vec(),
            },
            reclaimable_bitset: DurableBitSet {
                words: partition.relation_arena.reclaimable_bitset.words().to_vec(),
            },
            free_list: partition.relation_arena.free_list,
        },
        adjacency: partition
            .adjacency
            .into_iter()
            .map(|adjacency| adjacency.ids())
            .collect(),
        reverse_adjacency: partition
            .reverse_adjacency
            .into_iter()
            .map(|adjacency| adjacency.ids())
            .collect(),
    }
}

fn partition_from_image(image: PartitionCheckpointImage) -> PartitionState {
    PartitionState {
        partition_id: image.partition_id,
        adjacency_policy: crate::config::data::AdjacencyPolicy {
            backend: crate::config::data::AdjacencyBackend::CompressedFanoutAdjacency,
            small_degree_inline_capacity: 4,
        },
        entity_arena: EntityArena {
            partition_ids: vec![image.partition_id; image.entity_arena.generations.len()],
            generations: image.entity_arena.generations,
            lifecycle: image.entity_arena.lifecycle,
            kind_ids: image.entity_arena.kind_ids,
            payloads: image.entity_arena.payloads,
            payload_history: image
                .entity_arena
                .payload_history
                .into_iter()
                .map(|entries| {
                    entries
                        .into_iter()
                        .map(|entry| crate::storage::logic::state::VersionedPayload {
                            effective_at: entry.effective_at,
                            retired_at: entry.retired_at,
                            value: entry.value,
                        })
                        .collect()
                })
                .collect(),
            created_at: image.entity_arena.created_at,
            retired_at: image.entity_arena.retired_at,
            aspect_versions: image.entity_arena.aspect_versions,
            structural_fingerprints: image.entity_arena.structural_fingerprints,
            lineage_ids: image.entity_arena.lineage_ids,
            diagnostics_enrichment: image.entity_arena.diagnostics_enrichment,
            branch_pins: image.entity_arena.branch_pins,
            replay_pins: image.entity_arena.replay_pins,
            snapshot_pins: image.entity_arena.snapshot_pins,
            live_bitset: DenseSlotBitSet::from_words(image.entity_arena.live_bitset.words),
            reclaimable_bitset: DenseSlotBitSet::from_words(
                image.entity_arena.reclaimable_bitset.words,
            ),
            free_list: image.entity_arena.free_list,
        },
        relation_arena: RelationArena {
            partition_ids: vec![image.partition_id; image.relation_arena.generations.len()],
            generations: image.relation_arena.generations,
            lifecycle: image.relation_arena.lifecycle,
            kind_ids: image.relation_arena.kind_ids,
            payloads: image.relation_arena.payloads,
            payload_history: image
                .relation_arena
                .payload_history
                .into_iter()
                .map(|(slot, entries)| {
                    (
                        slot,
                        entries
                            .into_iter()
                            .map(|entry| crate::storage::logic::state::VersionedPayload {
                                effective_at: entry.effective_at,
                                retired_at: entry.retired_at,
                                value: entry.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
            created_at: image.relation_arena.created_at,
            retired_at: image.relation_arena.retired_at,
            endpoints: image
                .relation_arena
                .endpoints
                .into_iter()
                .map(|endpoints| {
                    endpoints.map(|endpoints| RelationEndpoints {
                        source: endpoints.source,
                        target: endpoints.target,
                    })
                })
                .collect(),
            diagnostics_enrichment: image.relation_arena.diagnostics_enrichment,
            snapshot_pins: image.relation_arena.snapshot_pins,
            live_bitset: DenseSlotBitSet::from_words(image.relation_arena.live_bitset.words),
            reclaimable_bitset: DenseSlotBitSet::from_words(
                image.relation_arena.reclaimable_bitset.words,
            ),
            free_list: image.relation_arena.free_list,
        },
        adjacency: image
            .adjacency
            .into_iter()
            .map(|ids| AdjacencySet::Compressed(ids))
            .collect(),
        reverse_adjacency: image
            .reverse_adjacency
            .into_iter()
            .map(|ids| AdjacencySet::Compressed(ids))
            .collect(),
    }
}
