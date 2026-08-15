use std::collections::BTreeMap;

mod actions;
mod baseline;
mod compilation;
#[cfg(test)]
mod lifecycle_tests;
mod performed_work;
#[cfg(test)]
mod receipt_tests;

pub(in crate::tests::domains::fintech) use actions::FinancialRestoreLifecycleEvidence;
pub(in crate::tests::domains::fintech) use compilation::{
    compile_financial_locality_world, compile_financial_locality_world_at_tier,
};
pub(in crate::tests::domains::fintech) use performed_work::{
    strategy_work_projection, FinancialPerformedCanonicalWork, FinancialPerformedWorkOrigin,
};

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::facade::SignalRuntime;
use crate::logic::context::EvaluationContext;
use crate::tests::domains::fintech::execution_tier::FintechTier;

use super::super::{
    FinancialLocalityDefinition, FinancialWorldDefinition, LocalitySemanticOutputId,
};
use super::locality_evaluation::LocalityEvaluationProgram;
use super::topology::signal_aspect;

type LocalityRuntime = SignalRuntime<(), (), (), (), FintechTier>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityRedObservation {
    pub(in crate::tests::domains::fintech) performed_counters:
        crate::data::telemetry::SignalInvalidationRealizedCounters,
    pub(in crate::tests::domains::fintech) direct_candidates_examined: u64,
    pub(in crate::tests::domains::fintech) reverse_candidates_returned: u64,
    pub(in crate::tests::domains::fintech) reverse_bucket_probes: u64,
    pub(in crate::tests::domains::fintech) contract_rejections: u64,
    pub(in crate::tests::domains::fintech) causality_rejections: u64,
    pub(in crate::tests::domains::fintech) nodes_visited: u64,
    pub(in crate::tests::domains::fintech) transitive_frontier_width: u64,
    pub(in crate::tests::domains::fintech) comparator_suppressed_count: u64,
    pub(in crate::tests::domains::fintech) work_items_admitted: u64,
    pub(in crate::tests::domains::fintech) work_items_merged: u64,
    pub(in crate::tests::domains::fintech) ready_items_enqueued: u64,
    pub(in crate::tests::domains::fintech) ready_items_popped: u64,
    pub(in crate::tests::domains::fintech) peak_ready_width: u64,
    pub(in crate::tests::domains::fintech) retained_ready_width: u64,
    pub(in crate::tests::domains::fintech) evaluated_outputs:
        std::collections::BTreeSet<LocalitySemanticOutputId>,
    pub(in crate::tests::domains::fintech) baseline_retained_outputs:
        std::collections::BTreeSet<LocalitySemanticOutputId>,
    pub(in crate::tests::domains::fintech) performed_work: FinancialPerformedCanonicalWork,
}

struct RedObservationInput {
    before: crate::data::telemetry::InvalidationTelemetry,
    after: crate::data::telemetry::InvalidationTelemetry,
    evaluation_before: crate::data::telemetry::EvaluationTelemetry,
    evaluation_after: crate::data::telemetry::EvaluationTelemetry,
    evaluated_outputs: std::collections::BTreeSet<LocalitySemanticOutputId>,
    baseline_retained_outputs: std::collections::BTreeSet<LocalitySemanticOutputId>,
    performed: crate::data::telemetry::SignalInvalidationRealizedCounters,
}

pub(super) struct CompiledFinancialLocalityWorld {
    runtime: LocalityRuntime,
    definition: FinancialWorldDefinition,
    handles: BTreeMap<LocalitySemanticOutputId, NodeId>,
    baseline_values: BTreeMap<LocalitySemanticOutputId, i64>,
}

impl CompiledFinancialLocalityWorld {
    fn graph_instance(&self) -> u64 {
        self.runtime.graph().runtime_instance_id()
    }

    fn establish_causally_complete_baseline(&mut self) -> Result<(), SignalError> {
        let program = LocalityEvaluationProgram::baseline(
            self.locality_definition(),
            &self.handles,
            &self.baseline_values,
        );
        let evaluator = |view: &mut EvaluationContext<'_, ()>| program.evaluate(view);
        let nodes = self
            .locality_definition()
            .outputs()
            .iter()
            .map(|output| self.handles[&output.id])
            .collect::<Vec<_>>();
        self.runtime.read_many(&nodes, &(), &evaluator).map(|_| ())
    }

