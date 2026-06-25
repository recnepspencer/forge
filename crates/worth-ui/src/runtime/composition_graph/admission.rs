use std::collections::{BTreeMap, BTreeSet};

use super::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionEdgeReceipt,
    WorthUiCompositionGraphAdmissionDenial, WorthUiCompositionGraphDefinition,
    WorthUiCompositionGraphDenialCode, WorthUiCompositionNodeKind, WorthUiCompositionNodeReceipt,
    WorthUiCompositionParentRef, WorthUiCompositionPolicyAttachmentReceipt,
    WorthUiCompositionRootReceipt,
};
use crate::runtime::WorthUiRuntimeGraphAuthority;

pub(super) fn admit_composition_graph(
    graph: WorthUiCompositionGraphDefinition,
) -> Result<WorthUiAdmittedCompositionGraphReceipt, Vec<WorthUiCompositionGraphAdmissionDenial>> {
    let denials = validate_graph(&graph);
    if !denials.is_empty() {
        return Err(denials);
    }
    let root = WorthUiCompositionRootReceipt::from_definition(graph.root());
    let mut nodes = graph
        .nodes()
        .iter()
        .map(WorthUiCompositionNodeReceipt::from_definition)
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.node_id().cmp(right.node_id()));
    let mut edges = graph
        .edges()
        .iter()
        .map(|edge| {
            WorthUiCompositionEdgeReceipt::new(
                edge.parent.clone(),
                edge.child.clone(),
                edge.order,
                edge.sizing,
            )
        })
        .collect::<Vec<_>>();
    edges.sort_by(|left, right| {
        left.parent()
            .identity()
            .cmp(right.parent().identity())
            .then_with(|| left.order().cmp(&right.order()))
            .then_with(|| left.child().cmp(right.child()))
    });
    let mut policy_attachments = graph
        .policy_attachments()
        .iter()
        .map(|attachment| {
            WorthUiCompositionPolicyAttachmentReceipt::new(
                attachment.node_id.clone(),
                attachment.policy_kind,
                attachment.policy_identity.clone(),
            )
        })
        .collect::<Vec<_>>();
    policy_attachments.sort_by(|left, right| {
        left.node_id()
            .cmp(right.node_id())
            .then_with(|| left.policy_kind().token().cmp(right.policy_kind().token()))
            .then_with(|| left.policy_identity().cmp(right.policy_identity()))
    });
    let mut dependency_facts = vec![root.fact_id().clone()];
    dependency_facts.extend(nodes.iter().map(|node| node.fact_id().clone()));
    dependency_facts.extend(edges.iter().map(|edge| edge.fact_id().clone()));
    dependency_facts.extend(
        policy_attachments
            .iter()
            .map(|attachment| attachment.fact_id().clone()),
    );
    let query_graph_execution = WorthUiRuntimeGraphAuthority::new()
        .plan_composition_topology_graph_operation(root.root_id().as_str(), dependency_facts)
        .into_execution_receipt();
    Ok(WorthUiAdmittedCompositionGraphReceipt::new(
        root,
        nodes,
        edges,
        policy_attachments,
        query_graph_execution,
    ))
}

fn validate_graph(
    graph: &WorthUiCompositionGraphDefinition,
) -> Vec<WorthUiCompositionGraphAdmissionDenial> {
    let mut denials = Vec::new();
    let node_kinds = node_kind_map(graph, &mut denials);
    validate_edges(graph, &node_kinds, &mut denials);
    validate_policy_attachments(graph, &node_kinds, &mut denials);
    validate_mounted_nodes(graph, &mut denials);
    validate_cycles(graph, &mut denials);
    denials
}

fn validate_policy_attachments(
    graph: &WorthUiCompositionGraphDefinition,
    node_kinds: &BTreeMap<String, WorthUiCompositionNodeKind>,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) {
    let mut attached_policy_kinds = BTreeSet::new();
    for attachment in graph.policy_attachments() {
        let Some(node_kind) = node_kinds.get(attachment.node_id.as_str()) else {
            denials.push(missing_policy_node_denial(attachment.node_id.as_str()));
            continue;
        };
        if !attachment.policy_kind.supports_node_kind(*node_kind) {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::UnsupportedPolicyNodeKind,
                attachment.node_id.as_str(),
                "composition policy kind must support the target node kind",
            ));
        }
        if !attached_policy_kinds.insert((
            attachment.node_id.as_str().to_owned(),
            attachment.policy_kind,
        )) {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::DuplicatePolicyAttachment,
                attachment.node_id.as_str(),
                "composition node may only have one attachment for a policy kind",
            ));
        }
    }
}

