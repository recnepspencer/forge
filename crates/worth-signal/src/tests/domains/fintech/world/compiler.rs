mod comparator_execution;
mod compiled_authority;
mod dependency_rewire;
mod evaluation;
mod factor_sequence;
mod inspection;
mod lifecycle_composition;
mod locality_evaluation;
mod locality_execution;
mod locality_topology;
mod quote_translation;
mod runtime_finance;
pub(super) mod topology;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::{ChangedRegion, NodeEvaluationResult};
use crate::facade::{NodeState, RuntimeMetrics, SignalGraph, SignalRuntime, SignalRuntimePolicy};

use super::baseline::{seal_financial_baseline, CausallyCompleteFinancialBaseline};
use super::{
    FinancialConsumerRole, FinancialEconomicSnapshot, FinancialSemanticProjection,
    FinancialWorldDefinition, InstrumentId, MarketFactorKey, SemanticOutputKey,
};
use crate::tests::domains::fintech::execution_tier::FintechTier;

use self::evaluation::FinancialEvaluationProgram;
use self::runtime_finance::runtime_financial_snapshot;
use self::topology::{build_semantic_topology, factor_signal_aspect};

type FinancialRuntime = SignalRuntime<(), (), (), (), FintechTier>;

pub(in crate::tests::domains::fintech) use compiled_authority::CompiledFinancialWorld;
use compiled_authority::{CompiledFinancialWorldKind, CompiledPortfolioFinancialWorld};
pub(in crate::tests::domains::fintech) use dependency_rewire::FinancialDependencyRewireEvidence;
pub(in crate::tests::domains::fintech) use factor_sequence::FinancialFactorSequenceEvidence;
pub(in crate::tests::domains::fintech) use lifecycle_composition::FinancialBranchLifecycleCompletion;
pub(in crate::tests::domains::fintech) use locality_execution::{
    compile_financial_locality_world, FinancialLocalityRedObservation,
};
pub(in crate::tests::domains::fintech) use quote_translation::FinancialQuoteTranslationEvidence;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FactorSourceHandle(
    pub(in crate::tests::domains::fintech) NodeId,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct PositionSemanticHandles {
    pub(in crate::tests::domains::fintech) valuation: NodeId,
    pub(in crate::tests::domains::fintech) risk: NodeId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct ConsumerSemanticHandle(
    pub(in crate::tests::domains::fintech) NodeId,
);

#[derive(Clone, Debug)]
pub(in crate::tests::domains::fintech) struct FinancialSemanticHandles {
    pub(super) factors: BTreeMap<MarketFactorKey, FactorSourceHandle>,
    pub(super) positions: BTreeMap<InstrumentId, PositionSemanticHandles>,
    pub(super) consumers: BTreeMap<FinancialConsumerRole, ConsumerSemanticHandle>,
}

impl FinancialSemanticHandles {
    pub(in crate::tests::domains::fintech) fn factor(
        &self,
        factor: MarketFactorKey,
    ) -> FactorSourceHandle {
        self.factors[&factor]
    }

    pub(in crate::tests::domains::fintech) fn position(
        &self,
        instrument: InstrumentId,
    ) -> PositionSemanticHandles {
        self.positions[&instrument]
    }

    pub(in crate::tests::domains::fintech) fn consumer(
        &self,
        role: FinancialConsumerRole,
    ) -> ConsumerSemanticHandle {
        self.consumers[&role]
    }

    pub(in crate::tests::domains::fintech) fn node_for(&self, key: SemanticOutputKey) -> NodeId {
        match key {
            SemanticOutputKey::Factor(factor) => self.factor(factor).0,
            SemanticOutputKey::Valuation(instrument) => self.position(instrument).valuation,
            SemanticOutputKey::Risk(instrument) => self.position(instrument).risk,
            SemanticOutputKey::Consumer(role) => self.consumer(role).0,
        }
    }

    pub(super) fn derived_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.positions
            .values()
            .flat_map(|handles| [handles.valuation, handles.risk])
            .chain(self.consumers.values().map(|handle| handle.0))
    }
}

#[derive(Clone, Default)]
pub(in crate::tests::domains::fintech) struct FinancialEvaluationLedger {
    counts: Arc<Mutex<BTreeMap<SemanticOutputKey, u64>>>,
}

impl FinancialEvaluationLedger {
    pub(super) fn record(&self, key: SemanticOutputKey) {
        let mut counts = self.counts.lock().expect("financial ledger lock poisoned");
        *counts.entry(key).or_default() += 1;
    }

    pub(super) fn clear(&self) {
        self.counts
            .lock()
            .expect("financial ledger lock poisoned")
            .clear();
    }

    pub(in crate::tests::domains::fintech) fn observed_work(&self) -> BTreeSet<SemanticOutputKey> {
        self.counts
            .lock()
            .expect("financial ledger lock poisoned")
            .keys()
            .copied()
            .collect()
    }

    pub(in crate::tests::domains::fintech) fn count(&self, key: SemanticOutputKey) -> u64 {
        self.counts
            .lock()
            .expect("financial ledger lock poisoned")
            .get(&key)
            .copied()
            .unwrap_or_default()
    }
}

pub(in crate::tests::domains::fintech) fn compile_financial_world(
    definition: FinancialWorldDefinition,
) -> Result<CausallyCompleteFinancialBaseline, SignalError> {
    if definition.locality().is_some() {
        return Err(SignalError::invalid_input(
            "locality courtrooms require compile_financial_locality_world",
        ));
    }
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .with_tiers::<FintechTier>()
        .build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::fintech()
            .with_history_limit(8)
            .with_detail_limit(4),
    );
    let handles = build_semantic_topology(&mut runtime.graph_mut(), &definition)?;
    let economic_snapshot = runtime_financial_snapshot(&definition);
    let projection = FinancialSemanticProjection::initial(&economic_snapshot);
    let ledger = FinancialEvaluationLedger::default();
    let baseline_dependency_revisions = projection
        .iter()
        .map(|(key, _)| {
            Ok((
                key,
                runtime
                    .graph()
                    .dependency_revision(handles.node_for(key))?
                    .0,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, SignalError>>()?;
    let portfolio = CompiledPortfolioFinancialWorld {
        runtime,
        definition,
        economic_snapshot,
        projection,
        handles,
        ledger,
        baseline_dependency_revisions,
        baseline_aspect_versions: BTreeMap::new(),
    };
    let mut compiled = CompiledFinancialWorld {
        kind: CompiledFinancialWorldKind::Portfolio(portfolio),
    };
    compiled.establish_initial_truth()?;
    compiled.baseline_aspect_versions = compiled
        .projection
        .iter()
        .map(|(key, _)| Ok((key, compiled.node_version(key)?)))
        .collect::<Result<BTreeMap<_, _>, SignalError>>()?;
    seal_financial_baseline(compiled)
}

impl CompiledFinancialWorld {
    fn program(&self) -> FinancialEvaluationProgram {
        FinancialEvaluationProgram::new(
            self.definition.clone(),
            self.projection.clone(),
            self.handles.clone(),
            self.ledger.clone(),
        )
    }

    fn establish_initial_truth(&mut self) -> Result<(), SignalError> {
        let program = self.program();
        let evaluator = program.evaluator();
        let factor_nodes = self
            .handles
            .factors
            .iter()
            .map(|(factor, handle)| (*factor, handle.0))
            .collect::<Vec<_>>();
        let risk_nodes = self
            .definition
            .positions()
            .iter()
            .map(|position| self.handles.position(position.instrument).risk)
            .collect::<Vec<_>>();
        let consumer_nodes = self
            .definition
            .consumers()
            .iter()
            .map(|consumer| self.handles.consumer(consumer.role).0)
            .collect::<Vec<_>>();
        self.runtime.transaction(&mut (), |tx| {
            for (factor, node) in &factor_nodes {
                let result = source_result(&program, *factor);
                tx.target(*node)
                    .on_demand()
                    .read(&move |view| Ok(view.finish(result.clone())))?;
            }
            for node in &risk_nodes {
                tx.read(*node, &evaluator)?;
            }
            for node in &consumer_nodes {
                tx.read(*node, &evaluator)?;
            }
            Ok(())
        })?;
        self.ledger.clear();
        Ok(())
    }

    pub(in crate::tests::domains::fintech) fn apply_factor_change(
        &mut self,
        next_definition: FinancialWorldDefinition,
        factor: MarketFactorKey,
    ) -> Result<(), SignalError> {
        let next_snapshot = runtime_financial_snapshot(&next_definition);
        let next_projection = self.projection.advance(&next_snapshot);
        let program = FinancialEvaluationProgram::new(
            next_definition.clone(),
            next_projection.clone(),
            self.handles.clone(),
            self.ledger.clone(),
        );
        let evaluator = program.evaluator();
        let source = self.handles.factor(factor).0;
        let matched = self.handles.consumer(FinancialConsumerRole::RiskMatched).0;
        let unmatched = self
            .handles
            .consumer(FinancialConsumerRole::RiskUnmatched)
            .0;
        let source_result = source_result(&program, factor);
        let ledger = self.ledger.clone();
        self.ledger.clear();
        self.runtime.transaction(&mut (), |tx| {
            tx.mark_changed(source, factor_signal_aspect(&next_definition, factor))?;
            ledger.record(SemanticOutputKey::Factor(factor));
            tx.target(source)
                .on_demand()
                .read(&move |view| Ok(view.finish(source_result.clone())))?;
            tx.read(matched, &evaluator)?;
            tx.read(unmatched, &evaluator)?;
            Ok(())
        })?;
        self.definition = next_definition;
        self.economic_snapshot = next_snapshot;
        self.projection = next_projection;
        Ok(())
    }

    pub(in crate::tests::domains::fintech) fn definition(&self) -> &FinancialWorldDefinition {
        match &self.kind {
            CompiledFinancialWorldKind::Portfolio(portfolio) => &portfolio.definition,
            CompiledFinancialWorldKind::Locality(locality) => locality.definition(),
        }
    }

    pub(in crate::tests::domains::fintech) fn economic_snapshot(
        &self,
    ) -> &FinancialEconomicSnapshot {
        &self.economic_snapshot
    }

    pub(in crate::tests::domains::fintech) fn projection(&self) -> &FinancialSemanticProjection {
        &self.projection
    }

    pub(in crate::tests::domains::fintech) fn handles(&self) -> &FinancialSemanticHandles {
        &self.handles
    }

    pub(in crate::tests::domains::fintech) fn ledger(&self) -> &FinancialEvaluationLedger {
        &self.ledger
    }

    pub(in crate::tests::domains::fintech) fn graph(&self) -> &SignalGraph {
        self.runtime.graph()
    }

    pub(super) fn runtime_mut(&mut self) -> &mut FinancialRuntime {
        &mut self.runtime
    }

    pub(in crate::tests::domains::fintech) fn metrics(&self) -> RuntimeMetrics {
        self.runtime.observe().metrics()
    }

    pub(in crate::tests::domains::fintech) fn node_version(
        &self,
        key: SemanticOutputKey,
    ) -> Result<AspectVersion, SignalError> {
        self.runtime
            .graph()
            .node_aspect_version(self.handles.node_for(key))
    }

    pub(in crate::tests::domains::fintech) fn node_state(
        &self,
        key: SemanticOutputKey,
    ) -> Result<NodeState, SignalError> {
        self.runtime.graph().get_state(self.handles.node_for(key))
    }

    pub(in crate::tests::domains::fintech) fn factor_slot(
        &self,
        factor: MarketFactorKey,
    ) -> crate::data::aspect::Aspect {
        factor_signal_aspect(&self.definition, factor)
    }
}

fn source_result(
    program: &FinancialEvaluationProgram,
    factor: MarketFactorKey,
) -> NodeEvaluationResult {
    let (partition, detail) = factor.partition();
    program
        .result_for(SemanticOutputKey::Factor(factor))
        .with_changed_region(ChangedRegion::new(partition).with_detail(detail))
}
