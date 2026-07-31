use crate::capability::UiSemanticInteractionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentRouteBinding {
    graph_node: crate::graph::UiGraphNodeIdentity,
    declaration_index: u32,
    interaction: UiSemanticInteractionFamily,
}

impl UiIntentRouteBinding {
    pub(crate) const fn new(
        graph_node: crate::graph::UiGraphNodeIdentity,
        declaration_index: u32,
        interaction: UiSemanticInteractionFamily,
    ) -> Self {
        Self {
            graph_node,
            declaration_index,
            interaction,
        }
    }

    pub fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub fn interaction(&self) -> UiSemanticInteractionFamily {
        self.interaction
    }

    pub(crate) const fn declaration_index(&self) -> u32 {
        self.declaration_index
    }
}
