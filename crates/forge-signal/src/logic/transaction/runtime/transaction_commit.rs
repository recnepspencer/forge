use crate::data::error::SignalError;
use crate::diagnostics::recorder::DiagnosticsRecorder;
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
            self.graph_patches.rollback_and_clear(self.graph)?;
            DiagnosticsRecorder::new(self.graph)
                .restore_snapshot(self.diagnostics_snapshot.clone());
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("poisoned transaction rollback".to_string()),
            );
            DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
            let profile = self.graph.diagnostics_profile();
            DiagnosticsRecorder::new(self.graph).record_failure_summary(
                ExecutionFailureContext::new(
                    ExecutionFailurePhase::Rollback,
                    None,
                    None,
                    None,
                    None,
                    None,
                    "transaction rolled back because it was poisoned",
                )
                .summarize(Some(&rollback), profile),
            );
            self.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }

        self.staged_patch_count = self.graph_patches.touched_count() as u64;

        if let Err(err) = self
            .event_bus
            .begin(runtime_ctx)
            .map_err(|e| SignalError::invalid_input(format!("event bus begin failed: {e:?}")))
        {
            self.event_bus.rollback(runtime_ctx);
            self.graph_patches.rollback_and_clear(self.graph)?;
            DiagnosticsRecorder::new(self.graph)
                .restore_snapshot(self.diagnostics_snapshot.clone());
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("event bus begin failed".to_string()),
            );
            DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
            let profile = self.graph.diagnostics_profile();
            DiagnosticsRecorder::new(self.graph).record_failure_summary(
                ExecutionFailureContext::from_error(
                    ExecutionFailurePhase::CommitPromotion,
                    &err,
                    None,
                )
                .summarize(Some(&rollback), profile),
            );
            self.telemetry.transaction_poison_count += 1;
            return Err(err);
        }
        for event in self.staged_events {
            self.event_bus.emit(event);
        }
        for barrier in self.staged_event_flushes {
            if let Err(err) = self
                .event_bus
                .flush(barrier, runtime_ctx)
                .map_err(|e| SignalError::invalid_input(format!("event bus flush failed: {e:?}")))
            {
                self.event_bus.rollback(runtime_ctx);
                self.graph_patches.rollback_and_clear(self.graph)?;
                DiagnosticsRecorder::new(self.graph)
                    .restore_snapshot(self.diagnostics_snapshot.clone());
                let rollback = RollbackDiagnostic::new(
                    true,
                    self.graph_patches.touched_count() as u64,
                    self.telemetry.max_touched_nodes_in_txn,
                    Some("event bus flush failed".to_string()),
                );
                DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
                let profile = self.graph.diagnostics_profile();
                DiagnosticsRecorder::new(self.graph).record_failure_summary(
                    ExecutionFailureContext::from_error(
                        ExecutionFailurePhase::CommitPromotion,
                        &err,
                        None,
                    )
                    .summarize(Some(&rollback), profile),
                );
                self.telemetry.transaction_poison_count += 1;
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
        self.checkpoint.telemetry_mut().checkpoint_flush_nanos += self.staged_checkpoint_flush_nanos;
        for ((family_id, key_id, memo_key_id), result) in self.staged_memo_writes {
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

        Ok(TransactionOutcome::Committed)
    }

    pub fn rollback(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;
        self.event_bus.rollback(runtime_ctx);
        self.graph_patches.rollback_and_clear(self.graph)?;
        DiagnosticsRecorder::new(self.graph).restore_snapshot(self.diagnostics_snapshot);
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
        DiagnosticsRecorder::new(self.graph).record_rollback(rollback);
        if let Some(failure) = self.pending_failure_summary {
            DiagnosticsRecorder::new(self.graph).record_failure_summary(failure);
        }
        if self.poisoned {
            self.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }
        Ok(TransactionOutcome::RolledBack)
    }
}
