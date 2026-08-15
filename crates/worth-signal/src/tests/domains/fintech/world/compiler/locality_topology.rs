use std::collections::BTreeMap;

use crate::data::aspect::AspectMask;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::EvaluationCondition;
use crate::data::output::PartitionSubscription;
use crate::data::output_equivalence::OutputEquivalencePolicy;

use super::super::{
    FinancialLocalityAdmissionPolicy, FinancialLocalityComparisonPolicy,
    FinancialLocalityDefinition, FinancialLocalityOutputPolicy, LocalitySemanticOutputId,
};
use super::topology::signal_aspect;

pub(super) fn build_locality_topology(
    graph: &mut SignalGraph,
    definition: &FinancialLocalityDefinition,
) -> Result<BTreeMap<LocalitySemanticOutputId, NodeId>, SignalError> {
    let mut handles = BTreeMap::new();
    for output in definition.outputs() {
        let reads = output
            .subscriptions
            .iter()
            .fold(AspectMask::EMPTY, |mask, subscription| {
                mask | AspectMask::from_aspect(signal_aspect(subscription.input_aspect))
            });
        let produces = output
            .produced_aspects()
            .iter()
            .fold(AspectMask::EMPTY, |mask, aspect| {
                mask | AspectMask::from_aspect(signal_aspect(*aspect))
            });
        let policy = output.execution_policy();
        let mut builder = graph
            .node()
            .reads_aspects(reads)
            .produces_aspects(produces)
            .dependency_comparator(match policy.dependency_comparison {
                FinancialLocalityComparisonPolicy::ExactEconomicRevision => {
                    VersionComparatorPolicy::Exact
                }
            })
            .output_equivalence(match policy.output_equivalence {
                FinancialLocalityOutputPolicy::ExactEconomicRevision => {
                    OutputEquivalencePolicy::ExactAspectVersion
                }
            });
        if let FinancialLocalityAdmissionPolicy::ChangedSubscribedAspect(aspects) = policy.admission
        {
            let trigger_mask = aspects.into_iter().fold(AspectMask::EMPTY, |mask, aspect| {
                mask | AspectMask::from_aspect(signal_aspect(aspect))
            });
            builder = builder.condition(EvaluationCondition::AspectFilter(trigger_mask));
        }
        if let Some(scope) = output
            .subscriptions
            .iter()
            .find_map(|subscription| subscription.eligibility_scope)
        {
            builder = builder.with_partition_scope(partition_subscription(scope));
        }
        handles.insert(output.id, builder.build());
    }
    install_locality_dependencies(graph, definition, &handles)?;
    Ok(handles)
}

pub(super) fn partition_subscription(scope: super::super::LocalityScope) -> PartitionSubscription {
    PartitionSubscription::partition_and_detail(
        scope.partition_label(),
        scope
            .detail_label()
            .expect("detail-scoped locality contract has detail identity"),
    )
}

fn install_locality_dependencies(
    graph: &mut SignalGraph,
    definition: &FinancialLocalityDefinition,
    handles: &BTreeMap<LocalitySemanticOutputId, NodeId>,
) -> Result<(), SignalError> {
    let reconciliations = definition
        .outputs()
        .iter()
        .map(|output| {
            let node = handles[&output.id];
            let dependencies =
                CanonicalDependencies::new(output.subscriptions.iter().map(|subscription| {
                    let source = handles[&subscription.upstream];
                    match subscription.edge_scope {
                        None => {
                            DependencyEdge::new(source, signal_aspect(subscription.input_aspect))
                        }
                        Some(scope) => DependencyEdge::partition_detail(
                            source,
                            signal_aspect(subscription.input_aspect),
                            scope.partition_label(),
                            scope
                                .detail_label()
                                .expect("detail-scoped locality edge has detail identity"),
                        ),
                    }
                }));
            (node, dependencies)
        })
        .collect::<Vec<_>>();
    graph.reconcile_dependencies_batch(&reconciliations)?;
    Ok(())
}
