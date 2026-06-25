use std::path::PathBuf;

use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationLiveViewSource,
    ValidationSourcePackage, ValidationThemeSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadLoopConfig {
    pub(super) source_path: Option<PathBuf>,
    pub(super) theme_path: PathBuf,
    pub(super) command_path: Option<PathBuf>,
    pub(super) command_projection_path: Option<PathBuf>,
    pub(super) component_path: Option<PathBuf>,
    pub(super) appearance_path: Option<PathBuf>,
    pub(super) density_path: Option<PathBuf>,
    pub(super) live_view_path: Option<PathBuf>,
    pub(super) initial_source: ValidationSourcePackage,
    pub(super) initial_theme: Option<ValidationThemeSource>,
    pub(super) initial_command: Option<ValidationCommandSource>,
    pub(super) initial_command_projection: Option<ValidationCommandProjectionSource>,
    pub(super) initial_component: Option<ValidationComponentSource>,
    pub(super) initial_appearance: Option<ValidationAppearanceSource>,
    pub(super) initial_density: Option<ValidationDensitySource>,
    pub(super) initial_live_view: Option<ValidationLiveViewSource>,
}

impl ValidationReloadLoopConfig {
    pub fn new(theme_path: impl Into<PathBuf>) -> Self {
        let initial_source = ValidationSourcePackage::sample();
        Self {
            source_path: None,
            theme_path: theme_path.into(),
            command_path: None,
            command_projection_path: None,
            component_path: None,
            appearance_path: None,
            density_path: None,
            live_view_path: None,
            initial_source,
            initial_theme: None,
            initial_command: None,
            initial_command_projection: None,
            initial_component: None,
            initial_appearance: None,
            initial_density: None,
            initial_live_view: None,
        }
    }

    pub fn with_source_path(mut self, source_path: impl Into<PathBuf>) -> Self {
        self.source_path = Some(source_path.into());
        self
    }

    pub fn with_theme_path(mut self, theme_path: impl Into<PathBuf>) -> Self {
        self.theme_path = theme_path.into();
        self
    }

    pub fn with_initial_source(mut self, initial_source: ValidationSourcePackage) -> Self {
        self.initial_source = initial_source;
        self
    }

    pub fn source_path(&self) -> Option<&PathBuf> {
        self.source_path.as_ref()
    }

    pub fn theme_path(&self) -> &PathBuf {
        &self.theme_path
    }

    pub fn with_initial_theme(mut self, initial_theme: ValidationThemeSource) -> Self {
        self.initial_theme = Some(initial_theme);
        self
    }

    pub fn with_command_path(mut self, command_path: impl Into<PathBuf>) -> Self {
        self.command_path = Some(command_path.into());
        self
    }

    pub fn command_path(&self) -> Option<&PathBuf> {
        self.command_path.as_ref()
    }

    pub fn with_initial_command(mut self, initial_command: ValidationCommandSource) -> Self {
        self.initial_command = Some(initial_command);
        self
    }

    pub fn with_command_projection_path(
        mut self,
        command_projection_path: impl Into<PathBuf>,
    ) -> Self {
        self.command_projection_path = Some(command_projection_path.into());
        self
    }

    pub fn command_projection_path(&self) -> Option<&PathBuf> {
        self.command_projection_path.as_ref()
    }

    pub fn with_initial_command_projection(
        mut self,
        initial_command_projection: ValidationCommandProjectionSource,
    ) -> Self {
        self.initial_command_projection = Some(initial_command_projection);
        self
    }

    pub fn with_appearance_path(mut self, appearance_path: impl Into<PathBuf>) -> Self {
        self.appearance_path = Some(appearance_path.into());
        self
    }

    pub fn appearance_path(&self) -> Option<&PathBuf> {
        self.appearance_path.as_ref()
    }

    pub fn with_initial_appearance(
        mut self,
        initial_appearance: ValidationAppearanceSource,
    ) -> Self {
        self.initial_appearance = Some(initial_appearance);
        self
    }

    pub fn with_component_path(mut self, component_path: impl Into<PathBuf>) -> Self {
        self.component_path = Some(component_path.into());
        self
    }

    pub fn component_path(&self) -> Option<&PathBuf> {
        self.component_path.as_ref()
    }

    pub fn with_initial_component(mut self, initial_component: ValidationComponentSource) -> Self {
        self.initial_component = Some(initial_component);
        self
    }

    pub fn with_density_path(mut self, density_path: impl Into<PathBuf>) -> Self {
        self.density_path = Some(density_path.into());
        self
    }

    pub fn density_path(&self) -> Option<&PathBuf> {
        self.density_path.as_ref()
    }

    pub fn with_initial_density(mut self, initial_density: ValidationDensitySource) -> Self {
        self.initial_density = Some(initial_density);
        self
    }

    pub fn with_live_view_path(mut self, live_view_path: impl Into<PathBuf>) -> Self {
        self.live_view_path = Some(live_view_path.into());
        self
    }

    pub fn live_view_path(&self) -> Option<&PathBuf> {
        self.live_view_path.as_ref()
    }

    pub fn with_initial_live_view(mut self, initial_live_view: ValidationLiveViewSource) -> Self {
        self.initial_live_view = Some(initial_live_view);
        self
    }

    pub(super) fn initial_source(&self) -> &ValidationSourcePackage {
        &self.initial_source
    }
}
