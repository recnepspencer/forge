use std::collections::BTreeMap;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::facade::{mark_dirty, mark_dirty_with_regions, NodeState, SignalRuntime};
use crate::logic::context::EvaluationContext;
use crate::tests::domains::fintech::execution_tier::FintechTier;

use super::super::{
    FinancialLocalityDefinition, FinancialWorldDefinition, LocalitySemanticOutputId,
};
use super::locality_evaluation::{
    runtime_baseline_values, runtime_shocked_values, LocalityEvaluationProgram,
};
use super::locality_topology::{build_locality_topology, partition_subscription};
use super::topology::signal_aspect;

type LocalityRuntime = SignalRuntime<(), (), (), (), FintechTier>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityRedObservation {
    pub(in crate::tests::domains::fintech) direct_candidates_examined: u64,
    pub(in crate::tests::domains::fintech) contract_rejections: u64,
    pub(in crate::tests::domains::fintech) causality_rejections: u64,
    pub(in crate::tests::domains::fintech) nodes_visited: u64,
    pub(in crate::tests::domains::fintech) transitive_frontier_width: u64,
    pub(in crate::tests::domains::fintech) independent_necessary_evaluations: u64,
    pub(in crate::tests::domains::fintech) unchanged_output_stops: u64,
    pub(in crate::tests::domains::fintech) evaluated_outputs:
        std::collections::BTreeSet<LocalitySemanticOutputId>,
}

pub(super) struct CompiledFinancialLocalityWorld {
    runtime: LocalityRuntime,
    definition: FinancialWorldDefinition,
    handles: BTreeMap<LocalitySemanticOutputId, NodeId>,
    baseline_values: BTreeMap<LocalitySemanticOutputId, i64>,
}

pub(in crate::tests::domains::fintech) fn compile_financial_locality_world(
    definition: FinancialWorldDefinition,
) -> Result<super::CompiledFinancialWorld, SignalError> {
    let locality = definition.locality().cloned().ok_or_else(|| {
        SignalError::invalid_input("financial locality compiler requires a locality courtroom")
    })?;
    locality.validate_generator_invariants();
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .with_tiers::<FintechTier>()
        .build();
    let handles = build_locality_topology(&mut runtime.graph_mut(), &locality)?;
    let baseline_values = runtime_baseline_values(&locality)?;
    let mut compiled = CompiledFinancialLocalityWorld {
        runtime,
        definition,
        handles,
        baseline_values,
    };
    compiled.establish_causally_complete_baseline()?;
    compiled.seal_baseline()?;
    Ok(super::CompiledFinancialWorld::from_locality(compiled))
}

impl CompiledFinancialLocalityWorld {
    fn establish_causally_complete_baseline(&mut self) -> Result<(), SignalError> {
        let program = LocalityEvaluationProgram::new(
            self.locality_definition(),
            &self.handles,
            &self.baseline_values,
            1,
        );
        let evaluator = |view: &mut EvaluationContext<'_, ()>| program.evaluate(view);
        let nodes = self
            .locality_definition()
            .outputs()
            .iter()
            .map(|output| self.handles[&output.id])
            .collect::<Vec<_>>();
        self.runtime
            .transaction(&mut (), |tx| {
                for node in &nodes {
                    tx.read(*node, &evaluator)?;
                }
                Ok(())
            })
            .map(|_| ())
    }

    fn seal_baseline(&self) -> Result<(), SignalError> {
        self.verify_topology_and_state()?;
        if self.committed_financial_values()? != self.baseline_values {
            return Err(SignalError::internal(
                "locality baseline committed artifacts disagree with compiled financial values",
            ));
        }
        Ok(())
    }

    fn verify_topology_and_state(&self) -> Result<(), SignalError> {
        if self.handles.len() != self.locality_definition().outputs().len() {
            return Err(SignalError::internal(
                "locality compiler lost a semantic output handle",
            ));
        }
        for output in self.locality_definition().outputs() {
            let node = self.handles[&output.id];
            self.verify_output_baseline(output, node)?;
        }
        Ok(())
    }