fn missing_policy_node_denial(node_id: &str) -> WorthUiCompositionGraphAdmissionDenial {
    WorthUiCompositionGraphAdmissionDenial::new(
        WorthUiCompositionGraphDenialCode::MissingPolicyNode,
        node_id,
        "composition policy attachment must reference an admitted node",
    )
}

fn node_kind_map(
    graph: &WorthUiCompositionGraphDefinition,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) -> BTreeMap<String, WorthUiCompositionNodeKind> {
    let mut node_kinds = BTreeMap::new();
    for node in graph.nodes() {
        if node_kinds
            .insert(node.node_id().as_str().to_owned(), node.kind())
            .is_some()
        {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::DuplicateNodeIdentity,
                node.node_id().as_str(),
                "composition node identity must be unique",
            ));
        }
    }
    node_kinds
}

fn validate_edges(
    graph: &WorthUiCompositionGraphDefinition,
    node_kinds: &BTreeMap<String, WorthUiCompositionNodeKind>,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) {
    let mut child_parent = BTreeMap::new();
    let mut parent_order = BTreeSet::new();
    for edge in graph.edges() {
        validate_edge_parent(edge, graph.root().root_id().as_str(), node_kinds, denials);
        if !node_kinds.contains_key(edge.child.as_str()) {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::MissingChild,
                edge.child.as_str(),
                "composition edge child must reference an admitted node",
            ));
        }
        if child_parent
            .insert(
                edge.child.as_str().to_owned(),
                edge.parent.identity().to_owned(),
            )
            .is_some()
        {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::MultipleParents,
                edge.child.as_str(),
                "composition node may only have one parent in this graph",
            ));
        }
        if !parent_order.insert((edge.parent.identity().to_owned(), edge.order)) {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::DuplicateChildOrder,
                edge.parent.identity(),
                "composition siblings must not share the same order under a parent",
            ));
        }
    }
}

fn validate_edge_parent(
    edge: &super::definition::WorthUiCompositionEdgeDefinition,
    root_id: &str,
    node_kinds: &BTreeMap<String, WorthUiCompositionNodeKind>,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) {
    match &edge.parent {
        WorthUiCompositionParentRef::Root(parent) if parent.as_str() == root_id => {}
        WorthUiCompositionParentRef::Root(parent) => {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::MissingParent,
                parent.as_str(),
                "composition root edge must reference the admitted root",
            ))
        }
        WorthUiCompositionParentRef::Node(parent) => match node_kinds.get(parent.as_str()) {
            Some(kind) if kind.can_parent_children() => {}
            Some(_) => denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::UnsupportedParentKind,
                parent.as_str(),
                "composition parent kind cannot contain children",
            )),
            None => denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::MissingParent,
                parent.as_str(),
                "composition edge parent must reference an admitted node",
            )),
        },
    }
}

fn validate_mounted_nodes(
    graph: &WorthUiCompositionGraphDefinition,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) {
    let children = graph
        .edges()
        .iter()
        .map(|edge| edge.child.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    for node in graph.nodes() {
        if !children.contains(node.node_id().as_str()) {
            denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                WorthUiCompositionGraphDenialCode::UnmountedNode,
                node.node_id().as_str(),
                "composition node must be mounted by an explicit parent edge",
            ));
        }
    }
}

fn validate_cycles(
    graph: &WorthUiCompositionGraphDefinition,
    denials: &mut Vec<WorthUiCompositionGraphAdmissionDenial>,
) {
    let mut parent_by_child = BTreeMap::new();
    for edge in graph.edges() {
        if let WorthUiCompositionParentRef::Node(parent) = &edge.parent {
            parent_by_child.insert(edge.child.as_str().to_owned(), parent.as_str().to_owned());
        }
    }
    for node in graph.nodes() {
        let mut seen = BTreeSet::new();
        let mut cursor = node.node_id().as_str().to_owned();
        while let Some(parent) = parent_by_child.get(&cursor) {
            if !seen.insert(cursor.clone()) {
                denials.push(WorthUiCompositionGraphAdmissionDenial::new(
                    WorthUiCompositionGraphDenialCode::Cycle,
                    node.node_id().as_str(),
                    "composition parent edges must not form a cycle",
                ));
                break;
            }
            cursor = parent.clone();
        }
    }
}
