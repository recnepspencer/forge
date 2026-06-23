use std::fmt;
use std::path::PathBuf;

use super::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationSourcePackage,
    ValidationThemeSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationReloadInput {
    ObservedAuthoredBatch(ValidationObservedAuthoredBatch),
    SourcePackage(ValidationSourcePackage),
    HeaderTheme(ValidationThemeSource),
    HeaderCommands(ValidationCommandSource),
    HeaderCommandProjections(ValidationCommandProjectionSource),
    HeaderComponents(ValidationComponentSource),
    HeaderAppearance(ValidationAppearanceSource),
    HeaderDensity(ValidationDensitySource),
    HeaderAppearanceAndDensity {
        appearance: ValidationAppearanceSource,
        density: ValidationDensitySource,
    },
    SourcePackageAndHeaderTheme {
        source: ValidationSourcePackage,
        theme: ValidationThemeSource,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationObservedAuthoredBatch {
    source: ValidationSourcePackage,
    theme: Option<ValidationThemeSource>,
    command: Option<ValidationCommandSource>,
    command_projection: Option<ValidationCommandProjectionSource>,
    component: Option<ValidationComponentSource>,
    appearance: Option<ValidationAppearanceSource>,
    density: Option<ValidationDensitySource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadInputDenial {
    path: PathBuf,
    reason: String,
}

impl ValidationReloadInput {
    pub fn source_digest(&self) -> Option<u64> {
        match self {
            Self::ObservedAuthoredBatch(batch) => Some(batch.source().source_digest()),
            Self::SourcePackage(source) => Some(source.source_digest()),
            Self::HeaderTheme(_) => None,
            Self::HeaderCommands(_) => None,
            Self::HeaderCommandProjections(_) => None,
            Self::HeaderComponents(_) => None,
            Self::HeaderAppearance(_) => None,
            Self::HeaderDensity(_) => None,
            Self::HeaderAppearanceAndDensity { .. } => None,
            Self::SourcePackageAndHeaderTheme { source, .. } => Some(source.source_digest()),
        }
    }

    pub fn theme_digest(&self) -> Option<u64> {
        match self {
            Self::ObservedAuthoredBatch(batch) => {
                batch.theme().map(ValidationThemeSource::source_digest)
            }
            Self::SourcePackage(_) => None,
            Self::HeaderTheme(theme) => Some(theme.source_digest()),
            Self::HeaderCommands(_) => None,
            Self::HeaderCommandProjections(_) => None,
            Self::HeaderComponents(_) => None,
            Self::HeaderAppearance(_) => None,
            Self::HeaderDensity(_) => None,
            Self::HeaderAppearanceAndDensity { .. } => None,
            Self::SourcePackageAndHeaderTheme { theme, .. } => Some(theme.source_digest()),
        }
    }
}

impl ValidationObservedAuthoredBatch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: ValidationSourcePackage,
        theme: Option<ValidationThemeSource>,
        command: Option<ValidationCommandSource>,
        command_projection: Option<ValidationCommandProjectionSource>,
        component: Option<ValidationComponentSource>,
        appearance: Option<ValidationAppearanceSource>,
        density: Option<ValidationDensitySource>,
    ) -> Self {
        Self {
            source,
            theme,
            command,
            command_projection,
            component,
            appearance,
            density,
        }
    }

    pub fn source(&self) -> &ValidationSourcePackage {
        &self.source
    }

    pub fn theme(&self) -> Option<&ValidationThemeSource> {
        self.theme.as_ref()
    }

    pub fn command(&self) -> Option<&ValidationCommandSource> {
        self.command.as_ref()
    }

    pub fn command_projection(&self) -> Option<&ValidationCommandProjectionSource> {
        self.command_projection.as_ref()
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

    pub fn into_parts(
        self,
    ) -> (
        ValidationSourcePackage,
        Option<ValidationThemeSource>,
        Option<ValidationCommandSource>,
        Option<ValidationCommandProjectionSource>,
        Option<ValidationComponentSource>,
        Option<ValidationAppearanceSource>,
        Option<ValidationDensitySource>,
    ) {
        (
            self.source,
            self.theme,
            self.command,
            self.command_projection,
            self.component,
            self.appearance,
            self.density,
        )
    }
}

impl ValidationReloadInputDenial {
    pub fn unreadable(path: impl Into<PathBuf>, error: &std::io::Error) -> Self {
        Self {
            path: path.into(),
            reason: error.to_string(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for ValidationReloadInputDenial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path.display(), self.reason)
    }
}
