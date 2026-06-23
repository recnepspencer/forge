use worth_ui::facade::{
    WorthUiDropdownSelectionInteractionReceipt, WorthUiDropdownSelectionInteractionStatus,
    WorthUiDropdownSelectionState,
};

fn main() {
    let _forged = WorthUiDropdownSelectionInteractionReceipt {
        projection_id: "validation.command_projection.header".to_owned(),
        command_id: "validation.command.header.refresh".to_owned(),
        previous_selection_state: WorthUiDropdownSelectionState::None,
        next_selection_state: WorthUiDropdownSelectionState::Single(
            "validation.command.header.refresh".to_owned(),
        ),
        status: WorthUiDropdownSelectionInteractionStatus::SelectedSingle,
    };
}
