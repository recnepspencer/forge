use std::sync::Mutex;
use std::time::Instant;

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::KeyedComputation;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::prepared::{
    PreparedEvaluationOrigin, PreparedKeyedContext, PreparedMemoDecision,
};

use super::super::transaction::SignalTransaction;
use super::shared::{
    absorb_execution_report_telemetry, execute_targets_with_prepared_runtime_config_detailed,
    executor_for_strategy,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn evaluate_keyed<F, O>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        evaluator: &F,
    ) -> Result<(), SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.evaluate_keyed_with_mode(node, computation, evaluator, EvaluationRequestMode::Default)
    }

    pub fn evaluate_keyed_with_mode<F, O>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        evaluator: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.graph.derive_evaluation_strategy();
        let executor = executor_for_strategy(strategy);
        self.telemetry.invalidation.keyed_evaluation_count += 1;
        self.stage_evaluate_candidates(node)?;
        let family_id = self.config.key_registry.intern_family(&computation.family);
        let key_id = self.config.key_registry.intern_key(&computation.key);
        let memo_key_id = computation
            .memo_key
            .as_ref()
            .map(|memo_key| self.config.key_registry.intern_memo_key(memo_key));
        let base_keyed_context = PreparedKeyedContext {
            family: Some(computation.family.clone()),
            key: Some(computation.key.clone()),
            memo_key: computation.memo_key.clone(),
            memoized_origin: crate::data::output::MemoizedResultOrigin::DirectCompute,
        };
        if let Some(memo_key) = computation.memo_key.as_ref() {
            if let Some(cached) = self
                .scratch
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
                self.telemetry.evaluation.memoization_hits += 1;
                let cached_result = cached;
                let execution_start = Instant::now();
                let report = match execute_targets_with_prepared_runtime_config_detailed(
                    self.graph,
                    self.config,
                    &[node],
                    request_mode,
                    &|_current, _view| {
                        Ok(crate::logic::prepared::PreparedEvaluation::from_result(
                            cached_result.clone(),
                        )
                        .with_origin(PreparedEvaluationOrigin::MemoizedReuse)
                        .with_memo_decision(PreparedMemoDecision::Hit)
                        .with_keyed(PreparedKeyedContext {
                            memoized_origin:
                                crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
                            ..base_keyed_context.clone()
                        }))
                    },
                    executor,
                ) {
                    Ok(report) => report,
                    Err(failure) => {
                        let err = failure.error;
                        self.record_failure_from_error(
                            ExecutionFailurePhase::Apply,
                            &err,
                            Some(failure.plan_summary),
                        );
                        return Err(err);
                    }
                };
                self.execution_state
                    .record_report(&report, execution_start.elapsed().as_nanos());
                absorb_execution_report_telemetry(self.telemetry, &report);
                return self.apply_result(Ok(()));
            }
            self.telemetry.evaluation.memoization_misses += 1;
        }

        let last_result = Mutex::new(None);
        let execution_start = Instant::now();
        let result = match execute_targets_with_prepared_runtime_config_detailed(
            self.graph,
            self.config,
            &[node],
            request_mode,
            &|current, view| {
                let mut ctx = EvaluationContext::new(view.graph(), current, &*self.runtime_ctx);
                let output = evaluator(&mut ctx)?;
                let prepared = ctx
                    .into_prepared(output)
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
            executor,
        ) {
            Ok(report) => Ok(report),
            Err(failure) => {
                let err = failure.error;
                self.record_failure_from_error(
                    ExecutionFailurePhase::Apply,
                    &err,
                    Some(failure.plan_summary),
                );
                Err(err)
            }
        };
        let result = match result {
            Ok(report) => {
                self.execution_state
                    .record_report(&report, execution_start.elapsed().as_nanos());
                absorb_execution_report_telemetry(self.telemetry, &report);
                self.apply_result(Ok(()))
            }
            Err(err) => self.apply_result(Err(err)),
        };
        if result.is_ok() {
            if let Ok(mut guard) = last_result.lock() {
                if let Some(last_result) = guard.take() {
                    self.scratch.staged_memo_writes.insert(
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
