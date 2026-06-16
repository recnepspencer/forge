use std::fmt;
use std::path::PathBuf;

use super::{
    ValidationCommandProjectionSource, ValidationCommandSource, ValidationSourcePackage,
    ValidationThemeSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationReloadInput {
    SourcePackage(ValidationSourcePackage),
    HeaderTheme(ValidationThemeSource),
    HeaderCommands(ValidationCommandSource),
    HeaderCommandProjections(ValidationCommandProjectionSource),
    SourcePackageAndHeaderTheme {
        source: ValidationSourcePackage,
        theme: ValidationThemeSource,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadInputDenial {
    path: PathBuf,
    reason: String,
}

impl ValidationReloadInput {
    pub fn source_digest(&self) -> Option<u64> {
        match self {
            Self::SourcePackage(source) => Some(source.source_digest()),
            Self::HeaderTheme(_) => None,
            Self::HeaderCommands(_) => None,
            Self::HeaderCommandProjections(_) => None,
            Self::SourcePackageAndHeaderTheme { source, .. } => Some(source.source_digest()),
        }
    }

    pub fn theme_digest(&self) -> Option<u64> {
        match self {
            Self::SourcePackage(_) => None,
            Self::HeaderTheme(theme) => Some(theme.source_digest()),
            Self::HeaderCommands(_) => None,
            Self::HeaderCommandProjections(_) => None,
            Self::SourcePackageAndHeaderTheme { theme, .. } => Some(theme.source_digest()),
        }
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
