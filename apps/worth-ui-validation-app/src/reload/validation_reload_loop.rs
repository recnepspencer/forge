mod config;

use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

use super::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationLiveViewSource,
    ValidationObservedAuthoredBatch, ValidationReloadInput, ValidationReloadInputDenial,
    ValidationReloadObservation, ValidationReloadTick, ValidationSourcePackage,
    ValidationThemeSource,
};

pub use config::ValidationReloadLoopConfig;

#[derive(Debug)]
pub struct ValidationReloadLoop {
    config: ValidationReloadLoopConfig,
    pending_inputs: VecDeque<ValidationReloadInput>,
    last_source_digest: u64,
    last_theme_digest: u64,
    last_command_digest: Option<u64>,
    last_command_projection_digest: Option<u64>,
    last_component_digest: Option<u64>,
    last_appearance_digest: Option<u64>,
    last_density_digest: Option<u64>,
    last_live_view_digest: Option<u64>,
}

impl ValidationReloadLoop {
    pub fn start(config: ValidationReloadLoopConfig) -> Result<Self, ValidationReloadInputDenial> {
        let theme = read_theme_source(config.theme_path())?;
        let command = read_optional_command_source(config.command_path.as_ref())?;
        let command_projection =
            read_optional_command_projection_source(config.command_projection_path.as_ref())?;
        let component = read_optional_component_source(config.component_path.as_ref())?;
        let appearance = read_optional_appearance_source(config.appearance_path.as_ref())?;
        let density = read_optional_density_source(config.density_path.as_ref())?;
        let live_view = read_optional_live_view_source(config.live_view_path.as_ref())?;
        Ok(Self {
            pending_inputs: VecDeque::new(),
            last_source_digest: config.initial_source().source_digest(),
            last_theme_digest: config.initial_theme.as_ref().map_or_else(
                || theme.source_digest(),
                ValidationThemeSource::source_digest,
            ),
            last_command_digest: config
                .initial_command
                .as_ref()
                .map(ValidationCommandSource::source_digest)
                .or_else(|| command.as_ref().map(ValidationCommandSource::source_digest)),
            last_command_projection_digest: config
                .initial_command_projection
                .as_ref()
                .map(ValidationCommandProjectionSource::source_digest)
                .or_else(|| {
                    command_projection
                        .as_ref()
                        .map(ValidationCommandProjectionSource::source_digest)
                }),
            last_component_digest: config
                .initial_component
                .as_ref()
                .map(ValidationComponentSource::source_digest)
                .or_else(|| {
                    component
                        .as_ref()
                        .map(ValidationComponentSource::source_digest)
                }),
            last_appearance_digest: config
                .initial_appearance
                .as_ref()
                .map(ValidationAppearanceSource::source_digest)
                .or_else(|| {
                    appearance
                        .as_ref()
                        .map(ValidationAppearanceSource::source_digest)
                }),
            last_density_digest: config
                .initial_density
                .as_ref()
                .map(ValidationDensitySource::source_digest)
                .or_else(|| density.as_ref().map(ValidationDensitySource::source_digest)),
            last_live_view_digest: config
                .initial_live_view
                .as_ref()
                .map(ValidationLiveViewSource::source_digest)
                .or_else(|| {
                    live_view
                        .as_ref()
                        .map(ValidationLiveViewSource::source_digest)
                }),
            config,
        })
    }

