use std::collections::BTreeMap;

use super::super::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionEdgeReceipt,
    WorthUiCompositionNodeReceipt, WorthUiCompositionParentRef, WorthUiCompositionParticipation,
    WorthUiCompositionPolicyAttachmentReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthUiCompositionGraphIndexes {
    nodes_by_id: BTreeMap<String, WorthUiCompositionNodeReceipt>,
    edges_by_identity: BTreeMap<String, WorthUiCompositionEdgeReceipt>,
    policies_by_identity: BTreeMap<String, WorthUiCompositionPolicyAttachmentReceipt>,
    children_by_parent: BTreeMap<String, Vec<WorthUiCompositionIndexedChild>>,
    parent_by_child: BTreeMap<String, String>,
    ancestors_by_node: BTreeMap<String, Vec<String>>,
    descendants_by_parent: BTreeMap<String, Vec<String>>,
    child_by_node: BTreeMap<String, WorthUiCompositionIndexedChild>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthUiCompositionIndexedChild {
    parent_id: String,
    edge: WorthUiCompositionEdgeReceipt,
    node: WorthUiCompositionNodeReceipt,
}

impl WorthUiCompositionGraphIndexes {
    pub(super) fn from_graph(graph: &WorthUiAdmittedCompositionGraphReceipt) -> Self {
        let nodes_by_id = graph
            .nodes()
            .iter()
            .cloned()
            .map(|node| (node.node_id().as_str().to_owned(), node))
            .collect::<BTreeMap<_, _>>();
        let edges_by_identity = graph
            .edges()
            .iter()
            .cloned()
            .map(|edge| (edge.fact_id().identity().to_owned(), edge))
            .collect::<BTreeMap<_, _>>();
        let policies_by_identity = graph
            .policy_attachments()
            .iter()
            .cloned()
            .map(|policy| (policy.fact_id().identity().to_owned(), policy))
            .collect::<BTreeMap<_, _>>();
        let parent_by_child = graph
            .edges()
            .iter()
            .map(|edge| {
                (
                    edge.child().as_str().to_owned(),
                    parent_identity(edge.parent()),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut children_by_parent = BTreeMap::<String, Vec<WorthUiCompositionIndexedChild>>::new();
        for edge in graph.edges() {
            let Some(node) = nodes_by_id.get(edge.child().as_str()) else {
                continue;
            };
            let parent_id = parent_identity(edge.parent());
            children_by_parent
                .entry(parent_id.clone())
                .or_default()
                .push(WorthUiCompositionIndexedChild {
                    parent_id,
                    edge: edge.clone(),
                    node: node.clone(),
                });
        }
        for children in children_by_parent.values_mut() {
            children.sort_by_key(|child| child.edge.order());
        }
        let mut child_by_node = BTreeMap::new();
        for children in children_by_parent.values() {
            for child in children {
                child_by_node.insert(child.node.node_id().as_str().to_owned(), child.clone());
            }
        }
        let root_id = graph.root().root_id().as_str().to_owned();
        let ancestors_by_node = ancestor_index(&root_id, nodes_by_id.keys(), &parent_by_child);
        let descendants_by_parent =
            descendant_index(&root_id, nodes_by_id.keys(), &children_by_parent);
        Self {
            nodes_by_id,
            edges_by_identity,
            policies_by_identity,
            children_by_parent,
            parent_by_child,
            ancestors_by_node,
            descendants_by_parent,
            child_by_node,
        }
    }

    pub(super) fn contains_parent(&self, parent_id: &str, root_id: &str) -> bool {
        parent_id == root_id || self.nodes_by_id.contains_key(parent_id)
    }

    pub(super) fn contains_node(&self, node_id: &str) -> bool {
        self.nodes_by_id.contains_key(node_id)
    }

    pub(super) fn contains_edge(&self, edge_identity: &str) -> bool {
        self.edges_by_identity.contains_key(edge_identity)
    }

    pub(super) fn contains_policy(&self, policy_identity: &str) -> bool {
        self.policies_by_identity.contains_key(policy_identity)
    }

    pub(super) fn children(&self, parent_id: &str) -> &[WorthUiCompositionIndexedChild] {
        self.children_by_parent
            .get(parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(super) fn parent_of(&self, node_id: &str) -> Option<&str> {
        self.parent_by_child.get(node_id).map(String::as_str)
    }

    pub(super) fn ancestors_of(&self, node_id: &str) -> &[String] {
        self.ancestors_by_node
            .get(node_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(super) fn participating_descendants(
        &self,
        parent_id: &str,
    ) -> Vec<&WorthUiCompositionIndexedChild> {
        self.descendants_by_parent
            .get(parent_id)
            .into_iter()
            .flat_map(|descendants| descendants.iter())
            .filter_map(|node_id| self.child_by_node.get(node_id))
            .filter(|child| child.node.participation() == WorthUiCompositionParticipation::Present)
            .collect()
    }
}

impl WorthUiCompositionIndexedChild {
    pub(super) fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub(super) fn edge(&self) -> &WorthUiCompositionEdgeReceipt {
        &self.edge
    }

    pub(super) fn node(&self) -> &WorthUiCompositionNodeReceipt {
        &self.node
    }
}

fn parent_identity(parent: &WorthUiCompositionParentRef) -> String {
    parent.identity().to_owned()
}

fn ancestor_index<'a>(
    root_id: &str,
    node_ids: impl Iterator<Item = &'a String>,
    parent_by_child: &BTreeMap<String, String>,
) -> BTreeMap<String, Vec<String>> {
    node_ids
        .map(|node_id| {
            let mut ancestors = Vec::new();
            let mut cursor = node_id.as_str();
            while let Some(parent) = parent_by_child.get(cursor) {
                ancestors.push(parent.clone());
                if parent == root_id {
                    break;
                }
                cursor = parent;
            }
            (node_id.clone(), ancestors)
        })
        .collect()
}

fn descendant_index<'a>(
    root_id: &str,
    node_ids: impl Iterator<Item = &'a String>,
    children_by_parent: &BTreeMap<String, Vec<WorthUiCompositionIndexedChild>>,
) -> BTreeMap<String, Vec<String>> {
    std::iter::once(root_id.to_owned())
        .chain(node_ids.cloned())
        .map(|parent_id| {
            let descendants = descendant_ids_for_parent(&parent_id, children_by_parent);
            (parent_id, descendants)
        })
        .collect()
}

fn descendant_ids_for_parent(
    parent_id: &str,
    children_by_parent: &BTreeMap<String, Vec<WorthUiCompositionIndexedChild>>,
) -> Vec<String> {
    let mut descendants = Vec::new();
    let mut pending_parents = vec![parent_id.to_owned()];
    while let Some(current_parent_id) = pending_parents.pop() {
        if let Some(children) = children_by_parent.get(&current_parent_id) {
            for child in children.iter().rev() {
                pending_parents.push(child.node.node_id().as_str().to_owned());
            }
            for child in children {
                let child_id = child.node.node_id().as_str().to_owned();
                descendants.push(child_id);
            }
        }
    }
    descendants
}
