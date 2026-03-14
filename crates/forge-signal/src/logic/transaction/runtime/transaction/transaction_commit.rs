use crate::data::error::SignalError;
use crate::diagnostics::epochs::{
    EventEpochOutcome, EventEpochSummary, EventSubscriberOutcome, EventSubscriberOutcomeKind,
};
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::recorder::record_transaction_semantic_event;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase, RollbackDiagnostic};
use crate::logic::events::EventFlushError;
use std::time::Instant;

use super::transaction_types::{
    SignalTransaction, StagedEventOperation, TransactionOutcome, TransactionReplayEntry,
    TransactionResult, TransactionTiming,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn commit(mut self) -> Result<TransactionResult, SignalError> {
        let commit_start = Instant::now();
        if self.finished {
            return Err(SignalError::transaction_finished());
        }
        self.finished = true;

        if self.poisoned {
            return self.fail_and_rollback(
                "poisoned transaction rollback",
                None,
                Some("transaction rolled back because it was poisoned".to_string()),
                ExecutionFailurePhase::Rollback,
                true,
                Ok(TransactionOutcome::Poisoned),
                commit_start,
            );
        }

        self.scratch.staged_patch_count = self.scratch.graph_patches.touched_count() as u64;

        if let Err(err) = self
            .event_bus
            .begin(self.runtime_ctx)
            .map_err(|e| SignalError::invalid_input(format!("event bus begin failed: {e:?}")))
        {
            return self.fail_and_rollback(
                "event bus begin failed",
                Some(err.clone()),
                None,
                ExecutionFailurePhase::CommitPromotion,
                true,
                Err(err),
                commit_start,
            );
        }
        let mut current_epoch_events = 0u32;
        let mut next_epoch_ordinal = 0u32;
        for operation in std::mem::take(&mut self.scratch.staged_event_operations) {
            match operation {
                StagedEventOperation::Emit(event) => {
                    current_epoch_events += 1;
                    self.event_bus.emit(event)
                }
                StagedEventOperation::Flush(barrier) => {
                    let flush_start = Instant::now();
                    let completed_subscribers = match self
                        .event_bus
                        .flush(barrier, self.runtime_ctx)
                    {
                        Ok(completed_subscribers) => completed_subscribers,
                        Err(flush_err) => {
                            self.scratch.staged_event_flush_nanos +=
                                flush_start.elapsed().as_nanos();
                            self.scratch
                                .semantic_delta
                                .event_epochs
                                .push(match &flush_err {
                                    EventFlushError::Registry(source) => EventEpochSummary {
                                        ordinal: next_epoch_ordinal,
                                        barrier,
                                        emitted_event_count: current_epoch_events,
                                        subscriber_count: self.event_bus.resolved_order().len()
                                            as u32,
                                        committed_subscriber_count: 0,
                                        failed_subscriber_position: None,
                                        subscriber_outcomes: Vec::new(),
                                        outcome: EventEpochOutcome::Failed,
                                        failure_subscriber: None,
                                        message: Some(format!(
                                            "subscriber registry invalid during flush: {source:?}"
                                        )),
                                    },
                                    EventFlushError::Subscriber {
                                        subscriber_name,
                                        completed_subscribers,
                                        failed_subscriber_requires,
                                        failed_subscriber_provides,
                                        failed_subscriber_staged,
                                        source,
                                        ..
                                    } => EventEpochSummary {
                                        ordinal: next_epoch_ordinal,
                                        barrier,
                                        emitted_event_count: current_epoch_events,
                                        subscriber_count: self.event_bus.resolved_order().len()
                                            as u32,
                                        committed_subscriber_count: completed_subscribers.len()
                                            as u32,
                                        failed_subscriber_position: Some(
                                            completed_subscribers.len() as u32 + 1,
                                        ),
                                        subscriber_outcomes: completed_subscribers
                                            .iter()
                                            .map(|subscriber| EventSubscriberOutcome {
                                                subscriber_name: subscriber.name.to_string(),
                                                outcome: EventSubscriberOutcomeKind::Committed,
                                                requires_data_ids: subscriber
                                                    .requires_data_ids
                                                    .clone(),
                                                provides_data_ids: subscriber
                                                    .provides_data_ids
                                                    .clone(),
                                                staged_data_ids: subscriber.staged_data_ids.clone(),
                                            })
                                            .chain(std::iter::once(EventSubscriberOutcome {
                                                subscriber_name: (*subscriber_name).to_string(),
                                                outcome: EventSubscriberOutcomeKind::Failed,
                                                requires_data_ids: failed_subscriber_requires
                                                    .clone(),
                                                provides_data_ids: failed_subscriber_provides
                                                    .clone(),
                                                staged_data_ids: failed_subscriber_staged.clone(),
                                            }))
                                            .collect(),
                                        outcome: EventEpochOutcome::Failed,
                                        failure_subscriber: Some((*subscriber_name).to_string()),
                                        message: Some(source.to_string()),
                                    },
                                });
                            let err = match &flush_err {
                                EventFlushError::Registry(source) => {
                                    SignalError::event_flush_failed(
                                        "registry",
                                        format!("{source:?}"),
                                    )
                                }
                                EventFlushError::Subscriber {
                                    subscriber_name,
                                    source,
                                    ..
                                } => SignalError::event_flush_failed(
                                    (*subscriber_name).to_string(),
                                    source.to_string(),
                                ),
                            };
                            return self.fail_and_rollback(
                                "event bus flush failed",
                                Some(err.clone()),
                                None,
                                ExecutionFailurePhase::CommitPromotion,
                                true,
                                Err(err),
                                commit_start,
                            );
                        }
                    };
                    self.scratch.staged_event_flush_nanos += flush_start.elapsed().as_nanos();
                    self.scratch
                        .semantic_delta
                        .event_epochs
                        .push(EventEpochSummary {
                            ordinal: next_epoch_ordinal,
                            barrier,
                            emitted_event_count: current_epoch_events,
                            subscriber_count: completed_subscribers.len() as u32,
                            committed_subscriber_count: completed_subscribers.len() as u32,
                            failed_subscriber_position: None,
                            subscriber_outcomes: completed_subscribers
                                .iter()
                                .map(|subscriber| EventSubscriberOutcome {
                                    subscriber_name: subscriber.name.to_string(),
                                    outcome: EventSubscriberOutcomeKind::Committed,
                                    requires_data_ids: subscriber.requires_data_ids.clone(),
                                    provides_data_ids: subscriber.provides_data_ids.clone(),
                                    staged_data_ids: subscriber.staged_data_ids.clone(),
                                })
                                .collect(),
                            outcome: EventEpochOutcome::Committed,
                            failure_subscriber: None,
                            message: None,
                        });
                    current_epoch_events = 0;
                    next_epoch_ordinal += 1;
                }
            }
        }

        while let Some(domain) = self.scratch.staged_dirty.first_dirty_domain() {
            if let Some(impact) = self.scratch.staged_dirty.take_domain_impact(domain) {
                self.checkpoint
                    .dirty_mut()
                    .merge_domain_impact(domain, impact);
            }
        }
        self.checkpoint
            .telemetry_mut()
            .checkpoint
            .checkpoint_flushes += self.scratch.staged_checkpoint_flushes;
        self.checkpoint
            .telemetry_mut()
            .checkpoint
            .checkpoint_flush_nanos += self.scratch.staged_checkpoint_flush_nanos;
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph);
        for ((family_id, key_id, memo_key_id), result) in
            std::mem::take(&mut self.scratch.staged_memo_writes)
        {
            let family = self.config.key_registry.family(family_id).clone();
            let key = self.config.key_registry.keys[key_id.index()].clone();
            let memo_key = self.config.key_registry.memo_key(memo_key_id).clone();
            self.config
                .store_memoized_result(&family, &key, &memo_key, result);
        }
        self.scratch.graph_patches.commit_and_clear();
        self.telemetry.transaction.transaction_commit_count += 1;
        self.telemetry.transaction.staged_node_patch_count += self.scratch.staged_patch_count;
        self.telemetry.transaction.max_touched_nodes_in_txn = self
            .telemetry
            .transaction
            .max_touched_nodes_in_txn
            .max(self.scratch.staged_patch_count);
        let policy = self.graph.runtime_policy();
        if policy.retains_explanation_facts() || policy.retains_provenance_facts() {
            for node in touched_nodes.as_slice().iter().copied() {
                if let Ok(explanation) = self.graph.observe().explain(node) {
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
        self.scratch
            .semantic_delta
            .replay_events
            .push(TransactionReplayEntry {
                kind: ReplayEventKind::TransactionCommitted,
                detail: "transaction committed".to_string(),
                execution_record_id: None,
                semantic_segment_id: None,
            });
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph).len() as u32;
        Ok(self.finalize_semantic_delta(
            false,
            TransactionOutcome::Committed,
            touched_nodes,
            commit_start.elapsed().as_nanos(),
        ))
    }

    pub fn rollback(mut self) -> Result<TransactionResult, SignalError> {
        let commit_start = Instant::now();
        if self.finished {
            return Err(SignalError::transaction_finished());
        }
        self.finished = true;
        let rollback_patch_count = self.rollback_patch_count();
        self.event_bus.rollback(self.runtime_ctx);
        self.rollback_graph_state()?;
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
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph).len() as u32;
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

    fn finalize_semantic_delta(
        &mut self,
        restore_baseline: bool,
        outcome: TransactionOutcome,
        touched_nodes: u32,
        commit_nanos: u128,
    ) -> TransactionResult {
        let rollback = self.scratch.semantic_delta.rollback.take();
        let failure_summary = self.scratch.semantic_delta.failure_summary.take();
        let replay_events = std::mem::take(&mut self.scratch.semantic_delta.replay_events);
        let event_epochs = std::mem::take(&mut self.scratch.semantic_delta.event_epochs);
        let execution_report = self.execution_state.latest_report.take();
        let timing = TransactionTiming {
            total_nanos: self.started_at.elapsed().as_nanos(),
            evaluation_nanos: self.execution_state.evaluation_nanos,
            event_flush_nanos: self.scratch.staged_event_flush_nanos,
            commit_nanos,
        };
        let evaluation_summary = std::mem::take(&mut self.execution_state.summary);
        let checkpoint_record =
            crate::logic::transaction::runtime::state::CheckpointRecord::from_checkpoint_telemetry(
                crate::data::telemetry::CheckpointTelemetry {
                    event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                    event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                    checkpoint_flushes: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                },
            );
        let reconstructability =
            crate::logic::transaction::runtime::state::ReconstructabilityRecord::from_transaction_boundary(
                self.graph.current_branch().id,
                self.graph.current_branch().head_snapshot_id,
                self.graph.diagnostics_state().latest_replay_cursor(),
                checkpoint_record,
                &replay_events,
            );
        // These clones are intentional second observers of finalized boundary
        // truth: the transaction result retains them, and diagnostics state may
        // also retain the same rollback/failure/epoch data after commit.
        let mut result = TransactionResult::from_boundary_state(
            outcome,
            execution_report,
            timing,
            touched_nodes,
            evaluation_summary,
            &replay_events,
            reconstructability,
            event_epochs.clone(),
            rollback.clone(),
            failure_summary.clone(),
            *self.telemetry,
        );
        self.telemetry.transaction.decision_log_event_count +=
            result.decision_log.records.len() as u64;
        self.telemetry.checkpoint.checkpoint_size +=
            result.event_epochs.len() as u64
                + u64::from(result.integrity_markers.execution_report_attached)
                + u64::from(result.integrity_markers.rollback_attached)
                + u64::from(result.integrity_markers.failure_attached);
        self.telemetry.checkpoint.journal_replay_span += result
            .reconstructability
            .journal
            .as_ref()
            .map(|journal| journal.replay_event_count as u64)
            .unwrap_or(0);
        result.reconstructability.checkpoint =
            crate::logic::transaction::runtime::state::CheckpointRecord::from_checkpoint_telemetry(
                crate::data::telemetry::CheckpointTelemetry {
                    event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                    event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                    checkpoint_flushes: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                },
            );
        result.performance_accounting = *self.telemetry;
        if restore_baseline {
            *self.config = self.baseline_config.clone();
            *self.graph.diagnostics_state_mut() = self.baseline_diagnostics_state.clone();
        }
        if let Some(rollback) = rollback {
            self.graph.diagnostics_state_mut().record_rollback(rollback);
        }
        if let Some(failure) = failure_summary {
            self.graph.diagnostics_state_mut().record_failure(failure);
        }
        self.graph
            .diagnostics_state_mut()
            .attach_event_epochs_to_latest_flow(event_epochs);
        for entry in replay_events {
            record_transaction_semantic_event(
                self.graph,
                entry.kind,
                entry.detail,
                entry.execution_record_id.map(|id| id.0),
                entry.semantic_segment_id.map(|id| id.0),
            );
        }
        result
    }

    fn rollback_graph_state(&mut self) -> Result<(), SignalError> {
        let mut rewired_sources = self
            .scratch
            .graph_patches
            .rollback_and_collect_dependency_sources_for_rollback(self.graph)?;
        for node in &self.scratch.created_nodes {
            if self.graph.is_alive(*node) {
                rewired_sources.extend(self.graph.dependency_sources_of(*node)?);
            }
        }
        self.graph
            .rollback_created_nodes(&self.scratch.created_nodes);
        self.scratch.created_nodes.clear();
        self.graph
            .reconcile_subscriber_membership_for_sources(&rewired_sources)?;
        self.scratch.dirty_targets.clear_all();
        Ok(())
    }

    fn rollback_patch_count(&self) -> u64 {
        self.scratch.graph_patches.touched_count() as u64 + self.scratch.created_nodes.len() as u64
    }

    fn fail_and_rollback(
        &mut self,
        rollback_reason: &str,
        error: Option<SignalError>,
        fallback_failure_message: Option<String>,
        failure_phase: ExecutionFailurePhase,
        increment_poison_count: bool,
        outcome: Result<TransactionOutcome, SignalError>,
        commit_start: Instant,
    ) -> Result<TransactionResult, SignalError> {
        let rollback_patch_count = self.rollback_patch_count();
        self.event_bus.rollback(self.runtime_ctx);
        self.rollback_graph_state()?;
        let touched_nodes = self.scratch.graph_patches.touched_nodes(self.graph).len() as u32;
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
