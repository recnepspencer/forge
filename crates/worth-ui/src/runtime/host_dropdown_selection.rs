use crate::capability::{CommandId, CommandProjectionId, CommandProjectionSelectionMode};
use crate::runtime::{
    WorthUiDropdownSelectionInteractionDenial, WorthUiDropdownSelectionInteractionReceipt,
    WorthUiDropdownSelectionInteractionStatus, WorthUiDropdownSelectionState, WorthUiRuntimeHost,
};

impl WorthUiRuntimeHost {
    pub fn select_dropdown_command(
        &mut self,
        projection_id: &CommandProjectionId,
        command_id: &CommandId,
    ) -> Result<WorthUiDropdownSelectionInteractionReceipt, WorthUiDropdownSelectionInteractionDenial>
    {
        let projection = self
            .active_state_for_read()
            .capability_snapshot()
            .command_projections()
            .get(projection_id)
            .ok_or_else(
                || WorthUiDropdownSelectionInteractionDenial::MissingProjection {
                    projection_id: projection_id.as_str().to_owned(),
                },
            )?;
        let valid_command_ids = projection
            .command_references()
            .iter()
            .map(|reference| reference.command_id().as_str().to_owned())
            .collect::<Vec<_>>();
        if !valid_command_ids
            .iter()
            .any(|candidate| candidate == command_id.as_str())
        {
            return Err(
                WorthUiDropdownSelectionInteractionDenial::CommandOutsideProjection {
                    projection_id: projection_id.as_str().to_owned(),
                    command_id: command_id.as_str().to_owned(),
                },
            );
        }

        let previous_selection_state = self
            .active_state_for_read()
            .dropdown_selection_state(projection_id)
            .cloned()
            .unwrap_or_else(|| {
                WorthUiDropdownSelectionState::empty_for_mode(projection.selection_mode())
            });
        let (next_selection_state, status) = next_selection_state(
            &previous_selection_state,
            projection.selection_mode(),
            command_id,
            &valid_command_ids,
        );
        self.active_state_for_swap_mut()
            .record_dropdown_selection_state(projection_id, &next_selection_state);
        Ok(WorthUiDropdownSelectionInteractionReceipt::new(
            projection_id,
            command_id,
            previous_selection_state,
            next_selection_state,
            status,
        ))
    }
}

fn next_selection_state(
    previous_selection_state: &WorthUiDropdownSelectionState,
    mode: CommandProjectionSelectionMode,
    command_id: &CommandId,
    valid_command_ids: &[String],
) -> (
    WorthUiDropdownSelectionState,
    WorthUiDropdownSelectionInteractionStatus,
) {
    match mode {
        CommandProjectionSelectionMode::SingleSelect => {
            let next = WorthUiDropdownSelectionState::Single(command_id.as_str().to_owned());
            let status = if previous_selection_state.contains(command_id.as_str()) {
                WorthUiDropdownSelectionInteractionStatus::AlreadySelected
            } else {
                WorthUiDropdownSelectionInteractionStatus::SelectedSingle
            };
            (next, status)
        }
        CommandProjectionSelectionMode::MultiSelect => {
            let mut selected = previous_selection_state.selected_command_ids();
            let status = if selected
                .iter()
                .any(|candidate| candidate == command_id.as_str())
            {
                WorthUiDropdownSelectionInteractionStatus::AlreadySelected
            } else {
                selected.push(command_id.as_str().to_owned());
                WorthUiDropdownSelectionInteractionStatus::AddedMultiSelection
            };
            let next = WorthUiDropdownSelectionState::Multi(
                valid_command_ids
                    .iter()
                    .filter(|candidate| {
                        selected.iter().any(|selected_id| selected_id == *candidate)
                    })
                    .cloned()
                    .collect(),
            );
            (next, status)
        }
    }
}
