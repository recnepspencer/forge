use std::time::Instant;

use crate::data::aspect::Aspect;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::DomainImpact;
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::output::{ComputationFamily, ComputationKey};
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};

use super::transaction_types::{SignalTransaction, StagedEventOperation};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn staged_graph(&self) -> &crate::data::graph::SignalGraph {
        self.graph
    }

    pub fn register_computation_family(
        &mut self,
        family: impl Into<ComputationFamily>,
    ) -> ComputationFamily {
        self.config.register_computation_family(family)
    }

    pub fn keyed_node(
        &mut self,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> NodeId {
        let (node, created) = self.config.keyed_node_with_created(self.graph, family, key);
        if created {
            self.created_nodes.push(node);
        }
        node
    }

    pub fn emit_event(&mut self, event: E) {
        self.staged_event_operations
            .push(StagedEventOperation::Emit(event));
    }

    pub fn record_effect<M>(&mut self, effect: &M::Effect)
    where
        M: EffectMapping<Domain = D, Impact = I>,
    {
        M::route(effect, &mut self.staged_dirty);
    }

    pub fn mark_dirty(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty(&mut *self.graph, source, changed_aspect);
        self.apply_result(result)
    }

    pub fn mark_dirty_with_regions(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result =
            mark_dirty_with_regions(&mut *self.graph, source, changed_aspect, changed_regions);
        self.apply_result(result)
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
        let flush_start = Instant::now();
        let domains: Vec<D> = self
            .staged_dirty
            .dirty_domains()
            .filter(|domain| self.checkpoint.policy().barrier_for(*domain) == barrier)
            .collect();

        for domain in &domains {
            let impact = self
                .staged_dirty
                .take_domain_impact(*domain)
                .unwrap_or_else(DomainImpact::empty);
            evaluator.refresh(*domain, impact, ctx)?;
        }

        self.staged_checkpoint_flushes += 1;
        self.staged_checkpoint_flush_nanos += flush_start.elapsed().as_nanos();
        Ok(domains.len())
    }

    pub fn flush_events(&mut self, barrier: CheckpointBarrier) -> Result<(), SignalError> {
        self.staged_event_operations
            .push(StagedEventOperation::Flush(barrier));
        Ok(())
    }

    pub(super) fn apply_result<R>(
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

    pub(super) fn stage_mark_dirty_candidates(
        &mut self,
        source: NodeId,
    ) -> Result<(), SignalError> {
        self.mark_dirty_staged
            .ensure_len(self.graph.arena_capacity());
        if self.mark_dirty_staged.contains(source.index() as usize) {
            return Ok(());
        }

        let mut stack = vec![source];
        self.mark_dirty_seen.clear_all();
        self.mark_dirty_seen.ensure_len(self.graph.arena_capacity());
        while let Some(node) = stack.pop() {
            if !self.mark_dirty_seen.mark(node.index() as usize) {
                continue;
            }
            self.telemetry.transaction_mark_dirty_candidate_visits += 1;
            if !self.graph.is_alive(node) {
                continue;
            }
            self.mark_dirty_staged.mark(node.index() as usize);
            self.dirty_targets.mark(node.index() as usize);
            self.graph_patches.stage_original(self.graph, node)?;
            for &subscriber in self.graph.runtime_subscribers_of(node)? {
                stack.push(subscriber);
            }
        }
        Ok(())
    }

    pub(super) fn stage_evaluate_candidates(&mut self, node: NodeId) -> Result<(), SignalError> {
        let mut stack = vec![node];
        self.evaluate_seen.clear_all();
        self.evaluate_seen.ensure_len(self.graph.arena_capacity());
        self.dirty_targets.ensure_len(self.graph.arena_capacity());
        while let Some(current) = stack.pop() {
            if !self.evaluate_seen.mark(current.index() as usize) {
                continue;
            }
            if !self.graph.is_alive(current) {
                continue;
            }
            self.dirty_targets.mark(current.index() as usize);
            self.graph_patches.stage_original(self.graph, current)?;
            for dependency in self.graph.runtime_dependencies_of(current)? {
                stack.push(dependency.source());
            }
        }
        Ok(())
    }

    pub(super) fn stage_task_candidates(
        &mut self,
        tasks: &[crate::logic::planner::EvaluationTask],
    ) -> Result<(), SignalError> {
        for task in tasks {
            self.stage_evaluate_candidates(task.node)?;
        }
        Ok(())
    }

    pub(super) fn record_failure_from_error(
        &mut self,
        phase: ExecutionFailurePhase,
        err: &SignalError,
        plan_summary: Option<crate::logic::planner::PlanSummary>,
    ) {
        let summary = ExecutionFailureContext::from_error(phase, err, plan_summary)
            .summarize(None, self.graph.diagnostics_profile());
        self.semantic_delta.failure_summary = Some(summary);
        self.semantic_delta.replay_events.push((
            ReplayEventKind::FailureRecorded,
            err.to_string(),
            None,
            None,
        ));
    }
}
