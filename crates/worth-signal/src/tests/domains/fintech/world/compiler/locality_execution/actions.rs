use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::data::node::NodeState;
use crate::data::output::ChangedRegion;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::invalidation::scheduling::merge_repeated_current_admission;
use crate::logic::planner::StageExecutor;

use super::super::locality_evaluation::runtime_shocked_values;
use super::{
    signal_aspect, CompiledFinancialLocalityWorld, FinancialLocalityRedObservation,
    LocalityEvaluationProgram, LocalitySemanticOutputId, RedObservationInput,
};
use crate::tests::domains::fintech::world::FinancialLocalityAction;
use crate::tests::domains::fintech::world::FinancialLocalityMutation;
use crate::tests::domains::fintech::world::FinancialLocalityScenario;

mod churn;
mod churn_program;
mod restore;

pub(in crate::tests::domains::fintech) use restore::FinancialRestoreLifecycleEvidence;

impl CompiledFinancialLocalityWorld {
    pub(super) fn certify_restore_lifecycle(
        &mut self,
    ) -> Result<FinancialRestoreLifecycleEvidence, SignalError> {
        restore::certify_restore_lifecycle(self)
    }

    pub(in crate::tests::domains::fintech) fn run_action_trace(
        &mut self,
        trace_index: usize,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        let executor = self
            .runtime
            .derive_evaluation_strategy()
            .parallelism
            .stage_executor();
        self.run_action_trace_with_executor(trace_index, executor)
    }

    pub(in crate::tests::domains::fintech) fn run_action_trace_with_executor(
        &mut self,
        trace_index: usize,
        executor: StageExecutor,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.runtime
            .graph_mut()
            .reset_invalidation_performed_counters();
        let before = self.runtime.graph().telemetry().invalidation;
        let evaluation_before = self.runtime.graph().telemetry().evaluation;
        let trace = &self.locality_definition().action_traces()[trace_index];
        let mutations = trace.committed_mutations();
        let retry_targets = trace
            .actions()
            .iter()
            .filter_map(|action| match action {
                FinancialLocalityAction::RetryAdmission { target, .. } => Some(*target),
                _ => None,
            })
            .collect::<Vec<_>>();
        let evaluated_outputs = if self.locality_definition().scenario()
            == FinancialLocalityScenario::PortfolioDependencyChurn
        {
            churn::run_churn_trace(self, trace_index, executor)?
        } else {
            self.apply_mutations(&mutations)?;
            self.settle_mutations_with_retries(&mutations, &retry_targets, executor)?
        };
        let after = self.runtime.graph().telemetry().invalidation;
        let evaluation_after = self.runtime.graph().telemetry().evaluation;
        let baseline_retained_outputs = self.baseline_retained_outputs(&evaluated_outputs)?;
        Ok(self.red_observation(RedObservationInput {
            before,
            after,
            evaluation_before,
            evaluation_after,
            evaluated_outputs,
            baseline_retained_outputs,
            performed: self.runtime.graph().invalidation_performed_counters(),
        }))
    }

    pub(super) fn settle_mutations(
        &mut self,
        mutations: &[FinancialLocalityMutation],
    ) -> Result<BTreeSet<LocalitySemanticOutputId>, SignalError> {
        let executor = self
            .runtime
            .derive_evaluation_strategy()
            .parallelism
            .stage_executor();
        self.settle_mutations_with_retries(mutations, &[], executor)
    }

    fn settle_mutations_with_retries(
        &mut self,
        mutations: &[FinancialLocalityMutation],
        retry_targets: &[LocalitySemanticOutputId],
        executor: StageExecutor,
    ) -> Result<BTreeSet<LocalitySemanticOutputId>, SignalError> {
        let shocked_values =
            runtime_shocked_values(self.locality_definition(), &self.baseline_values, mutations)?;
        let program = LocalityEvaluationProgram::shocked(
            self.locality_definition(),
            &self.handles,
            &self.baseline_values,
            &shocked_values,
            mutations,
        );
        let evaluator = |view: &mut EvaluationContext<'_, ()>| program.evaluate(view);
        for mutation in mutations {
            let source = self.handles[&mutation.producer];
            self.runtime.transaction(&mut (), |tx| {
                tx.read_with_executor(source, &evaluator, executor)
                    .map(|_| ())
            })?;
        }
        for target in retry_targets {
            merge_repeated_current_admission(&mut self.runtime.graph_mut(), self.handles[target])?;
        }
        let release_waves = self
            .locality_definition()
            .workload()
            .release_waves()
            .to_vec();
        for wave in release_waves {
            let nodes = wave
                .iter()
                .map(|output| self.handles[output])
                .filter(|node| {
                    self.runtime
                        .graph()
                        .get_state(*node)
                        .is_ok_and(|state| !matches!(state, NodeState::Clean))
                })
                .collect::<Vec<_>>();
            if nodes.is_empty() {
                continue;
            }
            let plan = self
                .runtime
                .graph_mut()
                .build_evaluation_plan(&nodes, EvaluationRequestMode::Default)?;
            self.runtime
                .execute_prepared_plan_with_executor(&plan, &(), &evaluator, executor)?;
        }
        Ok(program.evaluated_outputs())
    }

    pub(super) fn apply_mutations(
        &mut self,
        mutations: &[FinancialLocalityMutation],
    ) -> Result<(), SignalError> {
        for mutation in mutations {
            let source = self.handles[&mutation.producer];
            let aspect = signal_aspect(mutation.aspect);
            let region = mutation.scope.map(|scope| {
                let mut region = ChangedRegion::new(scope.partition_label());
                if let Some(detail) = scope.detail_label() {
                    region = region.with_detail(detail);
                }
                region
            });
            self.runtime
                .transaction(&mut (), |tx| match region.as_ref() {
                    None => tx.mark_changed(source, aspect),
                    Some(region) => {
                        tx.mark_changed_with_regions(source, aspect, std::slice::from_ref(region))
                    }
                })?;
        }
        Ok(())
    }
}
