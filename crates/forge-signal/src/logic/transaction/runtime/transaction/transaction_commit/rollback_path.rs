use crate::clock::RuntimeInstant;
use crate::data::error::SignalError;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase, RollbackDiagnostic};

use super::super::transaction_types::{
    CreatedNodeRollbackDelta, GraphPatchRollbackDelta, SignalTransaction,
    SubscriberRepairRollbackDelta, TransactionOutcome, TransactionReplayEntry, TransactionResult,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn rollback(mut self) -> Result<TransactionResult, SignalError> {
        let commit_start = RuntimeInstant::now();
        if self.finished {
            return Err(SignalError::transaction_finished());
        }
        self.finished = true;
        let rollback_patch_count = self.rollback_patch_count();
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph).len() as u32;
        self.event_bus.rollback(self.runtime_ctx);
        self.stage_graph_rollback_packets()?;
        if !self.poisoned {
            self.telemetry.transaction.transaction_rollback_count += 1;
        }
        let rollback = RollbackDiagnostic::new(
            true,
            rollback_patch_count,
            self.telemetry.transaction.max_touched_nodes_in_txn,
            Some(if self.poisoned {
                "poisoned transaction rollback".to_string()
            } else {
                "explicit rollback".to_string()
            }),
            self.scratch.semantic_delta.event_epochs.clone(),
        );
        self.scratch.semantic_delta.rollback = Some(rollback);
        if self.poisoned {
            self.telemetry.transaction.transaction_poison_count += 1;
            self.scratch
                .semantic_delta
                .replay_events
                .push(TransactionReplayEntry {
                    kind: ReplayEventKind::TransactionRolledBack,
                    detail: "poisoned transaction rollback".to_string(),
                    execution_record_id: None,
                    semantic_segment_id: None,
                });
            return Ok(self.finalize_semantic_delta(
                true,
                TransactionOutcome::Poisoned,
                touched_nodes,
                commit_start.elapsed().as_nanos(),
            ));
        }
        self.scratch
            .semantic_delta
            .replay_events
            .push(TransactionReplayEntry {
                kind: ReplayEventKind::TransactionRolledBack,
                detail: "explicit rollback".to_string(),
                execution_record_id: None,
                semantic_segment_id: None,
            });
        Ok(self.finalize_semantic_delta(
            true,
            TransactionOutcome::RolledBack,
            touched_nodes,
            commit_start.elapsed().as_nanos(),
        ))
    }

    pub(super) fn stage_graph_rollback_packets(&mut self) -> Result<(), SignalError> {
        let graph_patches = std::mem::take(&mut self.scratch.graph_patches);
        let created_nodes = std::mem::take(&mut self.scratch.created_nodes);
        let mut rewired_sources =
            graph_patches.collect_dependency_sources_for_rollback(self.graph)?;
        for &node in &created_nodes {
            if self.graph.is_alive(node) {
                rewired_sources.extend(self.graph.dependency_sources_of(node)?);
            }
        }
        if !graph_patches.is_empty() {
            self.rollback_packets
                .stage_graph_patches(GraphPatchRollbackDelta {
                    patches: graph_patches,
                })?;
        }
        if !created_nodes.is_empty() {
            self.rollback_packets
                .stage_created_nodes(CreatedNodeRollbackDelta { created_nodes })?;
        }
        if !rewired_sources.is_empty() {
            self.rollback_packets
                .stage_subscriber_repair(SubscriberRepairRollbackDelta {
                    sources: rewired_sources,
                })?;
        }
        self.scratch.dirty_targets.clear_all();
        Ok(())
    }

    pub(super) fn rollback_patch_count(&self) -> u64 {
        self.scratch.graph_patches.touched_count() as u64 + self.scratch.created_nodes.len() as u64
    }

    pub(super) fn fail_and_rollback(
        &mut self,
        rollback_reason: &str,
        error: Option<SignalError>,
        fallback_failure_message: Option<String>,
        failure_phase: ExecutionFailurePhase,
        increment_poison_count: bool,
        outcome: Result<TransactionOutcome, SignalError>,
        commit_start: RuntimeInstant,
    ) -> Result<TransactionResult, SignalError> {
        let rollback_patch_count = self.rollback_patch_count();
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph).len() as u32;
        self.event_bus.rollback(self.runtime_ctx);
        self.stage_graph_rollback_packets()?;
        let rollback = RollbackDiagnostic::new(
            true,
            rollback_patch_count,
            self.telemetry.transaction.max_touched_nodes_in_txn,
            Some(rollback_reason.to_string()),
            self.scratch.semantic_delta.event_epochs.clone(),
        );
        let profile = self.graph.diagnostics_profile();
        self.scratch.semantic_delta.rollback = Some(rollback);
        self.scratch.semantic_delta.failure_summary = Some(match &error {
            Some(err) => ExecutionFailureContext::from_error(failure_phase, err, None)
                .summarize(self.scratch.semantic_delta.rollback.as_ref(), profile),
            None => ExecutionFailureContext::new(
                failure_phase,
                None,
                None,
                None,
                None,
                None,
                fallback_failure_message
                    .as_deref()
                    .unwrap_or(rollback_reason),
            )
            .summarize(self.scratch.semantic_delta.rollback.as_ref(), profile),
        });
        self.scratch
            .semantic_delta
            .replay_events
            .push(TransactionReplayEntry {
                kind: ReplayEventKind::TransactionRolledBack,
                detail: rollback_reason.to_string(),
                execution_record_id: None,
                semantic_segment_id: None,
            });
        self.scratch
            .semantic_delta
            .replay_events
            .push(TransactionReplayEntry {
                kind: ReplayEventKind::FailureRecorded,
                detail: error
                    .as_ref()
                    .map(|err| err.to_string())
                    .or(fallback_failure_message)
                    .unwrap_or_else(|| rollback_reason.to_string()),
                execution_record_id: None,
                semantic_segment_id: None,
            });
        if increment_poison_count {
            self.telemetry.transaction.transaction_poison_count += 1;
        }
        match outcome {
            Ok(outcome) => Ok(self.finalize_semantic_delta(
                true,
                outcome,
                touched_nodes,
                commit_start.elapsed().as_nanos(),
            )),
            Err(err) => {
                let _ = self.finalize_semantic_delta(
                    true,
                    TransactionOutcome::RolledBack,
                    touched_nodes,
                    commit_start.elapsed().as_nanos(),
                );
                Err(err)
            }
        }
    }
}
