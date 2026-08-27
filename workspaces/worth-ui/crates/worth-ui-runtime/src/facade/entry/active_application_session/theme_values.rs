use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(crate) fn admit_application_theme_values(
        &mut self,
        changes: &[super::super::UiNativeThemeTokenValueChange],
    ) -> Result<(), ()> {
        let update = self.presentation.prepare_theme_values(changes)?;
        let mut changed_graph_nodes = Vec::new();
        for token in update.changed_tokens() {
            changed_graph_nodes
                .extend_from_slice(&self.application.theme_token_graph_consumers(token));
        }
        changed_graph_nodes.sort();
        changed_graph_nodes.dedup();
        self.presentation
            .commit_theme_values(update, changed_graph_nodes)
    }

    pub(crate) fn complete_application_theme_values_source(
        &self,
    ) -> crate::mounting::UiMountedThemeValueSource {
        let mut graph_nodes = Vec::new();
        for token in self.presentation.theme_token_ids() {
            graph_nodes.extend_from_slice(&self.application.theme_token_graph_consumers(token));
        }
        self.presentation
            .theme_values_source_with_graph_nodes(graph_nodes)
    }
}
