use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    /// Borrow admission authority from the generation currently executing.
    pub(crate) fn admission(&self) -> crate::admission::UiAdmissionBoundary<'_> {
        self.application.admission()
    }

    pub(crate) fn try_allocation_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        self.application
            .try_allocation_touch_for_node(graph_node_identity)
    }
}
