use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::data::node::NodeState;
use crate::data::output::ChangedRegion;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::invalidation::scheduling::merge_repeated_current_admission;
use crate::logic::planner::{StageExecutionOutcome, StageExecutionRecord, StageExecutor};

use super::super::locality_evaluation::runtime_shocked_values_for_batch;
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

pub(super) struct LocalityExecutionSettlement {
    pub(super) evaluated_outputs: BTreeSet<LocalitySemanticOutputId>,
    pub(super) stage_outcomes: Vec<StageExecutionOutcome>,
    pub(super) stage_records: Vec<StageExecutionRecord>,
}

impl CompiledFinancialLocalityWorld {
    pub(in crate::tests::domains::fintech::world::compiler) fn certify_restore_lifecycle(
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
        let settlement = if self.locality_definition().scenario()
            == FinancialLocalityScenario::PortfolioDependencyChurn
        {
            churn::run_churn_trace(self, trace_index, executor)?
        } else {
            self.apply_mutations(&mutations)?;
            self.settle_mutations_with_retries(&mutations, &retry_targets, executor)?
        };
        let after = self.runtime.graph().telemetry().invalidation;
        let evaluation_after = self.runtime.graph().telemetry().evaluation;
        let baseline_retained_outputs =
            self.baseline_retained_outputs(&settlement.evaluated_outputs)?;
        let graph = self.runtime.graph();
        let explanation_fact_count = self
            .handles
            .values()
            .filter(|node| graph.observe().explanation_fact(**node).is_some())
            .count();
        let provenance_fact_count = self
            .handles
            .values()
            .filter(|node| graph.observe().provenance_fact(**node).is_some())
            .count();
        Ok(self.red_observation(RedObservationInput {
            before,
            after,
            evaluation_before,
            evaluation_after,
            evaluated_outputs: settlement.evaluated_outputs,
            baseline_retained_outputs,
            performed: self.runtime.graph().invalidation_performed_counters(),
            execution_stage_outcomes: settlement.stage_outcomes,
            lineage_records: self.runtime.graph().observe().lineage_records().len(),
            explanation_fact_count,
            provenance_fact_count,
            frontier_summary_retained: graph
                .observe()
                .latest_frontier_execution_summary()
                .is_some(),
            replay_event_count: graph.observe().replay_events().len(),
            flow_summary_retained: graph.observe().latest_flow_diagnostics().is_some(),
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
            .map(|settlement| settlement.evaluated_outputs)
    }

    pub(super) fn settle_mutations_with_retries(
        &mut self,
        mutations: &[FinancialLocalityMutation],
        retry_targets: &[LocalitySemanticOutputId],
        executor: StageExecutor,
    ) -> Result<LocalityExecutionSettlement, SignalError> {
        self.settle_mutations_with_retries_at_batch(mutations, retry_targets, executor, 0)
    }

    pub(super) fn settle_mutations_with_retries_at_batch(
        &mut self,
        mutations: &[FinancialLocalityMutation],
        retry_targets: &[LocalitySemanticOutputId],
        executor: StageExecutor,
        batch_index: usize,
    ) -> Result<LocalityExecutionSettlement, SignalError> {
        let shocked_values = runtime_shocked_values_for_batch(
            self.locality_definition(),
            &self.baseline_values,
            mutations,
            batch_index,
        )?;
        let program = LocalityEvaluationProgram::shocked_for_batch(
            self.locality_definition(),
            &self.handles,
            &self.baseline_values,
            &shocked_values,
            mutations,
            batch_index,
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
        let mut stage_outcomes = Vec::new();
        let mut stage_records = Vec::new();
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
            let report = self.runtime.execute_prepared_plan_with_executor(
                &plan,
                &(),
                &evaluator,
                executor,
            )?;
            for stage in report.stages {
                stage_outcomes.push(stage.outcome);
                stage_records.push(stage);
            }
        }
        Ok(LocalityExecutionSettlement {
            evaluated_outputs: program.evaluated_outputs(),
            stage_outcomes,
            stage_records,
        })
    }

    pub(super) fn apply_mutations(
        &mut self,
        mutations: &[FinancialLocalityMutation],
    ) -> Result<(), SignalError> {
        self.runtime.transaction(&mut (), |tx| {
            let mut batch = tx.batch_changes();
            for mutation in mutations {
                let source = self.handles[&mutation.producer];
                let aspect = signal_aspect(mutation.aspect);
                batch = match mutation.scope.map(|scope| {
                    let mut region = ChangedRegion::new(scope.partition_label());
                    if let Some(detail) = scope.detail_label() {
                        region = region.with_detail(detail);
                    }
                    region
                }) {
                    None => batch.mark(source, aspect),
                    Some(region) => {
                        batch.mark_regions(source, aspect, std::slice::from_ref(&region))
                    }
                };
            }
            batch.apply().map(|_| ())
        })?;
        Ok(())
    }
}
