use super::indexes::WorthUiCompositionGraphIndexes;
use super::request::WorthUiCompositionGraphAccessRequest;
use crate::runtime::composition_graph::WorthUiAdmittedCompositionGraphReceipt;
use crate::runtime::WorthUiRuntimeFactId;

pub(super) fn consumed_facts_for_request(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
    request: &WorthUiCompositionGraphAccessRequest,
) -> Vec<WorthUiRuntimeFactId> {
    let mut consumed_facts = vec![graph.root().fact_id().clone()];
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => {
            consumed_facts.extend(graph.nodes().iter().map(|node| node.fact_id().clone()));
            consumed_facts.extend(graph.edges().iter().map(|edge| edge.fact_id().clone()));
        }
        WorthUiCompositionGraphAccessRequest::RootChildren => {
            consumed_facts.extend(root_child_facts(graph, indexes));
        }
        WorthUiCompositionGraphAccessRequest::OrderedChildren { parent_id } => {
            consumed_facts.push(WorthUiRuntimeFactId::composition_node(parent_id.clone()));
            consumed_facts.extend(child_facts(parent_id, indexes));
        }
        WorthUiCompositionGraphAccessRequest::ParticipatingDescendants { parent_id } => {
            consumed_facts.push(WorthUiRuntimeFactId::composition_node(parent_id.clone()));
            consumed_facts.extend(participating_descendant_facts(parent_id, indexes));
        }
        WorthUiCompositionGraphAccessRequest::ParentOf { node_id }
        | WorthUiCompositionGraphAccessRequest::AncestorsOf { node_id } => {
            consumed_facts.push(WorthUiRuntimeFactId::composition_node(node_id.as_str()));
            consumed_facts.extend(ancestor_facts(node_id.as_str(), indexes));
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedNode { node_id } => {
            consumed_facts.push(WorthUiRuntimeFactId::composition_node(node_id.as_str()));
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedEdge { edge_identity } => {
            consumed_facts.push(WorthUiRuntimeFactId::composition_edge(
                edge_identity.clone(),
            ));
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedPolicy {
            policy_identity,
        } => consumed_facts.push(WorthUiRuntimeFactId::composition_policy(
            policy_identity.clone(),
        )),
    }
    consumed_facts.sort();
    consumed_facts.dedup();
    consumed_facts
}

fn root_child_facts(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiRuntimeFactId> {
    child_facts(graph.root().root_id().as_str(), indexes)
}

fn child_facts(
    parent_id: &str,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiRuntimeFactId> {
    indexes
        .children(parent_id)
        .iter()
        .flat_map(|child| {
            [
                child.node().fact_id().clone(),
                child.edge().fact_id().clone(),
            ]
        })
        .collect()
}

fn participating_descendant_facts(
    parent_id: &str,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiRuntimeFactId> {
    indexes
        .participating_descendants(parent_id)
        .into_iter()
        .flat_map(|child| {
            [
                child.node().fact_id().clone(),
                child.edge().fact_id().clone(),
            ]
        })
        .collect()
}

fn ancestor_facts(
    node_id: &str,
    indexes: &WorthUiCompositionGraphIndexes,
) -> Vec<WorthUiRuntimeFactId> {
    indexes
        .ancestors_of(node_id)
        .iter()
        .map(|ancestor| WorthUiRuntimeFactId::composition_node(ancestor.clone()))
        .collect()
}
