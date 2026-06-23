use worth_ui_validation_app::{ValidationWorkbenchLaunch};

enum LocalDropdownModeState {
    Single,
    Multi,
}

fn main() {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare()
        .unwrap()
        .into_runtime_workbench();
    let local_mode_state = LocalDropdownModeState::Multi;
    let _ = workbench.apply_command_projection_source(local_mode_state);
}
