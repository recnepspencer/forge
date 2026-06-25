use std::fs;
use std::path::PathBuf;

use crate::reload::{ValidationManualReloadEdit, ValidationReloadLoopConfig};
use crate::ValidationWorkbenchAuthoredInputs;

pub(super) fn default_header_theme_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.theme")
}

pub(super) fn default_header_command_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.commands")
}

pub(super) fn default_header_command_projection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.projections")
}

pub(super) fn default_header_component_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.components")
}

pub(super) fn default_header_appearance_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.appearance")
}

pub(super) fn default_header_density_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.density")
}

pub(super) fn default_validation_source_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source/header.wui")
}

pub(super) fn default_live_view_source_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source/live_view.worth")
}

pub(super) fn resolve_manual_reload_edit(
    config: &ValidationReloadLoopConfig,
    edit: ValidationManualReloadEdit,
) -> ValidationManualReloadEdit {
    match edit {
        ValidationManualReloadEdit::SourceFile { source_text, .. } => {
            ValidationManualReloadEdit::source_file(
                config
                    .source_path()
                    .cloned()
                    .unwrap_or_else(default_validation_source_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::ThemeFile { source_text, .. } => {
            ValidationManualReloadEdit::theme_file(config.theme_path().clone(), source_text)
        }
        ValidationManualReloadEdit::CommandFile { source_text, .. } => {
            ValidationManualReloadEdit::command_file(
                config
                    .command_path()
                    .cloned()
                    .unwrap_or_else(default_header_command_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::CommandProjectionFile { source_text, .. } => {
            ValidationManualReloadEdit::command_projection_file(
                config
                    .command_projection_path()
                    .cloned()
                    .unwrap_or_else(default_header_command_projection_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::ComponentFile { source_text, .. } => {
            ValidationManualReloadEdit::component_file(
                config
                    .component_path()
                    .cloned()
                    .unwrap_or_else(default_header_component_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::AppearanceFile { source_text, .. } => {
            ValidationManualReloadEdit::appearance_file(
                config
                    .appearance_path()
                    .cloned()
                    .unwrap_or_else(default_header_appearance_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::DensityFile { source_text, .. } => {
            ValidationManualReloadEdit::density_file(
                config
                    .density_path()
                    .cloned()
                    .unwrap_or_else(default_header_density_path),
                source_text,
            )
        }
        ValidationManualReloadEdit::AppearanceAndDensityFiles {
            appearance_text,
            density_text,
            ..
        } => ValidationManualReloadEdit::appearance_and_density_files(
            config
                .appearance_path()
                .cloned()
                .unwrap_or_else(default_header_appearance_path),
            appearance_text,
            config
                .density_path()
                .cloned()
                .unwrap_or_else(default_header_density_path),
            density_text,
        ),
    }
}

pub(super) fn write_manual_reload_edit(
    config: &ValidationReloadLoopConfig,
    edit: ValidationManualReloadEdit,
) -> std::io::Result<()> {
    resolve_manual_reload_edit(config, edit).write_to_disk()
}

pub(super) fn restore_baseline_observed_files(
    config: &ValidationReloadLoopConfig,
    authored_inputs: &ValidationWorkbenchAuthoredInputs,
) -> std::io::Result<()> {
    if let Some(source_path) = config.source_path() {
        fs::write(source_path, authored_inputs.source().source_text())?;
    }
    fs::write(config.theme_path(), baseline_theme_text(authored_inputs))?;
    if let Some(command_path) = config.command_path() {
        if let Some(commands) = authored_inputs.commands() {
            fs::write(command_path, commands.source_text())?;
        }
    }
    if let Some(command_projection_path) = config.command_projection_path() {
        if let Some(command_projections) = authored_inputs.command_projections() {
            fs::write(command_projection_path, command_projections.source_text())?;
        }
    }
    if let Some(component_path) = config.component_path() {
        if let Some(component) = authored_inputs.component() {
            fs::write(component_path, component.source_text())?;
        }
    }
    if let Some(appearance_path) = config.appearance_path() {
        if let Some(appearance) = authored_inputs.appearance() {
            fs::write(appearance_path, appearance.source_text())?;
        }
    }
    if let Some(density_path) = config.density_path() {
        if let Some(density) = authored_inputs.density() {
            fs::write(density_path, density.source_text())?;
        }
    }
    if let Some(live_view_path) = config.live_view_path() {
        if let Some(live_view) = authored_inputs.live_view() {
            fs::write(live_view_path, live_view.source_text())?;
        }
    }
    Ok(())
}

fn baseline_theme_text(authored_inputs: &ValidationWorkbenchAuthoredInputs) -> &str {
    authored_inputs
        .theme()
        .map(|theme| theme.source_text())
        .unwrap_or_default()
}
