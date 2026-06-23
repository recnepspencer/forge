use worth_ui::facade::WorthUiAppearanceReloadPackage;
use worth_ui_validation_app::reload::ValidationReloadInput;

fn main() {
    let _ = ValidationReloadInput::HeaderAppearance(
        WorthUiAppearanceReloadPackage::from_source(
            "apps/worth-ui-validation-app/theme/header.appearance",
            "validation.appearance.header.menu_min_width = 260px",
        ),
    );
}
