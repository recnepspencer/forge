use crate::clock::RuntimeInstant;
use crate::data::aspect::Aspect;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::DomainImpact;
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::{DirtyBatch, SemanticBatchCommit};
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::invalidation::mark_dirty_batch;

use super::transaction_observation::{
    ClassifiedObservationEventSummary, ObservationScratchSummary,
};
use super::transaction_types::{
    BatchChangeSession, SignalTransaction, StagedEventOperation, TransactionReplayEntry,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) fn ensure_rollback_packets(&mut self) {
        self.rollback_packets
            .capture_runtime_baseline_if_needed(self.config, self.graph.diagnostics_state());
    }

    pub fn staged_graph(&self) -> &crate::data::graph::SignalGraph {
        self.graph
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn observation_scratch_summary(&self) -> ObservationScratchSummary {
        self.scratch.observations.summary()
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn classified_observation_summaries(
        &self,
    ) -> Vec<ClassifiedObservationEventSummary> {
        self.scratch.observations.classified_summaries()
    }

    pub(in crate::logic::transaction::runtime) fn resolve_defined_node(
        &mut self,
        family: &crate::data::output::ComputationFamily,
        key: impl Into<crate::data::output::ComputationKey>,
    ) -> NodeId {
        self.ensure_rollback_packets();
        let (node, created) = self
            .config
            .resolve_defined_node_with_created(self.graph, family, key);
        if created {
            self.scratch.created_nodes.push(node);
        }
        node
    }

    pub fn emit_event(&mut self, event: E) {
        self.scratch
            .staged_event_operations
            .push(StagedEventOperation::Emit(event));
    }

    pub fn record_effect<M>(&mut self, effect: &M::Effect)
    where
        M: EffectMapping<Domain = D, Impact = I>,
    {
        M::route(effect, &mut self.scratch.staged_dirty);
    }

    #[doc(hidden)]
    pub fn mark_dirty(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        let result = self.mark_dirty_batch(&DirtyBatch::singleton(
            source,
            changed_aspect,
            Vec::<ChangedRegion>::new(),
        ));
        self.apply_result(result).map(|_| ())
    }

    pub fn mark_changed(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.mark_dirty(source, changed_aspect)
    }

    #[doc(hidden)]
    pub fn mark_dirty_with_regions(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        let result = self.mark_dirty_batch(&DirtyBatch::singleton(
            source,
            changed_aspect,
            changed_regions.to_vec(),
        ));
        self.apply_result(result).map(|_| ())
    }

    pub fn mark_changed_with_regions(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        self.mark_dirty_with_regions(source, changed_aspect, changed_regions)
    }

    pub fn mark_dirty_batch(
        &mut self,
        dirty: &DirtyBatch,
    ) -> Result<SemanticBatchCommit, SignalError> {
        self.ensure_rollback_packets();
        for entry in dirty.as_slice() {
            self.stage_mark_dirty_candidates(entry.source)?;
        }
        mark_dirty_batch(&mut *self.graph, dirty)
    }

    pub fn apply_batch_changes(
        &mut self,
        dirty: &DirtyBatch,
    ) -> Result<SemanticBatchCommit, SignalError> {
        self.mark_dirty_batch(dirty)
    }

    pub fn batch_changes<'tx>(&'tx mut self) -> BatchChangeSession<'tx, 'a, D, I, E, Ctx, T> {
        BatchChangeSession::new(self)
    }

    pub fn flush_checkpoint<Ev>(
        &mut self,
        barrier: CheckpointBarrier,
        evaluator: &mut Ev,
        ctx: &mut Ev::Context,
    ) -> Result<usize, SignalError>
    where
        Ev: CheckpointEvaluator<Domain = D, Impact = I>,
    {
        let flush_start = RuntimeInstant::now();
        let domains: Vec<D> = self
            .scratch
            .staged_dirty
            .dirty_domains()
            .filter(|domain| self.checkpoint.policy().barrier_for(*domain) == barrier)
            .collect();

        for domain in &domains {
            let impact = self
                .scratch
                .staged_dirty
                .take_domain_impact(*domain)
                .unwrap_or_else(DomainImpact::empty);
            evaluator.refresh(*domain, impact, ctx)?;
        }

        self.scratch.staged_checkpoint_flushes += 1;
        self.scratch.staged_checkpoint_flush_nanos += flush_start.elapsed().as_nanos();
        Ok(domains.len())
    }

    pub fn flush_events(&mut self, barrier: CheckpointBarrier) -> Result<(), SignalError> {
        self.scratch
            .staged_event_operations
            .push(StagedEventOperation::Flush(barrier));
        Ok(())
    }

    pub(in crate::logic::transaction::runtime) fn apply_result<R>(
        &mut self,
        result: Result<R, SignalError>,
    ) -> Result<R, SignalError> {
        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                self.poisoned = true;
                Err(err)
            }
        }
    }

    pub(in crate::logic::transaction::runtime) fn stage_mark_dirty_candidates(
        &mut self,
        source: NodeId,
    ) -> Result<(), SignalError> {
        self.scratch
            .mark_dirty_staged
            .ensure_len(self.graph.arena_capacity());
        if self
            .scratch
            .mark_dirty_staged
            .contains(source.index() as usize)
        {
            return Ok(());
        }

        let mut stack = vec![source];
        self.scratch.mark_dirty_seen.clear_all();
        self.scratch
            .mark_dirty_seen
            .ensure_len(self.graph.arena_capacity());
        while let Some(node) = stack.pop() {
            if !self.scratch.mark_dirty_seen.mark(node.index() as usize) {
                continue;
            }
            self.telemetry
                .transaction
                .transaction_mark_dirty_candidate_visits += 1;
            if !self.graph.is_alive(node) {
                continue;
            }
            self.scratch.mark_dirty_staged.mark(node.index() as usize);
            self.scratch.dirty_targets.mark(node.index() as usize);
            self.scratch
                .graph_patches
                .stage_original(self.graph, node)?;
            self.stage_observation_candidates_for_node(node);
            for &subscriber in self.graph.runtime_subscribers_of(node)? {
                stack.push(subscriber);
            }
        }
        Ok(())
    }

    pub(in crate::logic::transaction::runtime) fn stage_evaluate_candidates(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        self.ensure_rollback_packets();
        let mut stack = vec![node];
        self.scratch.evaluate_seen.clear_all();
        self.scratch
            .evaluate_seen
            .ensure_len(self.graph.arena_capacity());
        self.scratch
            .dirty_targets
            .ensure_len(self.graph.arena_capacity());
        while let Some(current) = stack.pop() {
            if !self.scratch.evaluate_seen.mark(current.index() as usize) {
                continue;
            }
            if !self.graph.is_alive(current) {
                continue;
            }
            self.scratch.dirty_targets.mark(current.index() as usize);
            self.scratch
                .graph_patches
                .stage_original(self.graph, current)?;
            self.stage_observation_candidates_for_node(current);
            for dependency in self.graph.runtime_dependencies_of(current)? {
                stack.push(dependency.source());
            }
        }
        Ok(())
    }

    pub(in crate::logic::transaction::runtime) fn stage_task_candidates(
        &mut self,
        tasks: &[crate::logic::planner::EligibleTask],
    ) -> Result<(), SignalError> {
        for task in tasks {
            self.stage_evaluate_candidates(task.node)?;
        }
        Ok(())
    }

    pub(in crate::logic::transaction::runtime) fn record_failure_from_error(
        &mut self,
        phase: ExecutionFailurePhase,
        err: &SignalError,
        plan_summary: Option<crate::logic::planner::PlanSummary>,
    ) {
        let summary = ExecutionFailureContext::from_error(phase, err, plan_summary)
            .summarize(None, self.graph.diagnostics_profile());
        self.scratch.semantic_delta.failure_summary = Some(summary);
        self.scratch
            .semantic_delta
            .replay_events
            .push(TransactionReplayEntry {
                kind: ReplayEventKind::FailureRecorded,
                detail: err.to_string(),
                execution_record_id: None,
                semantic_segment_id: None,
            });
    }

    fn stage_observation_candidates_for_node(&mut self, node: NodeId) {
        let before_candidate_count = self.scratch.observations.staged_candidate_count();
        let mut staged_match_count = 0_u64;
        self.observations
            .for_each_matching_observer_for_node(node, |observer_id| {
                staged_match_count += 1;
                self.scratch
                    .observations
                    .stage_match(self.observations, observer_id, node);
            });
        let after_candidate_count = self.scratch.observations.staged_candidate_count();
        self.telemetry.transaction.staged_observation_match_count += staged_match_count;
        self.telemetry
            .transaction
            .staged_observation_candidate_count +=
            after_candidate_count.saturating_sub(before_candidate_count) as u64;
    }

    pub(in crate::logic::transaction::runtime) fn lower_observation_classifications_from_report(
        &mut self,
        report: &crate::logic::planner::ExecutionReport,
    ) -> Result<(), SignalError> {
        let before_classified_count = self.scratch.observations.classified_event_count();
        self.scratch
            .observations
            .lower_classifications(self.graph, report)?;
        let summary = self.scratch.observations.summary();
        let newly_classified = summary
            .classified_event_count
            .saturating_sub(before_classified_count);
        self.telemetry.transaction.classified_observation_count += newly_classified as u64;
        self.telemetry
            .transaction
            .observation_classification_breadth += report
            .stages
            .iter()
            .map(|stage| stage.task_records.len() as u64)
            .sum::<u64>();
        Ok(())
    }
}

impl<'tx, 'a, D, I, E, Ctx, T> BatchChangeSession<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn apply(mut self) -> Result<SemanticBatchCommit, SignalError> {
        let batch = DirtyBatch::new(self.entries.drain(..));
        let result = self.tx.apply_batch_changes(&batch);
        self.applied = true;
        result
    }
}
