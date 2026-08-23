use crate::clock::RuntimeInstant;
use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{
    admit_direct_task_with_policy_resolver, ExecutionReport, StageExecutor,
};

use super::super::super::transaction::SignalTransaction;
use super::super::shared::{
    absorb_execution_report_telemetry, execute_targets_with_runtime_config_detailed,
};

use super::request::TransactionExecutionIntent;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub(super) fn execute_evaluation<F, O>(
        &mut self,
        intent: TransactionExecutionIntent<'_>,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let owned_targets;
        let (targets, request_mode) = match intent {
            TransactionExecutionIntent::Targets {
                targets,
                request_mode,
                stage_task_candidates,
            } => {
                if stage_task_candidates {
                    let mut comparator = DefaultComparatorResolver;
                    let mut resolver = DefaultComparatorPolicyResolver {
                        fallback: VersionComparatorPolicy::Exact,
                        custom: &mut comparator,
                    };
                    let stage_targets = targets
                        .iter()
                        .copied()
                        .map(|node| {
                            admit_direct_task_with_policy_resolver(
                                &*self.graph,
                                node,
                                request_mode,
                                &mut resolver,
                            )
                        })
                        .collect::<Result<Vec<_>, SignalError>>()?;
                    self.stage_task_candidates(&stage_targets)?;
                } else {
                    self.stage_evaluate_candidate_batch(targets)?;
                }
                (targets, request_mode)
            }
            TransactionExecutionIntent::Dirty => {
                owned_targets = self.collect_dirty_targets();
                if owned_targets.is_empty() {
                    return Ok(crate::logic::transaction::helpers::empty_execution_report());
                }
                let mut comparator = DefaultComparatorResolver;
                let mut resolver = DefaultComparatorPolicyResolver {
                    fallback: VersionComparatorPolicy::Exact,
                    custom: &mut comparator,
                };
                let stage_targets = owned_targets
                    .iter()
                    .copied()
                    .map(|node| {
                        admit_direct_task_with_policy_resolver(
                            &*self.graph,
                            node,
                            EvaluationRequestMode::Default,
                            &mut resolver,
                        )
                    })
                    .collect::<Result<Vec<_>, SignalError>>()?;
                self.stage_task_candidates(&stage_targets)?;
                (&owned_targets[..], EvaluationRequestMode::Default)
            }
        };

        self.admit_temporal_wakes_for_nodes(targets)?;
        self.promote_due_temporal_wakes_ready()?;
        let temporal_lowering = self.temporal_lowering_context_for_nodes(targets);
        let execution_start = RuntimeInstant::now();
        let report = match execute_targets_with_runtime_config_detailed(
            self.graph,
            self.config,
            temporal_lowering,
            &*self.runtime_ctx,
            targets,
            request_mode,
            evaluator,
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
        self.scratch.temporal.absorb_report(&report);
        self.lower_observation_classifications_from_report(&report)?;
        self.with_telemetry(|telemetry| absorb_execution_report_telemetry(telemetry, &report));
        self.retire_consumed_temporal_wakes_from_report(&report)?;
        Ok(report)
    }
}
