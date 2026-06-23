use crate::capability::{CommandId, CommandProjectionId};

use super::WorthUiDropdownSelectionState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownSelectionInteractionReceipt {
    projection_id: String,
    command_id: String,
    previous_selection_state: WorthUiDropdownSelectionState,
    next_selection_state: WorthUiDropdownSelectionState,
    status: WorthUiDropdownSelectionInteractionStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownSelectionInteractionStatus {
    SelectedSingle,
    AddedMultiSelection,
    AlreadySelected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownSelectionInteractionDenial {
    MissingProjection {
        projection_id: String,
    },
    CommandOutsideProjection {
        projection_id: String,
        command_id: String,
    },
}

impl WorthUiDropdownSelectionInteractionReceipt {
    pub(crate) fn new(
        projection_id: &CommandProjectionId,
        command_id: &CommandId,
        previous_selection_state: WorthUiDropdownSelectionState,
        next_selection_state: WorthUiDropdownSelectionState,
        status: WorthUiDropdownSelectionInteractionStatus,
    ) -> Self {
        Self {
            projection_id: projection_id.as_str().to_owned(),
            command_id: command_id.as_str().to_owned(),
            previous_selection_state,
            next_selection_state,
            status,
        }
    }

    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn previous_selection_state(&self) -> &WorthUiDropdownSelectionState {
        &self.previous_selection_state
    }

    pub fn next_selection_state(&self) -> &WorthUiDropdownSelectionState {
        &self.next_selection_state
    }

    pub fn status(&self) -> &WorthUiDropdownSelectionInteractionStatus {
        &self.status
    }
}
