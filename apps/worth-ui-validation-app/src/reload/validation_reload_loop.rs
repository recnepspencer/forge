use std::fs;
use std::path::PathBuf;

use crate::sample_source::VALIDATION_SAMPLE_MODULE_PATH;

use super::{
    ValidationReloadInput, ValidationReloadInputDenial, ValidationReloadObservation,
    ValidationReloadTick, ValidationSourcePackage, ValidationThemeSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadLoopConfig {
    source_path: Option<PathBuf>,
    theme_path: PathBuf,
    initial_source: ValidationSourcePackage,
}

#[derive(Debug)]
pub struct ValidationReloadLoop {
    config: ValidationReloadLoopConfig,
    last_source_digest: u64,
    last_theme_digest: u64,
}

impl ValidationReloadLoopConfig {
    pub fn new(theme_path: impl Into<PathBuf>) -> Self {
        let initial_source = ValidationSourcePackage::sample();
        Self {
            source_path: None,
            theme_path: theme_path.into(),
            initial_source,
        }
    }

    pub fn with_source_path(mut self, source_path: impl Into<PathBuf>) -> Self {
        self.source_path = Some(source_path.into());
        self
    }

    pub fn source_path(&self) -> Option<&PathBuf> {
        self.source_path.as_ref()
    }

    pub fn theme_path(&self) -> &PathBuf {
        &self.theme_path
    }

    fn initial_source(&self) -> &ValidationSourcePackage {
        &self.initial_source
    }
}

impl ValidationReloadLoop {
    pub fn start(config: ValidationReloadLoopConfig) -> Result<Self, ValidationReloadInputDenial> {
        let theme = read_theme_source(config.theme_path())?;
        Ok(Self {
            last_source_digest: config.initial_source().source_digest(),
            last_theme_digest: theme.source_digest(),
            config,
        })
    }

    pub fn poll_inputs(&mut self) -> ValidationReloadTick {
        let source = match self.read_source_package() {
            Ok(source) => source,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };
        let theme = match read_theme_source(self.config.theme_path()) {
            Ok(theme) => theme,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };

        let source_changed = source.source_digest() != self.last_source_digest;
        let theme_changed = theme.source_digest() != self.last_theme_digest;
        self.last_source_digest = source.source_digest();
        self.last_theme_digest = theme.source_digest();

        match (source_changed, theme_changed) {
            (false, false) => ValidationReloadTick::Unchanged(ValidationReloadObservation::new(
                source.source_digest(),
                theme.source_digest(),
            )),
            (true, false) => {
                ValidationReloadTick::Changed(ValidationReloadInput::SourcePackage(source))
            }
            (false, true) => {
                ValidationReloadTick::Changed(ValidationReloadInput::HeaderTheme(theme))
            }
            (true, true) => {
                ValidationReloadTick::Changed(ValidationReloadInput::SourcePackageAndHeaderTheme {
                    source,
                    theme,
                })
            }
        }
    }

    fn read_source_package(&self) -> Result<ValidationSourcePackage, ValidationReloadInputDenial> {
        let Some(source_path) = self.config.source_path() else {
            return Ok(self.config.initial_source().clone());
        };
        let source_text = fs::read_to_string(source_path)
            .map_err(|error| ValidationReloadInputDenial::unreadable(source_path, &error))?;
        Ok(ValidationSourcePackage::new(
            VALIDATION_SAMPLE_MODULE_PATH,
            source_text,
        ))
    }
}

fn read_theme_source(path: &PathBuf) -> Result<ValidationThemeSource, ValidationReloadInputDenial> {
    fs::read_to_string(path)
        .map(ValidationThemeSource::new)
        .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
}
