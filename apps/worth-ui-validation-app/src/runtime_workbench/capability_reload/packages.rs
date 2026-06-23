use worth_ui::facade::{
    WorthUiAppearanceReloadPackage, WorthUiCommandProjectionReloadPackage,
    WorthUiCommandReloadPackage, WorthUiComponentReloadPackage, WorthUiDensityReloadPackage,
    WorthUiThemeTokenReloadPackage,
};

use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationThemeSource,
};

pub(super) fn theme_token_reload_package(
    theme: &ValidationThemeSource,
) -> WorthUiThemeTokenReloadPackage {
    WorthUiThemeTokenReloadPackage::from_source(theme.source_path(), theme.source_text())
}

pub(super) fn command_reload_package(
    commands: &ValidationCommandSource,
) -> WorthUiCommandReloadPackage {
    WorthUiCommandReloadPackage::from_source(commands.source_path(), commands.source_text())
}

pub(super) fn command_projection_reload_package(
    command_projections: &ValidationCommandProjectionSource,
) -> WorthUiCommandProjectionReloadPackage {
    WorthUiCommandProjectionReloadPackage::from_source(
        command_projections.source_path(),
        command_projections.source_text(),
    )
}

pub(super) fn component_reload_package(
    component: &ValidationComponentSource,
) -> WorthUiComponentReloadPackage {
    WorthUiComponentReloadPackage::from_source(component.source_path(), component.source_text())
}

pub(super) fn appearance_reload_package(
    appearance: &ValidationAppearanceSource,
) -> WorthUiAppearanceReloadPackage {
    WorthUiAppearanceReloadPackage::from_source(appearance.source_path(), appearance.source_text())
}

pub(super) fn density_reload_package(
    density: &ValidationDensitySource,
) -> WorthUiDensityReloadPackage {
    WorthUiDensityReloadPackage::from_source(density.source_path(), density.source_text())
}
