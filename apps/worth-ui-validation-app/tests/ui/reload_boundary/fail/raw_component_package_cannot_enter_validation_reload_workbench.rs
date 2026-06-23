use worth_ui::facade::WorthUiComponentReloadPackage;
use worth_ui_validation_app::ValidationWorkbenchLaunch;

fn main() {
    let workbench = ValidationWorkbenchLaunch::new()
        .prepare()
        .unwrap()
        .into_runtime_workbench();
    let package = WorthUiComponentReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.components",
        "component_id = validation.component.header.dropdown",
    );

    let _ = workbench.prepare_component_capability_reload(&package);
}
