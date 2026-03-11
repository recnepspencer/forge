use crate::data::error::SignalError;
use crate::diagnostics::epochs::{
    EventEpochOutcome, EventEpochSummary, EventSubscriberOutcome, EventSubscriberOutcomeKind,
};
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::recorder::record_transaction_semantic_event;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase, RollbackDiagnostic};
use crate::logic::events::EventFlushError;

use super::transaction_types::{SignalTransaction, StagedEventOperation, TransactionOutcome};

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
            let rollback_patch_count = self.rollback_patch_count();
            self.event_bus.rollback(runtime_ctx);
            self.rollback_graph_state()?;
            let rollback = RollbackDiagnostic::new(
                true,
                rollback_patch_count,
                self.telemetry.max_touched_nodes_in_txn,
                Some("poisoned transaction rollback".to_string()),
                self.semantic_delta.event_epochs.clone(),
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
            let rollback_patch_count = self.rollback_patch_count();
            self.event_bus.rollback(runtime_ctx);
            self.rollback_graph_state()?;
            let rollback = RollbackDiagnostic::new(
                true,
                rollback_patch_count,
                self.telemetry.max_touched_nodes_in_txn,
                Some("event bus begin failed".to_string()),
                self.semantic_delta.event_epochs.clone(),
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
        let mut current_epoch_events = 0u32;
        let mut next_epoch_ordinal = 0u32;
        for operation in std::mem::take(&mut self.staged_event_operations) {
            match operation {
                StagedEventOperation::Emit(event) => {
                    current_epoch_events += 1;
                    self.event_bus.emit(event)
                }
                StagedEventOperation::Flush(barrier) => {
                    let completed_subscribers = match self.event_bus.flush(barrier, runtime_ctx) {
                        Ok(completed_subscribers) => completed_subscribers,
                        Err(flush_err) => {
                            self.semantic_delta.event_epochs.push(match &flush_err {
                                EventFlushError::Registry(source) => EventEpochSummary {
                                    ordinal: next_epoch_ordinal,
                                    barrier,
                                    emitted_event_count: current_epoch_events,
                                    subscriber_count: self.event_bus.resolved_order().len() as u32,
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
                                    subscriber_count: self.event_bus.resolved_order().len() as u32,
                                    committed_subscriber_count: completed_subscribers.len() as u32,
                                    failed_subscriber_position: Some(
                                        completed_subscribers.len() as u32 + 1,
                                    ),
                                    subscriber_outcomes: completed_subscribers
                                        .iter()
                                        .map(|subscriber| EventSubscriberOutcome {
                                            subscriber_name: subscriber.name.to_string(),
                                            outcome: EventSubscriberOutcomeKind::Committed,
                                            requires_data_ids: subscriber.requires_data_ids.clone(),
                                            provides_data_ids: subscriber.provides_data_ids.clone(),
                                            staged_data_ids: subscriber.staged_data_ids.clone(),
                                        })
                                        .chain(std::iter::once(EventSubscriberOutcome {
                                            subscriber_name: (*subscriber_name).to_string(),
                                            outcome: EventSubscriberOutcomeKind::Failed,
                                            requires_data_ids: failed_subscriber_requires.clone(),
                                            provides_data_ids: failed_subscriber_provides.clone(),
                                            staged_data_ids: failed_subscriber_staged.clone(),
                                        }))
                                        .collect(),
                                    outcome: EventEpochOutcome::Failed,
                                    failure_subscriber: Some((*subscriber_name).to_string()),
                                    message: Some(source.to_string()),
                                },
                            });
                            let err = SignalError::invalid_input(format!(
                                "event bus flush failed: {flush_err:?}"
                            ));
                            let rollback_patch_count = self.rollback_patch_count();
                            self.event_bus.rollback(runtime_ctx);
                            self.rollback_graph_state()?;
                            let rollback = RollbackDiagnostic::new(
                                true,
                                rollback_patch_count,
                                self.telemetry.max_touched_nodes_in_txn,
                                Some("event bus flush failed".to_string()),
                                self.semantic_delta.event_epochs.clone(),
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
                    };
                    self.semantic_delta.event_epochs.push(EventEpochSummary {
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
        let rollback_patch_count = self.rollback_patch_count();
        self.event_bus.rollback(runtime_ctx);
        self.rollback_graph_state()?;
        if !self.poisoned {
            self.telemetry.transaction_rollback_count += 1;
        }
        let rollback = RollbackDiagnostic::new(
            true,
            rollback_patch_count,
            self.telemetry.max_touched_nodes_in_txn,
            Some(if self.poisoned {
                "poisoned transaction rollback".to_string()
            } else {
                "explicit rollback".to_string()
            }),
            self.semantic_delta.event_epochs.clone(),
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
        self.graph
            .diagnostics_state_mut()
            .attach_event_epochs_to_latest_flow(self.semantic_delta.event_epochs.clone());
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
        self.graph.compact_graph_storage();
        self.dirty_targets.clear_all();
        Ok(())
    }

    fn rollback_patch_count(&self) -> u64 {
        self.graph_patches.touched_count() as u64 + self.created_nodes.len() as u64
    }
}
