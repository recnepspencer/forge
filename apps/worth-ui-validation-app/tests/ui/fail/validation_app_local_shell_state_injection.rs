use worth_ui_validation_app::{
    ValidationWorkbenchLaunch, ValidationWorkspaceShell, ValidationWorkspaceState,
};

fn main() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation launch should prepare");
    let mut shell = ValidationWorkspaceShell::from_launch(launch);
    let _state = ValidationWorkspaceState::default();
    shell.state_mut().set_rail_width(240.0);
}
