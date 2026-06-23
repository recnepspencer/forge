use worth_ui::facade::WorthUiDropdownFrameReceipt;

fn main() {
    let _forged = WorthUiDropdownFrameReceipt {
        projection_id: "validation.command_projection.header".to_owned(),
        component_id: "validation.component.header.dropdown".to_owned(),
        selection_mode: forged_selection_mode(),
        commands: forged_commands(),
        appearance: forged_appearance(),
        selection_state: forged_selection_state(),
        reconciliation: forged_reconciliation(),
    };
}

fn forged_selection_mode() -> worth_ui::facade::CommandProjectionSelectionMode {
    panic!("compile-fail fixture should never execute");
}

fn forged_commands() -> Vec<worth_ui::facade::WorthUiDropdownCommand> {
    panic!("compile-fail fixture should never execute");
}

fn forged_appearance() -> worth_ui::facade::WorthUiDropdownAppearanceFrameReceipt {
    panic!("compile-fail fixture should never execute");
}

fn forged_selection_state() -> worth_ui::facade::WorthUiDropdownSelectionState {
    panic!("compile-fail fixture should never execute");
}

fn forged_reconciliation() -> worth_ui::facade::WorthUiDropdownSelectionStateReconciliationReceipt {
    panic!("compile-fail fixture should never execute");
}
