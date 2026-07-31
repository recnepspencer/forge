#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentConfirmationRouteBinding {
    graph_node: crate::graph::UiGraphNodeIdentity,
    declaration_index: u32,
}

impl UiIntentConfirmationRouteBinding {
    pub(crate) const fn new(
        graph_node: crate::graph::UiGraphNodeIdentity,
        declaration_index: u32,
    ) -> Self {
        Self {
            graph_node,
            declaration_index,
        }
    }

    pub fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub(crate) const fn declaration_index(&self) -> u32 {
        self.declaration_index
    }
}
