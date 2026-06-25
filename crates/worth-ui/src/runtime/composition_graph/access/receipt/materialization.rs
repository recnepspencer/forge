use crate::runtime::WorthUiRuntimeFactId;

use super::{
    WorthUiCompositionGraphAffectedConsumerRow, WorthUiCompositionGraphAncestorAccessRow,
    WorthUiCompositionGraphChildAccessRow,
};
use crate::runtime::composition_graph::access::indexes::{
    WorthUiCompositionGraphIndexes, WorthUiCompositionIndexedChild,
};
use crate::runtime::composition_graph::access::request::WorthUiCompositionGraphAccessRequest;
use crate::runtime::composition_graph::WorthUiAdmittedCompositionGraphReceipt;

pub(super) fn root_child_rows_for_request(
    request: &WorthUiCompositionGraphAccessRequest,
    root_id: &str,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiCompositionGraphChildAccessRow> {
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree
        | WorthUiCompositionGraphAccessRequest::RootChildren => {
            child_rows(indexes.children(root_id))
        }
        _ => Vec::new(),
    }
}

pub(super) fn child_rows_for_request(
    request: &WorthUiCompositionGraphAccessRequest,
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiCompositionGraphChildAccessRow> {
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => {
            std::iter::once(graph.root().root_id().as_str().to_owned())
                .chain(
                    graph
                        .nodes()
                        .iter()
                        .map(|node| node.node_id().as_str().to_owned()),
                )
                .flat_map(|parent_id| child_rows(indexes.children(&parent_id)))
                .collect()
        }
        WorthUiCompositionGraphAccessRequest::RootChildren => {
            child_rows(indexes.children(graph.root().root_id().as_str()))
        }
        WorthUiCompositionGraphAccessRequest::OrderedChildren { parent_id } => {
            child_rows(indexes.children(parent_id))
        }
        _ => Vec::new(),
    }
}

pub(super) fn ancestor_rows_for_request(
    request: &WorthUiCompositionGraphAccessRequest,
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiCompositionGraphAncestorAccessRow> {
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => graph
            .nodes()
            .iter()
            .flat_map(|node| ancestor_rows(indexes, node.node_id().as_str()))
            .collect(),
        WorthUiCompositionGraphAccessRequest::ParentOf { node_id } => indexes
            .parent_of(node_id.as_str())
            .map(|parent| {
                vec![WorthUiCompositionGraphAncestorAccessRow::new(
                    node_id.as_str(),
                    parent.to_owned(),
                    0,
                )]
            })
            .unwrap_or_default(),
        WorthUiCompositionGraphAccessRequest::AncestorsOf { node_id } => {
            ancestor_rows(indexes, node_id.as_str())
        }
        _ => Vec::new(),
    }
}

pub(super) fn participating_descendant_rows_for_request(
    request: &WorthUiCompositionGraphAccessRequest,
    root_id: &str,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiCompositionGraphChildAccessRow> {
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => {
            child_rows_from_refs(indexes.participating_descendants(root_id))
        }
        WorthUiCompositionGraphAccessRequest::ParticipatingDescendants { parent_id } => {
            child_rows_from_refs(indexes.participating_descendants(parent_id))
        }
        _ => Vec::new(),
    }
}

pub(super) fn affected_consumer_rows_for_request(
    request: &WorthUiCompositionGraphAccessRequest,
    graph: &WorthUiAdmittedCompositionGraphReceipt,
) -> Vec<WorthUiCompositionGraphAffectedConsumerRow> {
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => affected_consumer_rows(
            graph,
            graph
                .edges()
                .iter()
                .map(|edge| edge.fact_id().clone())
                .collect(),
        ),
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedNode { node_id } => {
            affected_consumer_rows(
                graph,
                vec![WorthUiRuntimeFactId::composition_node(node_id.as_str())],
            )
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedEdge { edge_identity } => {
            affected_consumer_rows(
                graph,
                vec![WorthUiRuntimeFactId::composition_edge(
                    edge_identity.clone(),
                )],
            )
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedPolicy {
            policy_identity,
        } => affected_consumer_rows(
            graph,
            vec![WorthUiRuntimeFactId::composition_policy(
                policy_identity.clone(),
            )],
        ),
        _ => Vec::new(),
    }
}

fn child_rows(
    children: &[WorthUiCompositionIndexedChild],
) -> Vec<WorthUiCompositionGraphChildAccessRow> {
    child_rows_from_refs(children.iter().collect())
}

fn child_rows_from_refs(
    children: Vec<&WorthUiCompositionIndexedChild>,
) -> Vec<WorthUiCompositionGraphChildAccessRow> {
    children
        .into_iter()
        .map(|child| {
            WorthUiCompositionGraphChildAccessRow::new(
                child.parent_id().to_owned(),
                child.edge().clone(),
                child.node().clone(),
            )
        })
        .collect()
}

fn ancestor_rows(
    indexes: &WorthUiCompositionGraphIndexes,
    node_id: &str,
) -> Vec<WorthUiCompositionGraphAncestorAccessRow> {
    indexes
        .ancestors_of(node_id)
        .iter()
        .enumerate()
        .map(|(depth, ancestor)| {
            WorthUiCompositionGraphAncestorAccessRow::new(node_id, ancestor.clone(), depth)
        })
        .collect()
}

fn affected_consumer_rows(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    changed_facts: Vec<WorthUiRuntimeFactId>,
) -> Vec<WorthUiCompositionGraphAffectedConsumerRow> {
    let topology_fact = WorthUiRuntimeFactId::composition_topology(graph.root().root_id().as_str());
    changed_facts
        .into_iter()
        .map(|changed_fact| {
            WorthUiCompositionGraphAffectedConsumerRow::new(changed_fact, topology_fact.clone())
        })
        .collect()
}
