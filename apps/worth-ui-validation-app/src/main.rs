use worth_ui_validation_app::{ValidationWorkbenchApp, ValidationWorkbenchLaunch};

fn main() -> eframe::Result<()> {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation workbench launch should prepare through Worth UI facade");
    ValidationWorkbenchApp::run_native(launch)
}
