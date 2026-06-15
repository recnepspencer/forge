use worth_ui::facade::{
    WorthUiApp, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial,
    WorthUiRuntimeSourceModule,
};

pub fn minimal_workbench_launch(
    app: &WorthUiApp,
) -> Result<WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial> {
    worth_ui::facade::WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("app/main.wui", ""))
        .prepare_for(app)
}
