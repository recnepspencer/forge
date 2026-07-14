use std::error::Error;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTestBackendError {
    kind: WorthQueryTestBackendErrorKind,
    message: String,
}

impl WorthQueryTestBackendError {
    pub(crate) fn new(kind: WorthQueryTestBackendErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryTestBackendErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for WorthQueryTestBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for WorthQueryTestBackendError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTestBackendErrorKind {
    BlankCollectionName,
    BlankAspectLabel,
    InvalidAspectLabel,
    BlankProjectionPath,
    InvalidProjectionPath,
    DuplicateAspectLabel,
    DuplicateProjectionPath,
    EmptyAspectSet,
    MissingSchema,
    InvariantRegistrationFailed,
    DomainInstallationFailed,
    WorkspaceBuildFailed,
}
