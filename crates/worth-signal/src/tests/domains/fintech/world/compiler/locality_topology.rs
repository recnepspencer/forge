use std::collections::BTreeMap;

use crate::data::aspect::AspectMask;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;

use super::super::{FinancialLocalityDefinition, LocalitySemanticOutputId};
use super::topology::signal_aspect;

pub(super) fn build_locality_topology(
    graph: &mut SignalGraph,
    definition: &FinancialLocalityDefinition,
) -> Result<BTreeMap<LocalitySemanticOutputId, NodeId>, SignalError> {
    let mut handles = BTreeMap::new();
    for output in definition.outputs() {
        let reads = output
            .dependencies
            .iter()
            .fold(AspectMask::EMPTY, |mask, dependency| {
                mask | AspectMask::from_aspect(signal_aspect(dependency.aspect))
            });
        let produces = output
            .produced_aspects
            .iter()
            .fold(AspectMask::EMPTY, |mask, aspect| {
                mask | AspectMask::from_aspect(signal_aspect(*aspect))
            });
        let mut builder = graph.node().reads_aspects(reads).produces_aspects(produces);
        if let Some(scope) = output
            .dependencies
            .iter()
            .find_map(|dependency| dependency.contract_scope)
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
    for output in definition.outputs() {
        let node = handles[&output.id];
        graph.set_dependencies(
            node,
            output.dependencies.iter().map(|dependency| {
                let source = handles[&dependency.producer];
                match dependency.edge_scope {
                    None => DependencyEdge::new(source, signal_aspect(dependency.aspect)),
                    Some(scope) => DependencyEdge::partition_detail(
                        source,
                        signal_aspect(dependency.aspect),
                        scope.partition_label(),
                        scope
                            .detail_label()
                            .expect("detail-scoped locality edge has detail identity"),
                    ),
                }
            }),
        )?;
    }
    Ok(())
}
