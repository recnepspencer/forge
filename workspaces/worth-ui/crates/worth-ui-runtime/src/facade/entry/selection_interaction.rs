use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn commit_selection_interaction(
        &mut self,
        activation: crate::facade::interaction::UiActivateInteraction,
        option: worth_ui_query_binding::UiProjectionOptionReference,
    ) -> Result<
        crate::facade::interaction::UiSelectionCommitInteraction,
        crate::facade::interaction::UiSelectionCommitStop,
    > {
        self.interaction
            .commit_selection(activation, option, &self.mounted)
    }
}
