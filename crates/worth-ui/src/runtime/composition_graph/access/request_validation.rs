use super::denial::{WorthUiCompositionGraphAccessDenial, WorthUiCompositionGraphAccessDenialCode};
use super::indexes::WorthUiCompositionGraphIndexes;
use super::request::WorthUiCompositionGraphAccessRequest;
use crate::runtime::composition_graph::WorthUiAdmittedCompositionGraphReceipt;

pub(super) fn validate_access_request(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
    request: &WorthUiCompositionGraphAccessRequest,
) -> Vec<WorthUiCompositionGraphAccessDenial> {
    let root_id = graph.root().root_id().as_str();
    match request {
        WorthUiCompositionGraphAccessRequest::OrderedChildren { parent_id }
        | WorthUiCompositionGraphAccessRequest::ParticipatingDescendants { parent_id }
            if !indexes.contains_parent(parent_id, root_id) =>
        {
            vec![WorthUiCompositionGraphAccessDenial::new(
                WorthUiCompositionGraphAccessDenialCode::MissingParent,
                parent_id,
                "composition access parent must reference the admitted root or node",
            )]
        }
        WorthUiCompositionGraphAccessRequest::ParentOf { node_id }
        | WorthUiCompositionGraphAccessRequest::AncestorsOf { node_id }
        | WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedNode { node_id }
            if !indexes.contains_node(node_id.as_str()) =>
        {
            vec![WorthUiCompositionGraphAccessDenial::new(
                WorthUiCompositionGraphAccessDenialCode::MissingNode,
                node_id.as_str(),
                "composition access node must reference an admitted node",
            )]
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedEdge { edge_identity }
            if !indexes.contains_edge(edge_identity) =>
        {
            vec![WorthUiCompositionGraphAccessDenial::new(
                WorthUiCompositionGraphAccessDenialCode::MissingEdge,
                edge_identity,
                "composition access edge must reference an admitted edge fact",
            )]
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedPolicy {
            policy_identity,
        } if !indexes.contains_policy(policy_identity) => {
            vec![WorthUiCompositionGraphAccessDenial::new(
                WorthUiCompositionGraphAccessDenialCode::MissingPolicy,
                policy_identity,
                "composition access policy must reference an admitted policy fact",
            )]
        }
        _ => Vec::new(),
    }
}
