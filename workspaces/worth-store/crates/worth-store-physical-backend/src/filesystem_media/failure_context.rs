use super::{MediaHandleIdentity, MediaOperationRole};
use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaOsCodeFamily {
    UnixErrno,
    WindowsSystem,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOsCode {
    family: MediaOsCodeFamily,
    value: i64,
}

impl MediaOsCode {
    pub const fn family(self) -> MediaOsCodeFamily {
        self.family
    }
    pub const fn value(self) -> i64 {
        self.value
    }

    #[cfg(test)]
    pub(super) const fn for_test(family: MediaOsCodeFamily, value: i64) -> Self {
        Self { family, value }
    }

    pub(super) fn from_io(error: &std::io::Error) -> Option<Self> {
        error.raw_os_error().map(|value| Self {
            family: if cfg!(windows) {
                MediaOsCodeFamily::WindowsSystem
            } else if cfg!(unix) {
                MediaOsCodeFamily::UnixErrno
            } else {
                MediaOsCodeFamily::Other
            },
            value: i64::from(value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaPathRole {
    Namespace(StoreNamespaceRelativeRole),
    ArtifactFamilyRoot,
    ArtifactOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCausalBoundary {
    BeforeOsCall,
    OsCallReturned,
    PrefixObserved,
    CompletionUnconfirmed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaFailureContext {
    operation: MediaOperationRole,
    path_role: MediaPathRole,
    handle: Option<MediaHandleIdentity>,
    io_kind: Option<std::io::ErrorKind>,
    os_code: Option<MediaOsCode>,
    causal_boundary: MediaCausalBoundary,
}

impl MediaFailureContext {
    pub const fn operation(self) -> MediaOperationRole {
        self.operation
    }
    pub const fn path_role(self) -> MediaPathRole {
        self.path_role
    }
    pub const fn handle(self) -> Option<MediaHandleIdentity> {
        self.handle
    }
    pub const fn os_code(self) -> Option<MediaOsCode> {
        self.os_code
    }
    pub const fn io_kind(self) -> Option<std::io::ErrorKind> {
        self.io_kind
    }
    pub const fn causal_boundary(self) -> MediaCausalBoundary {
        self.causal_boundary
    }

    #[cfg(test)]
    pub(super) const fn for_test(
        operation: MediaOperationRole,
        path_role: MediaPathRole,
        handle: Option<MediaHandleIdentity>,
        os_code: Option<MediaOsCode>,
        causal_boundary: MediaCausalBoundary,
    ) -> Self {
        Self {
            operation,
            path_role,
            handle,
            io_kind: None,
            os_code,
            causal_boundary,
        }
    }

    pub(super) const fn new(
        operation: MediaOperationRole,
        path_role: MediaPathRole,
        handle: Option<MediaHandleIdentity>,
        io_kind: Option<std::io::ErrorKind>,
        os_code: Option<MediaOsCode>,
        causal_boundary: MediaCausalBoundary,
    ) -> Self {
        Self {
            operation,
            path_role,
            handle,
            io_kind,
            os_code,
            causal_boundary,
        }
    }
}

pub(super) fn operation_failure(
    identity: super::MediaOperationIdentity,
    operation: MediaOperationRole,
    path_role: MediaPathRole,
    handle: Option<MediaHandleIdentity>,
    kind: super::MediaOperationFailureKind,
    error: Option<&std::io::Error>,
    causal_boundary: MediaCausalBoundary,
) -> super::MediaOperationFailure {
    super::MediaOperationFailure::new(
        identity,
        kind,
        MediaFailureContext::new(
            operation,
            path_role,
            handle,
            error.map(std::io::Error::kind),
            error.and_then(MediaOsCode::from_io),
            causal_boundary,
        ),
    )
}

pub(super) const fn causal_boundary(error: Option<&std::io::Error>) -> MediaCausalBoundary {
    if error.is_some() {
        MediaCausalBoundary::OsCallReturned
    } else {
        MediaCausalBoundary::BeforeOsCall
    }
}
