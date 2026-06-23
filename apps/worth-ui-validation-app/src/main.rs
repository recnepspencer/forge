use eframe::egui;
use worth_ui_validation_app::{
    validation_native_options, ValidationWorkbenchApp, ValidationWorkbenchLaunch,
};

fn main() -> eframe::Result<()> {
    match ValidationWorkbenchLaunch::new().prepare() {
        Ok(launch) => ValidationWorkbenchApp::run_native(launch),
        Err(error) => run_launch_error_window(error.to_string()),
    }
}

fn run_launch_error_window(error: String) -> eframe::Result<()> {
    eframe::run_simple_native(
        "Worth UI Validation App",
        validation_native_options(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Worth UI validation app failed to prepare");
                ui.add_space(12.0);
                ui.monospace(&error);
            });
        },
    )
}