    pub fn poll_inputs(&mut self) -> ValidationReloadTick {
        if let Some(input) = self.pending_inputs.pop_front() {
            return ValidationReloadTick::Changed(input);
        }
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
        let component = match read_optional_component_source(self.config.component_path.as_ref()) {
            Ok(component) => component,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };
        let appearance = match read_optional_appearance_source(self.config.appearance_path.as_ref())
        {
            Ok(appearance) => appearance,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };
        let density = match read_optional_density_source(self.config.density_path.as_ref()) {
            Ok(density) => density,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };
        let live_view = match read_optional_live_view_source(self.config.live_view_path.as_ref()) {
            Ok(live_view) => live_view,
            Err(denial) => return ValidationReloadTick::Unreadable(denial),
        };

        let source_changed = source.source_digest() != self.last_source_digest;
        let theme_changed = theme.source_digest() != self.last_theme_digest;
        let command_changed = optional_digest_changed(&command, self.last_command_digest);
        let command_projection_changed =
            optional_digest_changed(&command_projection, self.last_command_projection_digest);
        let component_changed = optional_digest_changed(&component, self.last_component_digest);
        let appearance_changed = optional_digest_changed(&appearance, self.last_appearance_digest);
        let density_changed = optional_digest_changed(&density, self.last_density_digest);
        let live_view_changed = optional_digest_changed(&live_view, self.last_live_view_digest);
        self.last_source_digest = source.source_digest();
        self.last_theme_digest = theme.source_digest();
        self.last_command_digest = command.as_ref().map(ValidationCommandSource::source_digest);
        self.last_command_projection_digest = command_projection
            .as_ref()
            .map(ValidationCommandProjectionSource::source_digest);
        self.last_component_digest = component
            .as_ref()
            .map(ValidationComponentSource::source_digest);
        self.last_appearance_digest = appearance
            .as_ref()
            .map(ValidationAppearanceSource::source_digest);
        self.last_density_digest = density.as_ref().map(ValidationDensitySource::source_digest);
        self.last_live_view_digest = live_view
            .as_ref()
            .map(ValidationLiveViewSource::source_digest);

        let mut changed_inputs = VecDeque::new();
        let capability_changed = command_changed
            || command_projection_changed
            || component_changed
            || appearance_changed
            || density_changed
            || theme_changed;
        if source_changed && capability_changed {
            return ValidationReloadTick::Changed(ValidationReloadInput::ObservedAuthoredBatch(
                ValidationObservedAuthoredBatch::new(
                    source,
                    theme_changed.then_some(theme),
                    command_changed.then_some(command).flatten(),
                    command_projection_changed
                        .then_some(command_projection)
                        .flatten(),
                    component_changed.then_some(component).flatten(),
                    appearance_changed.then_some(appearance).flatten(),
                    density_changed.then_some(density).flatten(),
                ),
            ));
        }
        if command_changed {
            let Some(command) = command else {
                unreachable!("changed optional command source must exist");
            };
            changed_inputs.push_back(ValidationReloadInput::HeaderCommands(command));
        }
        if live_view_changed {
            let Some(live_view) = live_view else {
                unreachable!("changed optional live-view source must exist");
            };
            changed_inputs.push_back(ValidationReloadInput::LiveViewSource(live_view));
        }
        if command_projection_changed {
            let Some(command_projection) = command_projection else {
                unreachable!("changed optional command projection source must exist");
            };
            changed_inputs.push_back(ValidationReloadInput::HeaderCommandProjections(
                command_projection,
            ));
        }
        if component_changed {
            let Some(component) = component else {
                unreachable!("changed optional component source must exist");
            };
            changed_inputs.push_back(ValidationReloadInput::HeaderComponents(component));
        }
        if appearance_changed {
            if density_changed {
                let Some(appearance) = appearance else {
                    unreachable!("changed optional appearance source must exist");
                };
                let Some(density) = density else {
                    unreachable!("changed optional density source must exist");
                };
                changed_inputs.push_back(ValidationReloadInput::HeaderAppearanceAndDensity {
                    appearance,
                    density,
                });
            } else {
                let Some(appearance) = appearance else {
                    unreachable!("changed optional appearance source must exist");
                };
                changed_inputs.push_back(ValidationReloadInput::HeaderAppearance(appearance));
            }
        } else if density_changed {
            let Some(density) = density else {
                unreachable!("changed optional density source must exist");
            };
            changed_inputs.push_back(ValidationReloadInput::HeaderDensity(density));
        }

        match (source_changed, theme_changed) {
            (false, false) => {}
            (true, false) => {
                changed_inputs.push_back(ValidationReloadInput::SourcePackage(source));
            }
            (false, true) => {
                changed_inputs.push_back(ValidationReloadInput::HeaderTheme(theme));
            }
            (true, true) => {
                changed_inputs.push_back(ValidationReloadInput::SourcePackageAndHeaderTheme {
                    source,
                    theme,
                });
            }
        }

        if let Some(input) = changed_inputs.pop_front() {
            self.pending_inputs = changed_inputs;
            ValidationReloadTick::Changed(input)
        } else {
            ValidationReloadTick::Unchanged(ValidationReloadObservation::new(
                self.last_source_digest,
                self.last_theme_digest,
            ))
        }
    }

    fn read_source_package(&self) -> Result<ValidationSourcePackage, ValidationReloadInputDenial> {
        let Some(source_path) = self.config.source_path() else {
            return Ok(self.config.initial_source().clone());
        };
        let source_text = fs::read_to_string(source_path)
            .map_err(|error| ValidationReloadInputDenial::unreadable(source_path, &error))?;
        Ok(ValidationSourcePackage::new(
            self.config.initial_source().module_path(),
            source_text,
        ))
    }
}

fn read_theme_source(path: &PathBuf) -> Result<ValidationThemeSource, ValidationReloadInputDenial> {
    fs::read_to_string(path)
        .map(|source_text| ValidationThemeSource::from_observed_file(path, source_text))
        .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
}

fn read_optional_command_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationCommandSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| ValidationCommandSource::from_observed_file(path, source_text))
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn read_optional_command_projection_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationCommandProjectionSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| {
                ValidationCommandProjectionSource::from_observed_file(path, source_text)
            })
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

impl OptionalReloadDigest for ValidationAppearanceSource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}

impl OptionalReloadDigest for ValidationComponentSource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}

impl OptionalReloadDigest for ValidationDensitySource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}

impl OptionalReloadDigest for ValidationLiveViewSource {
    fn source_digest(&self) -> u64 {
        self.source_digest()
    }
}

fn read_optional_component_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationComponentSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| ValidationComponentSource::from_observed_file(path, source_text))
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn read_optional_appearance_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationAppearanceSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| ValidationAppearanceSource::from_observed_file(path, source_text))
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn read_optional_density_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationDensitySource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| ValidationDensitySource::from_observed_file(path, source_text))
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}

fn read_optional_live_view_source(
    path: Option<&PathBuf>,
) -> Result<Option<ValidationLiveViewSource>, ValidationReloadInputDenial> {
    path.map(|path| {
        fs::read_to_string(path)
            .map(|source_text| ValidationLiveViewSource::from_observed_file(path, source_text))
            .map_err(|error| ValidationReloadInputDenial::unreadable(path, &error))
    })
    .transpose()
}
