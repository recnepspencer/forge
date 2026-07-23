use worth_query::facade::domain;

use super::super::{
    conditional_runtime_bridge, conditional_runtime_bridge_with_change,
    conditional_runtime_bridge_with_repeated_value_changes,
};
use super::providers::providers_for;

pub(crate) struct ConditionalInstallation {
    pub(crate) bridge: worth_runtime_bridge::facade::RuntimeBridge,
    pub(crate) graph: worth_signal::facade::SignalGraph,
    pub(crate) node_identity: String,
    pub(crate) signal_node: worth_signal::facade::NodeId,
    pub(crate) dependency: domain::WorthQueryConditionalDependencyInstallation,
    pub(crate) providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
}

pub(crate) fn conditional_installation(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> ConditionalInstallation {
    conditional_installation_in_partition(node, "geometry-signal")
}

fn conditional_installation_in_partition(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
    partition: &str,
) -> ConditionalInstallation {
    let dependency = &node.dependencies()[0];
    let bridge = conditional_runtime_bridge(dependency);
    conditional_installation_on_bridge(node, partition, bridge)
}

pub(super) fn conditional_installation_pair_in_partitions(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
    current_partition: &str,
    candidate_partition: &str,
) -> (ConditionalInstallation, ConditionalInstallation) {
    let bridge = conditional_runtime_bridge(&node.dependencies()[0]);
    (
        conditional_installation_on_bridge(node, current_partition, bridge.clone()),
        conditional_installation_on_bridge(node, candidate_partition, bridge),
    )
}

pub(super) fn conditional_installation_on_bridge(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
    partition: &str,
    bridge: worth_runtime_bridge::facade::RuntimeBridge,
) -> ConditionalInstallation {
    let dependency = &node.dependencies()[0];
    let mut graph = worth_signal::facade::SignalGraph::new();
    let signal_node_id = graph.node().build();
    let worth_proof::TransitionOutcome::Success(signal_node) =
        graph.admit_installed_node(signal_node_id)
    else {
        panic!("fresh Signal node should admit")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new(partition),
        signal_node,
    );
    let source = match dependency.locality() {
        domain::WorthQuerySemanticLocality::SourceRecord => {
            Some(worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 0, 1))
        }
        domain::WorthQuerySemanticLocality::SourcePartition(_)
        | domain::WorthQuerySemanticLocality::WholeLogicalGraph => None,
    };
    ConditionalInstallation {
        bridge,
        graph,
        node_identity: node.identity().to_string(),
        signal_node: signal_node_id,
        dependency: domain::WorthQueryConditionalDependencyInstallation::new(source, vec![target]),
        providers: providers_for(node),
    }
}

pub(crate) fn conditional_installation_with_change(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> (
    ConditionalInstallation,
    worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 2],
) {
    let dependency = &node.dependencies()[0];
    let (bridge, request, record, snapshots) = conditional_runtime_bridge_with_change(dependency);
    (
        conditional_installation_for_delivery(node, bridge, record),
        request,
        snapshots,
    )
}

pub(crate) fn conditional_installation_with_repeated_value_changes(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> (
    ConditionalInstallation,
    [worth_runtime_bridge::facade::RelationalCommittedPatchRequest; 2],
    [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 3],
) {
    let dependency = &node.dependencies()[0];
    let (bridge, requests, record, snapshots) =
        conditional_runtime_bridge_with_repeated_value_changes(dependency);
    (
        conditional_installation_for_delivery(node, bridge, record),
        requests,
        snapshots,
    )
}

fn conditional_installation_for_delivery(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
    bridge: worth_runtime_bridge::facade::RuntimeBridge,
    record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
) -> ConditionalInstallation {
    let mut graph = worth_signal::facade::SignalGraph::new();
    let signal_node_id = graph.node().build();
    let worth_proof::TransitionOutcome::Success(signal_node) =
        graph.admit_installed_node(signal_node_id)
    else {
        panic!("fresh Signal node should admit")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        signal_node,
    );
    ConditionalInstallation {
        bridge,
        graph,
        node_identity: node.identity().to_string(),
        signal_node: signal_node_id,
        dependency: domain::WorthQueryConditionalDependencyInstallation::new(
            Some(record),
            vec![target],
        ),
        providers: providers_for(node),
    }
}