    fn verify_output_baseline(
        &self,
        output: &super::super::FinancialLocalityOutput,
        node: NodeId,
    ) -> Result<(), SignalError> {
        if self.runtime.graph().get_state(node)? != NodeState::Clean {
            return Err(SignalError::internal(format!(
                "locality baseline output {:?} is not clean",
                output.id
            )));
        }
        let actual_edges = self.runtime.graph().dependencies_of(node)?;
        let actual_snapshot = self.runtime.graph().get_dep_snapshot(node)?;
        if actual_edges.len() != output.dependencies.len()
            || actual_snapshot.entries().len() != output.dependencies.len()
        {
            return Err(self.baseline_authority_error(output.id));
        }
        for ((edge, snapshot), declared) in actual_edges
            .iter()
            .zip(actual_snapshot.entries())
            .zip(&output.dependencies)
        {
            let source = self.handles[&declared.producer];
            let aspect = signal_aspect(declared.aspect);
            let expected_scope = declared.edge_scope.map(partition_subscription);
            let current_version = self.runtime.graph().node_version_for_scope(
                source,
                aspect,
                expected_scope.as_ref(),
            )?;
            if edge.source() != source
                || edge.aspect() != aspect
                || edge.scope_ref() != expected_scope.as_ref()
                || snapshot.source != source
                || snapshot.aspect != aspect
                || snapshot.scope != expected_scope
                || snapshot.cached_version != current_version
            {
                return Err(self.baseline_authority_error(output.id));
            }
        }
        let expected_contract_scope = output
            .dependencies
            .iter()
            .find_map(|dependency| dependency.contract_scope)
            .map(|scope| vec![partition_subscription(scope)]);
        if self
            .runtime
            .graph()
            .node_eval_config(node)?
            .contract
            .semantics
            .partition_scope
            != expected_contract_scope
        {
            return Err(self.baseline_authority_error(output.id));
        }
        Ok(())
    }

    fn baseline_authority_error(&self, output: LocalitySemanticOutputId) -> SignalError {
        SignalError::internal(format!(
            "locality baseline output {output:?} changed edge, contract, or snapshot authority"
        ))
    }

    pub(in crate::tests::domains::fintech) fn run_inherited_breadth_red_control(
        &mut self,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        let before = self.runtime.graph().telemetry().invalidation;
        self.apply_declared_mutation()?;
        let after = self.runtime.graph().telemetry().invalidation;
        let evaluated_outputs = self.settle_declared_mutation()?;
        Ok(self.red_observation(before, after, evaluated_outputs))
    }

    fn settle_declared_mutation(
        &mut self,
    ) -> Result<std::collections::BTreeSet<LocalitySemanticOutputId>, SignalError> {
        let shocked_values =
            runtime_shocked_values(self.locality_definition(), &self.baseline_values)?;
        let program = LocalityEvaluationProgram::new(
            self.locality_definition(),
            &self.handles,
            &shocked_values,
            2,
        );
        let evaluator = |view: &mut EvaluationContext<'_, ()>| program.evaluate(view);
        let source = self.handles[&self.locality_definition().mutation().producer];
        let nodes = self.settlement_targets();
        self.runtime
            .transaction(&mut (), |tx| tx.read(source, &evaluator).map(|_| ()))?;
        self.runtime.transaction(&mut (), |tx| {
            for node in &nodes {
                tx.read(*node, &evaluator)?;
            }
            Ok(())
        })?;
        Ok(program.evaluated_outputs())
    }

    fn settlement_targets(&self) -> Vec<NodeId> {
        self.locality_definition()
            .outputs()
            .iter()
            .filter(|candidate| {
                candidate.expected_for_mutation
                    && !self.locality_definition().outputs().iter().any(|consumer| {
                        consumer.expected_for_mutation
                            && consumer
                                .dependencies
                                .iter()
                                .any(|dependency| dependency.producer == candidate.id)
                    })
            })
            .map(|output| self.handles[&output.id])
            .collect()
    }

    fn apply_declared_mutation(&mut self) -> Result<(), SignalError> {
        let mutation = self.locality_definition().mutation();
        let source = self.handles[&mutation.producer];
        match mutation.scope {
            None => mark_dirty(
                self.runtime.graph_mut(),
                source,
                signal_aspect(mutation.aspect),
            ),
            Some(scope) => {
                let mut region = ChangedRegion::new(scope.partition_label());
                if let Some(detail) = scope.detail_label() {
                    region = region.with_detail(detail);
                }
                mark_dirty_with_regions(
                    self.runtime.graph_mut(),
                    source,
                    signal_aspect(mutation.aspect),
                    &[region],
                )
            }
        }
    }

    fn red_observation(
        &self,
        before: crate::data::telemetry::InvalidationTelemetry,
        after: crate::data::telemetry::InvalidationTelemetry,
        evaluated_outputs: std::collections::BTreeSet<LocalitySemanticOutputId>,
    ) -> FinancialLocalityRedObservation {
        FinancialLocalityRedObservation {
            direct_candidates_examined: delta(
                before.direct_subscriber_candidates_examined,
                after.direct_subscriber_candidates_examined,
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
            independent_necessary_evaluations: self
                .locality_definition()
                .outputs()
                .iter()
                .filter(|output| output.expected_for_mutation)
                .count() as u64,
            unchanged_output_stops: self
                .locality_definition()
                .outputs()
                .iter()
                .filter(|output| output.unchanged_output_stop)
                .count() as u64,
            evaluated_outputs,
        }
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

    pub(in crate::tests::domains::fintech) fn run_inherited_breadth_red_control(
        &mut self,
    ) -> Result<FinancialLocalityRedObservation, SignalError> {
        self.locality_mut().run_inherited_breadth_red_control()
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
