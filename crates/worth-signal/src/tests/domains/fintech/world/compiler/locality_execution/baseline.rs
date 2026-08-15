use crate::data::comparator::VersionComparatorPolicy;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::EvaluationCondition;
use crate::data::output_equivalence::OutputEquivalencePolicy;
use crate::facade::NodeState;

use super::super::super::{
    FinancialLocalityAdmissionPolicy, FinancialLocalityComparisonPolicy, FinancialLocalityOutput,
    FinancialLocalityOutputPolicy, LocalitySemanticOutputId,
};
use super::super::locality_topology::partition_subscription;
use super::super::topology::signal_aspect;
use super::CompiledFinancialLocalityWorld;

impl CompiledFinancialLocalityWorld {
    pub(super) fn seal_baseline(&self) -> Result<(), SignalError> {
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
        output: &FinancialLocalityOutput,
        node: NodeId,
    ) -> Result<(), SignalError> {
        if self.runtime.graph().get_state(node)? != NodeState::Clean {
            return Err(SignalError::internal(format!(
                "locality baseline output {:?} is not clean",
                output.id
            )));
        }
        let expected_revision = u64::from(!output.subscriptions.is_empty());
        if self.runtime.graph().dependency_revision(node)?.0 != expected_revision {
            return Err(self.baseline_authority_error(output.id));
        }
        self.verify_dependency_baseline(output, node)?;
        self.verify_contract_baseline(output, node)
    }

    fn verify_dependency_baseline(
        &self,
        output: &FinancialLocalityOutput,
        node: NodeId,
    ) -> Result<(), SignalError> {
        let actual_edges = self.runtime.graph().dependencies_of(node)?;
        let actual_snapshot = self.runtime.graph().get_dep_snapshot(node)?;
        if actual_edges.len() != output.subscriptions.len()
            || actual_snapshot.entries().len() != output.subscriptions.len()
        {
            return Err(self.baseline_authority_error(output.id));
        }
        for ((edge, snapshot), declared) in actual_edges
            .iter()
            .zip(actual_snapshot.entries())
            .zip(&output.subscriptions)
        {
            let source = self.handles[&declared.upstream];
            let aspect = signal_aspect(declared.input_aspect);
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
        Ok(())
    }

    fn verify_contract_baseline(
        &self,
        output: &FinancialLocalityOutput,
        node: NodeId,
    ) -> Result<(), SignalError> {
        let expected_contract_scope = output
            .subscriptions
            .iter()
            .find_map(|subscription| subscription.eligibility_scope)
            .map(|scope| vec![partition_subscription(scope)]);
        let config = self.runtime.graph().node_eval_config(node)?;
        let policy = output.execution_policy();
        let expected_condition = match policy.admission {
            FinancialLocalityAdmissionPolicy::Always => EvaluationCondition::Always,
            FinancialLocalityAdmissionPolicy::ChangedSubscribedAspect(aspects) => {
                EvaluationCondition::AspectFilter(aspects.into_iter().fold(
                    crate::data::aspect::AspectMask::EMPTY,
                    |mask, aspect| {
                        mask | crate::data::aspect::AspectMask::from_aspect(signal_aspect(aspect))
                    },
                ))
            }
        };
        let expected_comparator = match policy.dependency_comparison {
            FinancialLocalityComparisonPolicy::ExactEconomicRevision => {
                Some(VersionComparatorPolicy::Exact)
            }
        };
        let expected_output_equivalence = match policy.output_equivalence {
            FinancialLocalityOutputPolicy::ExactEconomicRevision => {
                OutputEquivalencePolicy::ExactAspectVersion
            }
        };
        if config.contract.semantics.partition_scope != expected_contract_scope
            || config.condition != expected_condition
            || config.comparator != expected_comparator
            || config.output_equivalence != expected_output_equivalence
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
}
