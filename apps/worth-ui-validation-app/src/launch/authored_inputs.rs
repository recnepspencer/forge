use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationLiveViewSource,
    ValidationSourcePackage, ValidationThemeSource,
};
use crate::sample_source::{
    VALIDATION_SAMPLE_APPEARANCE_SOURCE, VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
    VALIDATION_SAMPLE_COMMAND_SOURCE, VALIDATION_SAMPLE_COMPONENT_SOURCE,
    VALIDATION_SAMPLE_DENSITY_SOURCE, VALIDATION_SAMPLE_THEME_SOURCE,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationWorkbenchAuthoredInputs {
    source: ValidationSourcePackage,
    theme: Option<ValidationThemeSource>,
    commands: Option<ValidationCommandSource>,
    command_projections: Option<ValidationCommandProjectionSource>,
    component: Option<ValidationComponentSource>,
    appearance: Option<ValidationAppearanceSource>,
    density: Option<ValidationDensitySource>,
    live_view: Option<ValidationLiveViewSource>,
}

impl ValidationWorkbenchAuthoredInputs {
    pub fn sample() -> Self {
        Self::new(ValidationSourcePackage::sample())
            .with_theme(ValidationThemeSource::new(VALIDATION_SAMPLE_THEME_SOURCE))
            .with_commands(ValidationCommandSource::new(
                VALIDATION_SAMPLE_COMMAND_SOURCE,
            ))
            .with_command_projections(ValidationCommandProjectionSource::new(
                VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
            ))
            .with_component(ValidationComponentSource::new(
                VALIDATION_SAMPLE_COMPONENT_SOURCE,
            ))
            .with_appearance(ValidationAppearanceSource::new(
                VALIDATION_SAMPLE_APPEARANCE_SOURCE,
            ))
            .with_density(ValidationDensitySource::new(
                VALIDATION_SAMPLE_DENSITY_SOURCE,
            ))
            .with_live_view_source(ValidationLiveViewSource::sample())
    }

    pub fn new(source: ValidationSourcePackage) -> Self {
        Self {
            source,
            theme: None,
            commands: None,
            command_projections: None,
            component: None,
            appearance: None,
            density: None,
            live_view: None,
        }
    }

    pub fn source(&self) -> &ValidationSourcePackage {
        &self.source
    }

    pub fn theme(&self) -> Option<&ValidationThemeSource> {
        self.theme.as_ref()
    }

    pub fn commands(&self) -> Option<&ValidationCommandSource> {
        self.commands.as_ref()
    }

    pub fn command_projections(&self) -> Option<&ValidationCommandProjectionSource> {
        self.command_projections.as_ref()
    }

    pub fn component(&self) -> Option<&ValidationComponentSource> {
        self.component.as_ref()
    }

    pub fn appearance(&self) -> Option<&ValidationAppearanceSource> {
        self.appearance.as_ref()
    }

    pub fn density(&self) -> Option<&ValidationDensitySource> {
        self.density.as_ref()
    }

    pub fn live_view(&self) -> Option<&ValidationLiveViewSource> {
        self.live_view.as_ref()
    }

    pub fn with_theme(mut self, theme: ValidationThemeSource) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_source(mut self, source: ValidationSourcePackage) -> Self {
        self.source = source;
        self
    }

    pub fn with_commands(mut self, commands: ValidationCommandSource) -> Self {
        self.commands = Some(commands);
        self
    }

    pub fn with_command_projections(
        mut self,
        command_projections: ValidationCommandProjectionSource,
    ) -> Self {
        self.command_projections = Some(command_projections);
        self
    }

    pub fn with_component(mut self, component: ValidationComponentSource) -> Self {
        self.component = Some(component);
        self
    }

    pub fn with_appearance(mut self, appearance: ValidationAppearanceSource) -> Self {
        self.appearance = Some(appearance);
        self
    }

    pub fn with_density(mut self, density: ValidationDensitySource) -> Self {
        self.density = Some(density);
        self
    }

    pub fn with_live_view_source(mut self, live_view: ValidationLiveViewSource) -> Self {
        self.live_view = Some(live_view);
        self
    }
}
