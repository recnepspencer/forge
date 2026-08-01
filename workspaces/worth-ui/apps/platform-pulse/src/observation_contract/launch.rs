use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformPulseLaunchConfigurationDenial {
    UnexpectedArgument,
    MissingSourceRootValue,
    MissingQuerySourceRootValue,
    MissingIntentSourceRootValue,
    SurplusArgument,
    RelativeSourceRoot(PathBuf),
    MissingSourceRoot(PathBuf),
    SourceRootMetadataUnavailable(PathBuf),
    SourceRootNotDirectory(PathBuf),
    MissingEntrySource(PathBuf),
    RelativeQuerySourceRoot(PathBuf),
    QuerySourceRootMetadataUnavailable(PathBuf),
    QuerySourceRootNotDirectory(PathBuf),
    RelativeIntentSourceRoot(PathBuf),
    IntentSourceRootMetadataUnavailable(PathBuf),
    IntentSourceRootNotDirectory(PathBuf),
    MissingIntentSource(PathBuf),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseLaunchConfigurationDenialKind {
    UnexpectedArgument,
    MissingSourceRootValue,
    MissingQuerySourceRootValue,
    MissingIntentSourceRootValue,
    SurplusArgument,
    RelativeSourceRoot,
    MissingSourceRoot,
    SourceRootMetadataUnavailable,
    SourceRootNotDirectory,
    MissingEntrySource,
    RelativeQuerySourceRoot,
    QuerySourceRootMetadataUnavailable,
    QuerySourceRootNotDirectory,
    RelativeIntentSourceRoot,
    IntentSourceRootMetadataUnavailable,
    IntentSourceRootNotDirectory,
    MissingIntentSource,
}

impl PlatformPulseLaunchConfigurationDenial {
    pub fn kind(&self) -> PlatformPulseLaunchConfigurationDenialKind {
        match self {
            Self::UnexpectedArgument => Kind::UnexpectedArgument,
            Self::MissingSourceRootValue => Kind::MissingSourceRootValue,
            Self::MissingQuerySourceRootValue => Kind::MissingQuerySourceRootValue,
            Self::MissingIntentSourceRootValue => Kind::MissingIntentSourceRootValue,
            Self::SurplusArgument => Kind::SurplusArgument,
            Self::RelativeSourceRoot(_) => Kind::RelativeSourceRoot,
            Self::MissingSourceRoot(_) => Kind::MissingSourceRoot,
            Self::SourceRootMetadataUnavailable(_) => Kind::SourceRootMetadataUnavailable,
            Self::SourceRootNotDirectory(_) => Kind::SourceRootNotDirectory,
            Self::MissingEntrySource(_) => Kind::MissingEntrySource,
            Self::RelativeQuerySourceRoot(_) => Kind::RelativeQuerySourceRoot,
            Self::QuerySourceRootMetadataUnavailable(_) => Kind::QuerySourceRootMetadataUnavailable,
            Self::QuerySourceRootNotDirectory(_) => Kind::QuerySourceRootNotDirectory,
            Self::RelativeIntentSourceRoot(_) => Kind::RelativeIntentSourceRoot,
            Self::IntentSourceRootMetadataUnavailable(_) => {
                Kind::IntentSourceRootMetadataUnavailable
            }
            Self::IntentSourceRootNotDirectory(_) => Kind::IntentSourceRootNotDirectory,
            Self::MissingIntentSource(_) => Kind::MissingIntentSource,
        }
    }
}

use PlatformPulseLaunchConfigurationDenialKind as Kind;
