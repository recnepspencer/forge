use std::collections::BTreeSet;

use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::data::{
    DurabilityError, DurabilityMode, RecoveryCoverage, RecoveryFailureClass, RecoveryPlan,
};
use crate::history::data::VersionNode;
use crate::logic::runtime::{RecoveryOutcome as RuntimeRecoveryOutcome, RelationalRuntime};
use crate::transactions::data::{TransactionOptions, WorkerIntentBatch};

use crate::durability::checkpoints::images::partition_from_image;

impl RelationalRuntime {
    pub fn recover(
        &mut self,
        plan: RecoveryPlan,
    ) -> Result<RuntimeRecoveryOutcome, DurabilityError> {
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
        restored.durability.log = plan.tail_log;
        restored.durability.store = plan.store.clone();
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
            recovered_commits: restored.history.commit_envelopes.len(),
            latest_commit: restored.latest_commit().cloned(),
            restored_branches: restored.history.branch_heads.len(),
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
        restored.durability.store = None;

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
                .contains_key(&restored.config.main_branch)
            {
                restored
                    .history
                    .branch_heads
                    .insert(restored.config.main_branch.clone(), None);
            }
            restored.history.commit_envelopes = checkpoint
                .envelopes
                .iter()
                .cloned()
                .map(|envelope| (envelope.commit.commit_id, envelope))
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
            restored.lineage.correspondence_candidates =
                checkpoint.correspondence_candidates.clone();
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
                .symbols
                .restore_snapshot(checkpoint.symbol_table.clone());
            restored.durability.checkpoints.push(checkpoint.clone());
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
            for counter in &mut partition.entity_arena.snapshot_pins {
                *counter = 0;
            }
            for counter in &mut partition.entity_arena.branch_pins {
                *counter = 0;
            }
            for counter in &mut partition.entity_arena.replay_pins {
                *counter = 0;
            }
            for counter in &mut partition.relation_arena.snapshot_pins {
                *counter = 0;
            }
            for counter in &mut partition.relation_arena.branch_pins {
                *counter = 0;
            }
            for counter in &mut partition.relation_arena.replay_pins {
                *counter = 0;
            }
        }

        let available_commit_ids = restored
            .history
            .commit_envelopes
            .keys()
            .copied()
            .chain(
                plan.tail_log
                    .iter()
                    .map(|entry| entry.envelope.commit.commit_id),
            )
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
                .any(|parent| !restored.history.commit_envelopes.contains_key(parent))
            {
                return Err(DurabilityError {
                    class: RecoveryFailureClass::MissingParentChain,
                    detail: format!(
                        "parent commit not recoverable before child {}",
                        envelope.commit.commit_id.0
                    ),
                });
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
            .chain(
                restored
                    .lineage
                    .correspondence_candidates
                    .iter()
                    .map(|candidate| candidate.candidate_id),
            )
            .max()
            .unwrap_or(0)
            + 1;
        restored.config.durability_mode = original_durability_mode;
        restored.rebuild_unique_field_indexes();
        restored.rebuild_branch_pins_from_heads();
        restored
            .snapshots
            .visibility_states
            .write()
            .expect("visibility state lock poisoned")
            .clear();
        restored
            .snapshots
            .visibility_residency
            .write()
            .expect("visibility residency lock poisoned")
            .clear();
        {
            let mut recent_policy = restored
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned");
            recent_policy.order.clear();
            recent_policy.resident_count = 0;
        }
        restored.rebuild_branch_head_visibility_residency();

        Ok(restored)
    }
}