    pub(in crate::tests::domains::fintech) fn run_inherited_breadth_red_control(
        &mut self,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.runtime
            .graph_mut()
            .reset_invalidation_performed_counters();
        let before = self.runtime.graph().telemetry().invalidation;
        let evaluation_before = self.runtime.graph().telemetry().evaluation;
        self.apply_declared_mutation()?;
        let evaluated_outputs = self.settle_declared_mutation()?;
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

    fn settle_declared_mutation(
        &mut self,
    ) -> Result<std::collections::BTreeSet<LocalitySemanticOutputId>, SignalError> {
        self.settle_mutations(&[self.locality_definition().mutation()])
    }

    fn apply_declared_mutation(&mut self) -> Result<(), SignalError> {
        self.apply_mutations(&[self.locality_definition().mutation()])
    }

    fn red_observation(&self, input: RedObservationInput) -> FinancialLocalityRedObservation {
        let RedObservationInput {
            before,
            after,
            evaluation_before,
            evaluation_after,
            evaluated_outputs,
            baseline_retained_outputs,
            performed,
        } = input;
        FinancialLocalityRedObservation {
            performed_counters: performed,
            direct_candidates_examined: delta(
                before.direct_subscriber_candidates_examined,
                after.direct_subscriber_candidates_examined,
            ),
            reverse_candidates_returned: delta(
                before.reverse_subscription_candidates_returned,
                after.reverse_subscription_candidates_returned,
            ),
            reverse_bucket_probes: delta(
                before.reverse_subscription_bucket_probes,
                after.reverse_subscription_bucket_probes,
            ),
            contract_rejections: delta(
                before.direct_contract_rejections,
                after.direct_contract_rejections,
            ),
            causality_rejections: delta(
                before.direct_causality_rejections,
                after.direct_causality_rejections,
            ),
            nodes_visited: delta(
                before.invalidation_nodes_visited,
                after.invalidation_nodes_visited,
            ),
            transitive_frontier_width: delta(
                before.transitive_frontier_width,
                after.transitive_frontier_width,
            ),
            comparator_suppressed_count: delta(
                evaluation_before.skipped_by_comparator,
                evaluation_after.skipped_by_comparator,
            ),
            work_items_admitted: performed.work_items_admitted(),
            work_items_merged: performed.work_items_merged(),
            ready_items_enqueued: performed.ready_items_enqueued(),
            ready_items_popped: performed.ready_items_popped(),
            peak_ready_width: performed.maximum_ready_frontier_width(),
            retained_ready_width: performed.retained_ready_frontier_width(),
            evaluated_outputs,
            baseline_retained_outputs,
            performed_work: self.performed_canonical_work(),
        }
    }

    fn baseline_retained_outputs(
        &self,
        evaluated: &std::collections::BTreeSet<LocalitySemanticOutputId>,
    ) -> Result<std::collections::BTreeSet<LocalitySemanticOutputId>, SignalError> {
        evaluated
            .iter()
            .filter_map(|output| {
                let declaration = &self.locality_definition().outputs()[output.ordinal() as usize];
                let node = self.handles[output];
                let retained = declaration.produced_aspects().iter().all(|aspect| {
                    self.runtime
                        .graph()
                        .node_version_for_scope(node, signal_aspect(*aspect), None)
                        .is_ok_and(|version| {
                            version
                                == self
                                    .locality_definition()
                                    .workload()
                                    .baseline_aspect_version()
                        })
                });
                retained.then_some(Ok(*output))
            })
            .collect()
    }

    pub(super) fn definition(&self) -> &FinancialWorldDefinition {
        &self.definition
    }

    fn locality_definition(&self) -> &FinancialLocalityDefinition {
        self.definition
            .locality()
            .expect("compiled locality backend retains locality definition")
    }

    fn committed_financial_values(
        &self,
    ) -> Result<BTreeMap<LocalitySemanticOutputId, i64>, SignalError> {
        self.handles
            .iter()
            .map(|(output, node)| self.committed_value(*output, *node))
            .collect()
    }

    fn committed_value(
        &self,
        output: LocalitySemanticOutputId,
        node: NodeId,
    ) -> Result<(LocalitySemanticOutputId, i64), SignalError> {
        let identity = self
            .runtime
            .graph()
            .node_runtime_artifact_warm(node)?
            .and_then(|warm| warm.output_identity.as_ref())
            .ok_or_else(|| {
                SignalError::internal(format!(
                    "locality output {output:?} lacks a committed artifact identity"
                ))
            })?;
        let (_, value) = identity.as_str().rsplit_once(':').ok_or_else(|| {
            SignalError::internal(format!(
                "locality output {output:?} has malformed financial identity"
            ))
        })?;
        let value = value.parse::<i64>().map_err(|_| {
            SignalError::internal(format!(
                "locality output {output:?} has non-numeric financial identity"
            ))
        })?;
        Ok((output, value))
    }
}

impl super::CompiledFinancialWorld {
    pub(super) fn from_locality(locality: CompiledFinancialLocalityWorld) -> Self {
        Self {
            kind: super::CompiledFinancialWorldKind::Locality(locality),
        }
    }

