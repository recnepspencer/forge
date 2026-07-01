use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphPageMembership {
    page_node_identity: UiGraphNodeIdentity,
}

impl UiGraphPageMembership {
    pub(in crate::graph::topology) const fn new(page_node_identity: UiGraphNodeIdentity) -> Self {
        Self { page_node_identity }
    }

    pub fn page_node_identity(self) -> UiGraphNodeIdentity {
        self.page_node_identity
    }
}
