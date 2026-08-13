use crate::data::comparator::{
    ComparatorPolicyResolver, InstalledSignalComparatorIdentity, VersionComparatorPolicy,
    VersionComparatorResolver,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::facade::{mark_dirty, DefaultConditionResolver, EvaluationRequestMode};
use crate::tests::support::evaluate_with_policy_and_condition_resolvers;

use super::super::{
    FinancialComparatorPolicy, FinancialWorldDefinition, MarketFactorKey, SemanticOutputKey,
};
use super::evaluation::FinancialEvaluationProgram;
use super::runtime_finance::runtime_financial_snapshot;
use super::topology::factor_signal_aspect;
use super::{source_result, CompiledFinancialWorld};

struct InstalledComparatorRule {
    identity: InstalledSignalComparatorIdentity,
    epsilon: u64,
}

struct FinancialComparatorResolver {
    installed: Vec<InstalledComparatorRule>,
}

impl FinancialComparatorResolver {
    fn from_world(world: &CompiledFinancialWorld) -> Result<Self, SignalError> {
        let mut installed = Vec::new();
        for declaration in world.definition.consumers() {
            let FinancialComparatorPolicy::InstalledTolerance { epsilon } = declaration.comparator
            else {
                continue;
            };
            let node = world.handles.consumer(declaration.role).0;
            let Some(VersionComparatorPolicy::Installed { identity }) = world
                .runtime
                .graph()
                .node_eval_config(node)?
                .comparator
                .as_ref()
            else {
                return Err(SignalError::internal(
                    "compiled installed financial comparator lost its runtime identity",
                ));
            };
            installed.push(InstalledComparatorRule {
                identity: identity.clone(),
                epsilon,
            });
        }
        Ok(Self { installed })
    }
}

impl VersionComparatorResolver for FinancialComparatorResolver {
    fn resolve(
        &mut self,
        key: &str,
        _aspect: crate::facade::Aspect,
        _cached: u64,
        _current: u64,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "financial courtroom has no portable custom comparator named {key}"
        )))
    }

    fn resolve_installed(
        &mut self,
        identity: &InstalledSignalComparatorIdentity,
        _aspect: crate::facade::Aspect,
        cached: u64,
        current: u64,
    ) -> Result<bool, SignalError> {
        let rule = self
            .installed
            .iter()
            .find(|rule| rule.identity.is_same_installed_identity(identity))
            .ok_or_else(|| SignalError::invalid_input("unknown installed financial comparator"))?;
        Ok(current.abs_diff(cached) > rule.epsilon)
    }
}

impl ComparatorPolicyResolver for FinancialComparatorResolver {
    fn policy_for_node(
        &self,
        _node: NodeId,
        node_override: Option<&VersionComparatorPolicy>,
    ) -> VersionComparatorPolicy {
        node_override.cloned().unwrap_or_default()
    }
}

impl CompiledFinancialWorld {
    pub(in crate::tests::domains::fintech) fn apply_factor_change_with_runtime_comparators(
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
        let source = self.handles.factor(factor).0;
        let consumers = next_definition
            .consumers()
            .iter()
            .map(|consumer| self.handles.consumer(consumer.role).0)
            .collect::<Vec<_>>();
        let source_candidate = source_result(&program, factor);
        let mut resolver = FinancialComparatorResolver::from_world(self)?;
        let mut condition = DefaultConditionResolver;
        self.ledger.clear();
        mark_dirty(
            self.runtime.graph_mut(),
            source,
            factor_signal_aspect(&next_definition, factor),
        )?;
        self.ledger.record(SemanticOutputKey::Factor(factor));
        evaluate_with_policy_and_condition_resolvers(
            self.runtime.graph_mut(),
            source,
            &mut |_node, _graph| Ok(source_candidate.clone()),
            &mut resolver,
            &mut condition,
            EvaluationRequestMode::Default,
        )?;
        for target in consumers {
            evaluate_with_policy_and_condition_resolvers(
                self.runtime.graph_mut(),
                target,
                &mut |node, _graph| program.result_for_node(node),
                &mut resolver,
                &mut condition,
                EvaluationRequestMode::Default,
            )?;
        }
        self.definition = next_definition;
        self.economic_snapshot = next_snapshot;
        self.projection = next_projection;
        Ok(())
    }

    pub(in crate::tests::domains::fintech) fn projected_revision(
        &self,
        key: SemanticOutputKey,
    ) -> u64 {
        self.projection.output(key).revision
    }
}
