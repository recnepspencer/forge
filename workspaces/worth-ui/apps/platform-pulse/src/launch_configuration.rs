use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use worth_ui_platform_pulse::observation_contract::PlatformPulseLaunchConfigurationDenial;

const ENTRY_SOURCE: &str = "main.wui";
const INTENT_SOURCE: &str = "platform-pulse-intent.json";

#[cfg(test)]
mod tests;

pub(crate) struct AdmittedPlatformPulseLaunchConfiguration {
    source_root: PathBuf,
    query_source_root: PathBuf,
    intent_source_root: PathBuf,
}

impl AdmittedPlatformPulseLaunchConfiguration {
    pub(crate) fn from_process() -> Result<Self, PlatformPulseLaunchConfigurationDenial> {
        admit(
            std::env::args_os().skip(1),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("app"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("query_samples"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("intent_samples"),
        )
    }

    pub(crate) fn source_root(&self) -> &Path {
        &self.source_root
    }

    pub(crate) fn query_source_root(&self) -> &Path {
        &self.query_source_root
    }

    pub(crate) fn intent_source_root(&self) -> &Path {
        &self.intent_source_root
    }
}

fn admit(
    arguments: impl IntoIterator<Item = OsString>,
    default_source_root: PathBuf,
    default_query_source_root: PathBuf,
    default_intent_source_root: PathBuf,
) -> Result<AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial> {
    let mut arguments = arguments.into_iter();
    let mut source_root = None;
    let mut query_source_root = None;
    let mut intent_source_root = None;
    let mut admitted_option = false;
    while let Some(option) = arguments.next() {
        let (target, missing) = if option == OsStr::new("--source-root") && source_root.is_none() {
            (&mut source_root, MissingValue::Source)
        } else if option == OsStr::new("--query-source-root") && query_source_root.is_none() {
            (&mut query_source_root, MissingValue::Query)
        } else if option == OsStr::new("--intent-source-root") && intent_source_root.is_none() {
            (&mut intent_source_root, MissingValue::Intent)
        } else {
            return Err(if admitted_option {
                PlatformPulseLaunchConfigurationDenial::SurplusArgument
            } else {
                PlatformPulseLaunchConfigurationDenial::UnexpectedArgument
            });
        };
        *target = Some(arguments.next().ok_or_else(|| missing.denial())?.into());
        admitted_option = true;
    }
    admit_roots(
        source_root.unwrap_or(default_source_root),
        query_source_root.unwrap_or(default_query_source_root),
        intent_source_root.unwrap_or(default_intent_source_root),
    )
}

#[derive(Clone, Copy)]
enum MissingValue {
    Source,
    Query,
    Intent,
}

impl MissingValue {
    fn denial(self) -> PlatformPulseLaunchConfigurationDenial {
        match self {
            Self::Source => PlatformPulseLaunchConfigurationDenial::MissingSourceRootValue,
            Self::Query => PlatformPulseLaunchConfigurationDenial::MissingQuerySourceRootValue,
            Self::Intent => PlatformPulseLaunchConfigurationDenial::MissingIntentSourceRootValue,
        }
    }
}

fn admit_roots(
    source_root: PathBuf,
    query_source_root: PathBuf,
    intent_source_root: PathBuf,
) -> Result<AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial> {
    admit_source_root(&source_root)?;
    admit_query_root(&query_source_root)?;
    admit_intent_root(&intent_source_root)?;
    Ok(AdmittedPlatformPulseLaunchConfiguration {
        source_root,
        query_source_root,
        intent_source_root,
    })
}

fn admit_source_root(root: &Path) -> Result<(), PlatformPulseLaunchConfigurationDenial> {
    if !root.is_absolute() {
        return Err(PlatformPulseLaunchConfigurationDenial::RelativeSourceRoot(
            root.to_owned(),
        ));
    }
    match root.try_exists() {
        Ok(false) => {
            return Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRoot(
                root.to_owned(),
            ))
        }
        Err(_) => {
            return Err(
                PlatformPulseLaunchConfigurationDenial::SourceRootMetadataUnavailable(
                    root.to_owned(),
                ),
            )
        }
        Ok(true) => {}
    }
    let metadata = std::fs::metadata(root).map_err(|_| {
        PlatformPulseLaunchConfigurationDenial::SourceRootMetadataUnavailable(root.to_owned())
    })?;
    if !metadata.is_dir() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::SourceRootNotDirectory(root.to_owned()),
        );
    }
    let entry = root.join(ENTRY_SOURCE);
    if !entry.is_file() {
        return Err(PlatformPulseLaunchConfigurationDenial::MissingEntrySource(
            entry,
        ));
    }
    Ok(())
}

fn admit_query_root(root: &Path) -> Result<(), PlatformPulseLaunchConfigurationDenial> {
    if !root.is_absolute() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::RelativeQuerySourceRoot(root.to_owned()),
        );
    }
    let metadata = std::fs::metadata(root).map_err(|_| {
        PlatformPulseLaunchConfigurationDenial::QuerySourceRootMetadataUnavailable(root.to_owned())
    })?;
    if !metadata.is_dir() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::QuerySourceRootNotDirectory(root.to_owned()),
        );
    }
    Ok(())
}

fn admit_intent_root(root: &Path) -> Result<(), PlatformPulseLaunchConfigurationDenial> {
    if !root.is_absolute() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::RelativeIntentSourceRoot(root.to_owned()),
        );
    }
    let metadata = std::fs::metadata(root).map_err(|_| {
        PlatformPulseLaunchConfigurationDenial::IntentSourceRootMetadataUnavailable(root.to_owned())
    })?;
    if !metadata.is_dir() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::IntentSourceRootNotDirectory(root.to_owned()),
        );
    }
    let input = root.join(INTENT_SOURCE);
    if !input.is_file() {
        return Err(PlatformPulseLaunchConfigurationDenial::MissingIntentSource(
            input,
        ));
    }
    Ok(())
}
