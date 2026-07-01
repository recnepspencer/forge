use std::collections::BTreeMap;

use crate::graph::{UiGraphNode, UiGraphNodeIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphNodeIdentityIndex {
    node_positions: BTreeMap<UiGraphNodeIdentity, usize>,
}

impl UiGraphNodeIdentityIndex {
    pub(crate) fn build(nodes: &[UiGraphNode]) -> Self {
        Self {
            node_positions: nodes
                .iter()
                .enumerate()
                .map(|(index, node)| (node.graph_node_identity(), index))
                .collect(),
        }
    }

    pub fn node<'a>(
        &self,
        nodes: &'a [UiGraphNode],
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&'a UiGraphNode> {
        self.node_positions
            .get(&graph_node_identity)
            .and_then(|index| nodes.get(*index))
    }
}
