use std::collections::BTreeSet;
use std::time::Instant;

use crate::data::aspect::Aspect;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::DomainImpact;
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};

use super::transaction_types::SignalTransaction;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn staged_graph(&self) -> &crate::data::graph::SignalGraph {
        self.graph
    }

    pub fn emit_event(&mut self, event: E) {
        self.staged_events.push(event);
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
        let result = mark_dirty(self.graph, source, changed_aspect);
        self.apply_result(result)
    }

    pub fn mark_dirty_with_regions(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty_with_regions(self.graph, source, changed_aspect, changed_regions);
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
        self.staged_event_flushes.push(barrier);
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
        let mut stack = vec![source];
        let mut seen: BTreeSet<NodeId> = BTreeSet::new();
        while let Some(node) = stack.pop() {
            if !seen.insert(node) {
                continue;
            }
            if !self.graph.is_alive(node) {
                continue;
            }
            self.graph_patches.stage_original(self.graph, node)?;
            for &subscriber in self.graph.subscribers_of(node)? {
                stack.push(subscriber);
            }
        }
        Ok(())
    }

    pub(super) fn stage_evaluate_candidates(&mut self, node: NodeId) -> Result<(), SignalError> {
        let mut stack = vec![node];
        let mut seen: BTreeSet<NodeId> = BTreeSet::new();
        while let Some(current) = stack.pop() {
            if !seen.insert(current) {
                continue;
            }
            if !self.graph.is_alive(current) {
                continue;
            }
            self.graph_patches.stage_original(self.graph, current)?;
            for dependency in self.graph.dependencies_of(current)? {
                stack.push(dependency.source());
            }
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
