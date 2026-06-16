use std::fs;
use std::path::PathBuf;

use crate::sample_source::VALIDATION_SAMPLE_MODULE_PATH;

use super::{
    ValidationCommandProjectionSource, ValidationCommandSource, ValidationReloadInput,
    ValidationReloadInputDenial, ValidationReloadObservation, ValidationReloadTick,
    ValidationSourcePackage, ValidationThemeSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadLoopConfig {
    source_path: Option<PathBuf>,
    theme_path: PathBuf,
    command_path: Option<PathBuf>,
    command_projection_path: Option<PathBuf>,
    initial_source: ValidationSourcePackage,
}

#[derive(Debug)]
pub struct ValidationReloadLoop {
    config: ValidationReloadLoopConfig,
    last_source_digest: u64,
    last_theme_digest: u64,
    last_command_digest: Option<u64>,
    last_command_projection_digest: Option<u64>,
}

impl ValidationReloadLoopConfig {
    pub fn new(theme_path: impl Into<PathBuf>) -> Self {
        let initial_source = ValidationSourcePackage::sample();
        Self {
            source_path: None,
            theme_path: theme_path.into(),
            command_path: None,
            command_projection_path: None,
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

    fn initial_source(&self) -> &ValidationSourcePackage {
        &self.initial_source
    }
}

impl ValidationReloadLoop {
    pub fn start(config: ValidationReloadLoopConfig) -> Result<Self, ValidationReloadInputDenial> {
        let theme = read_theme_source(config.theme_path())?;
        let command = read_optional_command_source(config.command_path.as_ref())?;
        let command_projection =
            read_optional_command_projection_source(config.command_projection_path.as_ref())?;
        Ok(Self {
            last_source_digest: config.initial_source().source_digest(),
            last_theme_digest: theme.source_digest(),
            last_command_digest: command.as_ref().map(ValidationCommandSource::source_digest),
            last_command_projection_digest: command_projection
                .as_ref()
                .map(ValidationCommandProjectionSource::source_digest),
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
        let command = match read_optional_command_source(self.config.command_path.as_ref()) {
            Ok(command) => command,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };
        let command_projection = match read_optional_command_projection_source(
            self.config.command_projection_path.as_ref(),
        ) {
            Ok(command_projection) => command_projection,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };

        let source_changed = source.source_digest() != self.last_source_digest;
        let theme_changed = theme.source_digest() != self.last_theme_digest;
        let command_changed = optional_digest_changed(&command, self.last_command_digest);
        let command_projection_changed =
            optional_digest_changed(&command_projection, self.last_command_projection_digest);
        self.last_source_digest = source.source_digest();
        self.last_theme_digest = theme.source_digest();
        self.last_command_digest = command.as_ref().map(ValidationCommandSource::source_digest);
        self.last_command_projection_digest = command_projection
            .as_ref()
            .map(ValidationCommandProjectionSource::source_digest);

        if command_changed {
            let Some(command) = command else {
                unreachable!("changed optional command source must exist");
            };
            return ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommands(command));
        }
        if command_projection_changed {
            let Some(command_projection) = command_projection else {
                unreachable!("changed optional command projection source must exist");
            };
            return ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommandProjections(
                command_projection,
            ));
        }

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

fn read_optional_command_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationCommandSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(ValidationCommandSource::new)
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn read_optional_command_projection_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationCommandProjectionSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(ValidationCommandProjectionSource::new)
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn optional_digest_changed<T>(source: &Option<T>, previous_digest: Option<u64>) -> bool
where
    T: OptionalReloadDigest,
{
    source.as_ref().map(T::source_digest) != previous_digest
}

trait OptionalReloadDigest {
    fn source_digest(&self) -> u64;
}

impl OptionalReloadDigest for ValidationCommandSource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}

impl OptionalReloadDigest for ValidationCommandProjectionSource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}
