use crate::data::diagnostics::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::data::durability::{
    DurabilityError, DurableCheckpoint, DurableCommitEnvelope, RecoveryFailureClass, RecoveryPlan,
};
use crate::data::history::BranchHead;
use crate::data::transaction::{TransactionOptions, WorkerIntentBatch};
use crate::logic::runtime::{RecoveryOutcome, RelationalRuntime};
use serde_json::json;
use std::collections::BTreeSet;

impl RelationalRuntime {
    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = DurableCheckpoint {
            up_to_commit: self.latest_commit().cloned(),
            branches: self.branches(),
            envelopes: self.commit_envelopes.values().cloned().collect(),
            lineage_event_ids: self
                .lineage_events
                .iter()
                .map(|event| event.event_id)
                .collect(),
            index_generation_ids: self
                .index_generations
                .values()
                .flat_map(|generations| {
                    generations
                        .iter()
                        .map(|generation| generation.generation_id.0)
                })
                .collect(),
        };
        self.durable_checkpoints.push(checkpoint.clone());
        self.push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "durable checkpoint created".to_string(),
                fields: json!({
                    "up_to_commit": checkpoint.up_to_commit.as_ref().map(|commit| commit.commit_id.0),
                    "envelope_count": checkpoint.envelopes.len(),
                }),
            }],
        );
        Ok(checkpoint)
    }

    pub fn recover(&mut self, plan: RecoveryPlan) -> Result<RecoveryOutcome, DurabilityError> {
        if plan.config.schema_registry != self.config.schema_registry {
            return Err(DurabilityError {
                class: RecoveryFailureClass::SchemaMismatch,
                detail: "recovery schema registry mismatch".to_string(),
            });
        }
        let mut restored = RelationalRuntime::new(plan.config.clone());
        let checkpoint = plan.checkpoint.clone();
        let mut envelopes = checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.clone())
            .unwrap_or_default();
        envelopes.extend(plan.tail_log.iter().map(|entry| entry.envelope.clone()));
        envelopes.sort_by_key(|envelope| envelope.commit.commit_id);
        let available_commit_ids = envelopes
            .iter()
            .map(|envelope| envelope.commit.commit_id)
            .collect::<BTreeSet<_>>();

        for envelope in &envelopes {
            if envelope.commit.parents.iter().any(|parent| {
                !restored.commit_envelopes.contains_key(parent)
                    && !available_commit_ids.contains(parent)
            }) {
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
                    .and_then(|parent| {
                        envelopes
                            .iter()
                            .find(|candidate| candidate.commit.commit_id == *parent)
                    })
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

        if let Some(checkpoint) = checkpoint {
            restored.durable_checkpoints.push(checkpoint);
        }
        restored.durable_log = plan.tail_log;
        restored.push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "runtime recovered from canonical durable envelopes".to_string(),
                fields: json!({
                    "recovered_commits": restored.commit_envelopes.len(),
                    "restored_branches": restored.branch_heads.len(),
                }),
            }],
        );
        let outcome = RecoveryOutcome {
            recovered_commits: restored.commit_envelopes.len(),
            latest_commit: restored.latest_commit().cloned(),
            restored_branches: restored.branch_heads.len(),
        };
        *self = restored;
        Ok(outcome)
    }

    pub fn recovery_plan(&self) -> RecoveryPlan {
        let checkpoint = self.durable_checkpoints.last().cloned();
        RecoveryPlan {
            config: self.config.clone(),
            checkpoint,
            tail_log: if self.durable_checkpoints.is_empty() {
                self.durable_log.clone()
            } else {
                Vec::new()
            },
        }
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.branches()
    }

    pub fn durable_log(&self) -> &[DurableCommitEnvelope] {
        &self.durable_log
    }
}
