use std::{io, path::PathBuf};

use super::ValidationObservedWorkbenchFiles;

pub(super) fn default_observed_workbench_files() -> io::Result<ValidationObservedWorkbenchFiles> {
    Ok(
        ValidationObservedWorkbenchFiles::new(default_validation_source_path())
            .with_theme_path(default_header_theme_path())
            .with_command_path(default_header_command_path())
            .with_command_projection_path(default_header_command_projection_path())
            .with_component_path(default_header_component_path())
            .with_appearance_path(default_header_appearance_path())
            .with_density_path(default_header_density_path())
            .with_live_view_path(default_live_view_source_path()),
    )
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn default_validation_source_path() -> PathBuf {
    manifest_dir().join("source/header.wui")
}

fn default_live_view_source_path() -> PathBuf {
    manifest_dir().join("source/live_view.worth")
}

fn default_header_theme_path() -> PathBuf {
    manifest_dir().join("theme/header.theme")
}

fn default_header_command_path() -> PathBuf {
    manifest_dir().join("theme/header.commands")
}

fn default_header_command_projection_path() -> PathBuf {
    manifest_dir().join("theme/header.projections")
}

fn default_header_component_path() -> PathBuf {
    manifest_dir().join("theme/header.components")
}

fn default_header_appearance_path() -> PathBuf {
    manifest_dir().join("theme/header.appearance")
}

fn default_header_density_path() -> PathBuf {
    manifest_dir().join("theme/header.density")
}
