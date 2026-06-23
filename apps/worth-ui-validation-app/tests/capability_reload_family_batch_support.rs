#![allow(dead_code)]

use worth_ui::facade::{
    AppearanceTokenId, CommandId, CommandProjectionId, DensityTokenId, ThemeTokenId,
    WorthUiCapabilityReloadRequest, WorthUiCommandProjectionReloadPackage,
    WorthUiCommandReloadPackage, WorthUiRuntimeFactId, WorthUiThemeTokenReloadPackage,
};
use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::reload::{ValidationAppearanceSource, ValidationDensitySource};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

pub fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
            ValidationSourcePackage::sample(),
        ))
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

pub fn assert_exact_changed_facts(evidence: &worth_ui::facade::WorthUiCapabilityReloadEvidence) {
    let changed = evidence.changed_facts();
    assert_eq!(changed.len(), 12);
    assert!(
        changed.contains_exact(&WorthUiRuntimeFactId::theme_token(&theme_id(
            "validation.theme.header.panel",
        )))
    );
    assert!(
        changed.contains_exact(&WorthUiRuntimeFactId::theme_token(&theme_id(
            "validation.theme.header.text",
        )))
    );
    assert!(
        changed.contains_exact(&WorthUiRuntimeFactId::command(&command_id(
            "validation.command.file.new",
        )))
    );
    assert!(
        changed.contains_exact(&WorthUiRuntimeFactId::command(&command_id(
            "validation.command.file.open",
        )))
    );
    for projection_id in [
        "validation.header.menu.file",
        "validation.header.menu.edit",
        "validation.header.menu.terminal",
        "validation.header.menu.help",
    ] {
        let projection_id = command_projection_id(projection_id);
        assert!(changed.contains_exact(&WorthUiRuntimeFactId::command_projection(&projection_id,)));
        assert!(changed.contains_exact(
            &WorthUiRuntimeFactId::command_projection_interaction_policy(&projection_id,)
        ));
    }
}

pub fn theme_id(raw: &str) -> ThemeTokenId {
    ThemeTokenId::new(raw).expect("test fixture uses valid theme token id")
}

pub fn density_id(raw: &str) -> DensityTokenId {
    DensityTokenId::new(raw).expect("test fixture uses valid density token id")
}

pub fn appearance_id(raw: &str) -> AppearanceTokenId {
    AppearanceTokenId::new(raw).expect("test fixture uses valid appearance token id")
}

fn command_id(raw: &str) -> CommandId {
    CommandId::new(raw).expect("test fixture uses valid command id")
}

fn command_projection_id(raw: &str) -> CommandProjectionId {
    CommandProjectionId::new(raw).expect("test fixture uses valid command projection id")
}

pub fn batch_request() -> WorthUiCapabilityReloadRequest {
    WorthUiCapabilityReloadRequest::batch([
        WorthUiCapabilityReloadRequest::from_theme_tokens(theme_package(
            "\
validation.theme.header.panel = #102030
validation.theme.header.text = #A0B0C0",
        )),
        WorthUiCapabilityReloadRequest::from_commands(command_package(
            "\
validation.command.file.new = Create File
validation.command.file.open = Open Workspace",
        )),
        WorthUiCapabilityReloadRequest::from_command_projections(projection_package(
            "\
validation.header.menu.file = multi
validation.header.menu.edit = multi
validation.header.menu.terminal = single
validation.header.menu.help = single",
        )),
    ])
}

pub fn theme_package(source: &str) -> WorthUiThemeTokenReloadPackage {
    WorthUiThemeTokenReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.theme",
        source,
    )
}

pub fn command_package(source: &str) -> WorthUiCommandReloadPackage {
    WorthUiCommandReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.commands",
        source,
    )
}

pub fn projection_package(source: &str) -> WorthUiCommandProjectionReloadPackage {
    WorthUiCommandProjectionReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.projections",
        source,
    )
}

pub fn mixed_appearance_source() -> ValidationAppearanceSource {
    ValidationAppearanceSource::from_observed_file(
        "apps/worth-ui-validation-app/theme/header.appearance",
        "validation.appearance.header.menu_min_width = 260px",
    )
}

pub fn mixed_density_source() -> ValidationDensitySource {
    ValidationDensitySource::from_observed_file(
        "apps/worth-ui-validation-app/theme/header.density",
        "validation.density.header.container_padding = 4px 8px",
    )
}
