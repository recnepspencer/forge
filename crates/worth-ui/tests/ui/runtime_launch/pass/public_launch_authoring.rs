use worth_ui::facade::{
    WorthUi, WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeSourceModule,
};

fn main() {
    let app = WorthUi::app().freeze();
    let launch = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("app/main.wui", ""))
        .with_diagnostics(WorthUiRuntimeDiagnosticPolicy::minimal())
        .prepare_for(&app)
        .unwrap();
    let _ = app.launch_runtime(launch).unwrap();
}