    fn locality(&self) -> &CompiledFinancialLocalityWorld {
        match &self.kind {
            super::CompiledFinancialWorldKind::Locality(locality) => locality,
            super::CompiledFinancialWorldKind::Portfolio(_) => {
                panic!("locality operation used with compiled portfolio world")
            }
        }
    }

    fn locality_mut(&mut self) -> &mut CompiledFinancialLocalityWorld {
        match &mut self.kind {
            super::CompiledFinancialWorldKind::Locality(locality) => locality,
            super::CompiledFinancialWorldKind::Portfolio(_) => {
                panic!("locality mutation used with compiled portfolio world")
            }
        }
    }

    pub(in crate::tests::domains::fintech) fn locality_definition(
        &self,
    ) -> &FinancialLocalityDefinition {
        self.locality().locality_definition()
    }

    pub(in crate::tests::domains::fintech) fn locality_graph_instance(&self) -> u64 {
        self.locality().graph_instance()
    }

    pub(in crate::tests::domains::fintech) fn run_inherited_breadth_red_control(
        &mut self,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.locality_mut().run_inherited_breadth_red_control()
    }

    pub(in crate::tests::domains::fintech) fn run_locality_action_trace(
        &mut self,
        trace_index: usize,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.locality_mut().run_action_trace(trace_index)
    }

    pub(in crate::tests::domains::fintech) fn run_locality_action_trace_with_executor(
        &mut self,
        trace_index: usize,
        executor: crate::logic::planner::StageExecutor,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.locality_mut()
            .run_action_trace_with_executor(trace_index, executor)
    }

    pub(in crate::tests::domains::fintech) fn observe_locality_action_trace_with_executor(
        &mut self,
        trace_index: usize,
        executor: crate::logic::planner::StageExecutor,
    ) -> Result<
        (
            FinancialLocalityRedObservation,
            crate::data::proof::SignalInvalidationExecutionReceipt,
        ),
        SignalError,
    > {
        let token = self
            .locality_mut()
            .runtime
            .begin_invalidation_execution_observation();
        let observation = self.run_locality_action_trace_with_executor(trace_index, executor)?;
        let receipt = self
            .locality()
            .runtime
            .finish_invalidation_execution_observation(token)?;
        Ok((observation, receipt))
    }

    pub(in crate::tests::domains::fintech) fn set_locality_diagnostics_tier(
        &mut self,
        tier: crate::facade::DiagnosticsTier,
    ) {
        self.locality_mut()
            .runtime
            .graph_mut()
            .reset_runtime_policy_to_tier(tier);
    }

    pub(in crate::tests::domains::fintech) fn locality_retained_fact_counts(
        &self,
    ) -> (usize, usize) {
        let observer = self.locality().runtime.graph().observe();
        self.locality()
            .handles
            .values()
            .fold((0, 0), |(explanations, provenance), node| {
                (
                    explanations + usize::from(observer.explanation_fact(*node).is_some()),
                    provenance + usize::from(observer.provenance_fact(*node).is_some()),
                )
            })
    }

    pub(in crate::tests::domains::fintech) fn certify_restore_locality_lifecycle(
        &mut self,
    ) -> Result<FinancialRestoreLifecycleEvidence, SignalError> {
        self.locality_mut().certify_restore_lifecycle()
    }

    pub(in crate::tests::domains::fintech) fn committed_locality_financial_values(
        &self,
    ) -> Result<BTreeMap<LocalitySemanticOutputId, i64>, SignalError> {
        self.locality().committed_financial_values()
    }
}

fn delta(before: u64, after: u64) -> u64 {
    after
        .checked_sub(before)
        .expect("locality telemetry is monotonic")
}
