use std::error::Error;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendError {
    kind: ForgeQueryTestBackendErrorKind,
    message: String,
}

impl ForgeQueryTestBackendError {
    pub(crate) fn new(kind: ForgeQueryTestBackendErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryTestBackendErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for ForgeQueryTestBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ForgeQueryTestBackendError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryTestBackendErrorKind {
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
    WorkspaceBuildFailed,
}
