use crate::data::error::SignalError;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::recorder::record_transaction_semantic_event;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase, RollbackDiagnostic};

use super::transaction_types::{SignalTransaction, TransactionOutcome};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn commit(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;

        if self.poisoned {
            self.event_bus.rollback(runtime_ctx);
            self.rollback_graph_state()?;
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("poisoned transaction rollback".to_string()),
            );
            let profile = self.graph.diagnostics_profile();
            self.semantic_delta.rollback = Some(rollback);
            self.semantic_delta.failure_summary = Some(
                ExecutionFailureContext::new(
                    ExecutionFailurePhase::Rollback,
                    None,
                    None,
                    None,
                    None,
                    None,
                    "transaction rolled back because it was poisoned",
                )
                .summarize(self.semantic_delta.rollback.as_ref(), profile),
            );
            self.semantic_delta.replay_events.push((
                ReplayEventKind::TransactionRolledBack,
                "poisoned transaction rollback".to_string(),
                None,
                None,
            ));
            self.semantic_delta.replay_events.push((
                ReplayEventKind::FailureRecorded,
                "transaction rolled back because it was poisoned".to_string(),
                None,
                None,
            ));
            self.telemetry.transaction_poison_count += 1;
            self.finalize_semantic_delta(true);
            return Ok(TransactionOutcome::Poisoned);
        }

        self.staged_patch_count = self.graph_patches.touched_count() as u64;

        if let Err(err) = self
            .event_bus
            .begin(runtime_ctx)
            .map_err(|e| SignalError::invalid_input(format!("event bus begin failed: {e:?}")))
        {
            self.event_bus.rollback(runtime_ctx);
            self.rollback_graph_state()?;
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("event bus begin failed".to_string()),
            );
            let profile = self.graph.diagnostics_profile();
            self.semantic_delta.rollback = Some(rollback);
            self.semantic_delta.failure_summary = Some(
                ExecutionFailureContext::from_error(
                    ExecutionFailurePhase::CommitPromotion,
                    &err,
                    None,
                )
                .summarize(self.semantic_delta.rollback.as_ref(), profile),
            );
            self.semantic_delta.replay_events.push((
                ReplayEventKind::TransactionRolledBack,
                "event bus begin failed".to_string(),
                None,
                None,
            ));
            self.semantic_delta.replay_events.push((
                ReplayEventKind::FailureRecorded,
                err.to_string(),
                None,
                None,
            ));
            self.telemetry.transaction_poison_count += 1;
            self.finalize_semantic_delta(true);
            return Err(err);
        }
        for event in std::mem::take(&mut self.staged_events) {
            self.event_bus.emit(event);
        }
        for barrier in std::mem::take(&mut self.staged_event_flushes) {
            if let Err(err) = self
                .event_bus
                .flush(barrier, runtime_ctx)
                .map_err(|e| SignalError::invalid_input(format!("event bus flush failed: {e:?}")))
            {
                self.event_bus.rollback(runtime_ctx);
                self.rollback_graph_state()?;
                let rollback = RollbackDiagnostic::new(
                    true,
                    self.graph_patches.touched_count() as u64,
                    self.telemetry.max_touched_nodes_in_txn,
                    Some("event bus flush failed".to_string()),
                );
                let profile = self.graph.diagnostics_profile();
                self.semantic_delta.rollback = Some(rollback);
                self.semantic_delta.failure_summary = Some(
                    ExecutionFailureContext::from_error(
                        ExecutionFailurePhase::CommitPromotion,
                        &err,
                        None,
                    )
                    .summarize(self.semantic_delta.rollback.as_ref(), profile),
                );
                self.semantic_delta.replay_events.push((
                    ReplayEventKind::TransactionRolledBack,
                    "event bus flush failed".to_string(),
                    None,
                    None,
                ));
                self.semantic_delta.replay_events.push((
                    ReplayEventKind::FailureRecorded,
                    err.to_string(),
                    None,
                    None,
                ));
                self.telemetry.transaction_poison_count += 1;
                self.finalize_semantic_delta(true);
                return Err(err);
            }
        }

        while let Some(domain) = self.staged_dirty.first_dirty_domain() {
            if let Some(impact) = self.staged_dirty.take_domain_impact(domain) {
                self.checkpoint
                    .dirty_mut()
                    .merge_domain_impact(domain, impact);
            }
        }
        self.checkpoint.telemetry_mut().checkpoint_flushes += self.staged_checkpoint_flushes;
        self.checkpoint.telemetry_mut().checkpoint_flush_nanos +=
            self.staged_checkpoint_flush_nanos;
        let touched_nodes = self.graph_patches.touched_nodes(self.graph);
        for ((family_id, key_id, memo_key_id), result) in
            std::mem::take(&mut self.staged_memo_writes)
        {
            let family = self.config.key_registry.family(family_id).clone();
            let key = self.config.key_registry.keys[key_id.index()].clone();
            let memo_key = self.config.key_registry.memo_key(memo_key_id).clone();
            self.config
                .store_memoized_result(&family, &key, &memo_key, result);
        }
        self.graph_patches.commit_and_clear();
        self.telemetry.transaction_commit_count += 1;
        self.telemetry.staged_node_patch_count += self.staged_patch_count;
        self.telemetry.max_touched_nodes_in_txn = self
            .telemetry
            .max_touched_nodes_in_txn
            .max(self.staged_patch_count);
        let policy = self.graph.runtime_policy();
        if policy.retains_explanation_facts() || policy.retains_provenance_facts() {
            for node in touched_nodes {
                if let Ok(explanation) = self.graph.explain(node) {
                    if policy.retains_explanation_facts() {
                        self.graph.diagnostics_state_mut().record_explanation_fact(
                            ExplanationFact::from_explanation(&explanation),
                        );
                    }
                    if policy.retains_provenance_facts() {
                        self.graph
                            .diagnostics_state_mut()
                            .record_provenance_fact(ProvenanceFact::from_explanation(&explanation));
                    }
                }
            }
        }
        self.semantic_delta.replay_events.push((
            ReplayEventKind::TransactionCommitted,
            "transaction committed".to_string(),
            None,
            None,
        ));
        self.finalize_semantic_delta(false);

        Ok(TransactionOutcome::Committed)
    }

    pub fn rollback(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;
        self.event_bus.rollback(runtime_ctx);
        self.rollback_graph_state()?;
        self.telemetry.transaction_rollback_count += 1;
        let rollback = RollbackDiagnostic::new(
            true,
            self.graph_patches.touched_count() as u64,
            self.telemetry.max_touched_nodes_in_txn,
            Some(if self.poisoned {
                "poisoned transaction rollback".to_string()
            } else {
                "explicit rollback".to_string()
            }),
        );
        self.semantic_delta.rollback = Some(rollback);
        if self.poisoned {
            self.telemetry.transaction_poison_count += 1;
            self.semantic_delta.replay_events.push((
                ReplayEventKind::TransactionRolledBack,
                "poisoned transaction rollback".to_string(),
                None,
                None,
            ));
            self.finalize_semantic_delta(true);
            return Ok(TransactionOutcome::Poisoned);
        }
        self.semantic_delta.replay_events.push((
            ReplayEventKind::TransactionRolledBack,
            "explicit rollback".to_string(),
            None,
            None,
        ));
        self.finalize_semantic_delta(true);
        Ok(TransactionOutcome::RolledBack)
    }

    fn finalize_semantic_delta(&mut self, restore_baseline: bool) {
        if restore_baseline {
            *self.config = self.baseline_config.clone();
            *self.graph.diagnostics_state_mut() = self.baseline_diagnostics_state.clone();
        }
        if let Some(rollback) = self.semantic_delta.rollback.take() {
            self.graph.diagnostics_state_mut().record_rollback(rollback);
        }
        if let Some(failure) = self.semantic_delta.failure_summary.take() {
            self.graph.diagnostics_state_mut().record_failure(failure);
        }
        for (kind, detail, execution_record_id, semantic_segment_id) in
            std::mem::take(&mut self.semantic_delta.replay_events)
        {
            record_transaction_semantic_event(
                self.graph,
                kind,
                detail,
                execution_record_id,
                semantic_segment_id,
            );
        }
    }

    fn rollback_graph_state(&mut self) -> Result<(), SignalError> {
        self.graph_patches.rollback_and_clear(self.graph)?;
        self.graph.rollback_created_nodes(&self.created_nodes);
        self.created_nodes.clear();
        self.graph.rebuild_subscriber_index_from_dependencies()?;
        Ok(())
    }
}
