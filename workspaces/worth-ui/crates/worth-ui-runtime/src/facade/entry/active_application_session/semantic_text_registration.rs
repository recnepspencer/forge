impl super::WorthUiActiveApplicationSession {
    pub(crate) fn register_application_semantic_text(
        &mut self,
        authored_identity: Box<str>,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<(), ()> {
        self.presentation
            .register_semantic_text(authored_identity, graph_node)
    }

    pub(crate) fn admit_application_semantic_text(
        &mut self,
        changes: &[crate::native_platform::UiNativeComponentSemanticTextChange],
    ) -> Result<(), ()> {
        self.presentation.admit_semantic_text(changes)
    }
}
