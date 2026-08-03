use super::WorthUiActiveApplicationSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCurrentProjectionOptionStop {
    ProjectionNotRegistered,
    ProjectionUnavailable,
    ProjectionIdentityChanged,
    ProjectionShapeMismatch,
    ProjectionNotCurrent(worth_ui_query_binding::UiProjectionInputPosture),
    RowUnavailable,
}

impl WorthUiActiveApplicationSession {
    pub fn current_projection_option(
        &self,
        projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
        row: &worth_ui_query_binding::UiCollectionProjectionRowReference,
    ) -> Result<worth_ui_query_binding::UiProjectionOptionReference, UiCurrentProjectionOptionStop>
    {
        let slot = self
            .application
            .prepared_authority()
            .query_binding_plan()
            .projection_input_slot(projection)
            .ok_or(UiCurrentProjectionOptionStop::ProjectionNotRegistered)?;
        let input = self
            .mounted
            .current_projection_input(slot)
            .ok_or(UiCurrentProjectionOptionStop::ProjectionUnavailable)?;
        if input.revision().projection_identity() != projection {
            return Err(UiCurrentProjectionOptionStop::ProjectionIdentityChanged);
        }
        let worth_ui_query_binding::UiProjectionInputFactReference::Collection(collection) = input
        else {
            return Err(UiCurrentProjectionOptionStop::ProjectionShapeMismatch);
        };
        if collection.posture() != worth_ui_query_binding::UiProjectionInputPosture::Current {
            return Err(UiCurrentProjectionOptionStop::ProjectionNotCurrent(
                collection.posture(),
            ));
        }
        collection
            .current_option(row)
            .ok_or(UiCurrentProjectionOptionStop::RowUnavailable)
    }

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
