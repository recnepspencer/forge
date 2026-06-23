use worth_ui::facade::WorthUiDensityReloadPackage;
use worth_ui_validation_app::reload::ValidationReloadInput;

fn main() {
    let _ = ValidationReloadInput::HeaderDensity(WorthUiDensityReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.density",
        "validation.density.header.control_spacing = 12px",
    ));
}
