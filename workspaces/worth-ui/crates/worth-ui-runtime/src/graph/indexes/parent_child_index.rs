use std::collections::BTreeMap;

use crate::graph::UiGraphNodeIdentity;

const EMPTY_NODE_SET: [UiGraphNodeIdentity; 0] = [];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphParentChildIndex {
    children_by_parent: BTreeMap<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>,
}

impl UiGraphParentChildIndex {
    pub(crate) fn new(
        children_by_parent: BTreeMap<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>,
    ) -> Self {
        Self { children_by_parent }
    }

    pub fn children_of(&self, parent_node_identity: UiGraphNodeIdentity) -> &[UiGraphNodeIdentity] {
        self.children_by_parent
            .get(&parent_node_identity)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_SET)
    }
}
