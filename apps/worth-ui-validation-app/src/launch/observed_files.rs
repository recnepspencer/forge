use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationSourcePackage,
    ValidationThemeSource,
};
use crate::sample_source::VALIDATION_SAMPLE_MODULE_PATH;

use super::ValidationWorkbenchAuthoredInputs;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationObservedWorkbenchFiles {
    source_path: PathBuf,
    theme_path: Option<PathBuf>,
    command_path: Option<PathBuf>,
    command_projection_path: Option<PathBuf>,
    component_path: Option<PathBuf>,
    appearance_path: Option<PathBuf>,
    density_path: Option<PathBuf>,
}

impl ValidationObservedWorkbenchFiles {
    pub fn new(source_path: impl Into<PathBuf>) -> Self {
        Self {
            source_path: source_path.into(),
            theme_path: None,
            command_path: None,
            command_projection_path: None,
            component_path: None,
            appearance_path: None,
            density_path: None,
        }
    }

    pub fn from_workspace_root(workspace_root: impl AsRef<Path>) -> Self {
        let workspace_root = workspace_root.as_ref();
        Self::new(workspace_root.join("source/header.wui"))
            .with_theme_path(workspace_root.join("theme/header.theme"))
            .with_command_path(workspace_root.join("theme/header.commands"))
            .with_command_projection_path(workspace_root.join("theme/header.projections"))
            .with_component_path(workspace_root.join("theme/header.components"))
            .with_appearance_path(workspace_root.join("theme/header.appearance"))
            .with_density_path(workspace_root.join("theme/header.density"))
    }

    pub fn with_theme_path(mut self, theme_path: impl Into<PathBuf>) -> Self {
        self.theme_path = Some(theme_path.into());
        self
    }

    pub fn with_command_path(mut self, command_path: impl Into<PathBuf>) -> Self {
        self.command_path = Some(command_path.into());
        self
    }

    pub fn with_command_projection_path(
        mut self,
        command_projection_path: impl Into<PathBuf>,
    ) -> Self {
        self.command_projection_path = Some(command_projection_path.into());
        self
    }

    pub fn with_component_path(mut self, component_path: impl Into<PathBuf>) -> Self {
        self.component_path = Some(component_path.into());
        self
    }

    pub fn with_appearance_path(mut self, appearance_path: impl Into<PathBuf>) -> Self {
        self.appearance_path = Some(appearance_path.into());
        self
    }

    pub fn with_density_path(mut self, density_path: impl Into<PathBuf>) -> Self {
        self.density_path = Some(density_path.into());
        self
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
}

impl ValidationWorkbenchAuthoredInputs {
    pub fn from_observed_files(files: &ValidationObservedWorkbenchFiles) -> io::Result<Self> {
        let mut inputs = Self::new(ValidationSourcePackage::new(
            VALIDATION_SAMPLE_MODULE_PATH,
            fs::read_to_string(files.source_path())?,
        ));
        if let Some(path) = files.theme_path.as_ref() {
            inputs = inputs.with_theme(ValidationThemeSource::from_observed_file(
                path.clone(),
                fs::read_to_string(path)?,
            ));
        }
        if let Some(path) = files.command_path.as_ref() {
            inputs = inputs.with_commands(ValidationCommandSource::from_observed_file(
                path.clone(),
                fs::read_to_string(path)?,
            ));
        }
        if let Some(path) = files.command_projection_path.as_ref() {
            inputs = inputs.with_command_projections(
                ValidationCommandProjectionSource::from_observed_file(
                    path.clone(),
                    fs::read_to_string(path)?,
                ),
            );
        }
        if let Some(path) = files.component_path.as_ref() {
            inputs = inputs.with_component(ValidationComponentSource::from_observed_file(
                path.clone(),
                fs::read_to_string(path)?,
            ));
        }
        if let Some(path) = files.appearance_path.as_ref() {
            inputs = inputs.with_appearance(ValidationAppearanceSource::from_observed_file(
                path.clone(),
                fs::read_to_string(path)?,
            ));
        }
        if let Some(path) = files.density_path.as_ref() {
            inputs = inputs.with_density(ValidationDensitySource::from_observed_file(
                path.clone(),
                fs::read_to_string(path)?,
            ));
        }
        Ok(inputs)
    }
}
