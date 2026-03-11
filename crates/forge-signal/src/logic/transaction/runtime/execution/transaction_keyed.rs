use std::sync::Mutex;

use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::KeyedComputation;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_prepared_plan_with_policy, StageExecutor,
};
use crate::logic::prepared::{
    ExecutionReadView, PreparedEvaluation, PreparedEvaluationOrigin, PreparedKeyedContext,
    PreparedMemoDecision,
};

use super::super::transaction::SignalTransaction;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn evaluate_keyed<F>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        precompute: &F,
    ) -> Result<(), SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_keyed_with_mode(
            node,
            computation,
            precompute,
            EvaluationRequestMode::Default,
        )
    }

    pub fn evaluate_keyed_with_mode<F>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.telemetry.keyed_evaluation_count += 1;
        self.stage_evaluate_candidates(node)?;
        let family_id = self.config.key_registry.intern_family(&computation.family);
        let key_id = self.config.key_registry.intern_key(&computation.key);
        let memo_key_id = computation
            .memo_key
            .as_ref()
            .map(|memo_key| self.config.key_registry.intern_memo_key(memo_key));
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let plan = match build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        ) {
            Ok(plan) => plan,
            Err(err) => {
                self.record_failure_from_error(ExecutionFailurePhase::Planning, &err, None);
                return Err(err);
            }
        };
        let base_keyed_context = PreparedKeyedContext {
            family: Some(computation.family.clone()),
            key: Some(computation.key.clone()),
            memo_key: computation.memo_key.clone(),
            memoized_origin: crate::data::output::MemoizedResultOrigin::DirectCompute,
        };
        if let Some(memo_key) = computation.memo_key.as_ref() {
            if let Some(cached) = self
                .staged_memo_writes
                .get(&(
                    family_id,
                    key_id,
                    memo_key_id.expect("memo key id should exist"),
                ))
                .cloned()
                .or_else(|| {
                    self.config.lookup_memoized_result(
                        &computation.family,
                        &computation.key,
                        memo_key,
                    )
                })
            {
                self.telemetry.memoization_hits += 1;
                let cached_result = cached.clone();
                let report = match execute_prepared_plan_with_policy(
                    self.graph,
                    &plan,
                    &|_current, _view| {
                        Ok(PreparedEvaluation::from_result(cached_result.clone())
                            .with_origin(PreparedEvaluationOrigin::MemoizedReuse)
                            .with_memo_decision(PreparedMemoDecision::Hit)
                            .with_keyed(PreparedKeyedContext {
                                memoized_origin:
                                    crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
                                ..base_keyed_context.clone()
                            }))
                    },
                    &mut resolver,
                    StageExecutor::Serial,
                ) {
                    Ok(report) => report,
                    Err(err) => {
                        self.record_failure_from_error(
                            ExecutionFailurePhase::Apply,
                            &err,
                            Some(plan.summary.clone()),
                        );
                        return Err(err);
                    }
                };
                self.absorb_execution_report_telemetry(&report);
                return self.apply_result(Ok(()));
            }
            self.telemetry.memoization_misses += 1;
        }

        let last_result = Mutex::new(None);
        let result = match execute_prepared_plan_with_policy(
            self.graph,
            &plan,
            &|current, view| {
                let prepared = precompute(current, view)?
                    .with_memo_decision(PreparedMemoDecision::Miss)
                    .with_keyed(base_keyed_context.clone());
                if current == node {
                    let mut guard = last_result
                        .lock()
                        .map_err(|_| SignalError::internal("memo capture mutex poisoned"))?;
                    *guard = Some(prepared.result.clone());
                }
                Ok(prepared)
            },
            &mut resolver,
            StageExecutor::Serial,
        ) {
            Ok(report) => Ok(report),
            Err(err) => {
                self.record_failure_from_error(
                    ExecutionFailurePhase::Apply,
                    &err,
                    Some(plan.summary.clone()),
                );
                Err(err)
            }
        };
        let result = match result {
            Ok(report) => {
                self.absorb_execution_report_telemetry(&report);
                self.apply_result(Ok(()))
            }
            Err(err) => self.apply_result(Err(err)),
        };
        if result.is_ok() {
            if let Ok(mut guard) = last_result.lock() {
                if let Some(last_result) = guard.take() {
                    self.staged_memo_writes.insert(
                        (
                            family_id,
                            key_id,
                            memo_key_id.expect("memo key id should exist when memo key exists"),
                        ),
                        last_result,
                    );
                }
            }
        }
        result
    }
}
