use crate::reload::ValidationManualReloadEdit;

use super::ValidationManualFlowId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationManualAppAction {
    ExecuteFlow(ValidationManualFlowId),
    ResetToBaseline,
    StageReloadEdit(ValidationManualReloadEdit),
    SubmitStagedReloadEdit,
    SelectDropdownCommand {
        projection_id: String,
        command_id: String,
    },
    AdvanceReloadCycle,
}

impl ValidationManualAppAction {
    pub fn select_dropdown_command(
        projection_id: impl Into<String>,
        command_id: impl Into<String>,
    ) -> Self {
        Self::SelectDropdownCommand {
            projection_id: projection_id.into(),
            command_id: command_id.into(),
        }
    }

    pub fn seed_dropdown_selection(
        projection_id: impl Into<String>,
        command_id: impl Into<String>,
    ) -> Self {
        Self::select_dropdown_command(projection_id, command_id)
    }
}
