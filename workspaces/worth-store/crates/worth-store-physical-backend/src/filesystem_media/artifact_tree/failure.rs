#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactTreeFailureKind {
    Absent,
    AlreadyExists,
    AccessLimitExceeded,
    DeniedBeforeEffect,
    PartialWrite { completed_bytes: u64 },
    IndeterminateEffect,
    Damaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactTreeFailure {
    kind: ArtifactTreeFailureKind,
    io_kind: Option<std::io::ErrorKind>,
    access_limit: Option<ArtifactTreeAccessLimit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactTreeAccessLimit {
    pub observed: u64,
    pub admitted: u64,
}

impl ArtifactTreeFailure {
    pub const fn kind(self) -> ArtifactTreeFailureKind {
        self.kind
    }

    pub const fn io_kind(self) -> Option<std::io::ErrorKind> {
        self.io_kind
    }

    pub const fn access_limit(self) -> Option<ArtifactTreeAccessLimit> {
        self.access_limit
    }

    #[cfg(feature = "recovery-runtime-owner")]
    pub(crate) const fn recovery_denial() -> Self {
        Self::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
    }

    pub(in crate::filesystem_media) fn io(
        kind: ArtifactTreeFailureKind,
        error: &std::io::Error,
    ) -> Self {
        Self {
            kind,
            io_kind: Some(error.kind()),
            access_limit: None,
        }
    }

    pub(in crate::filesystem_media) const fn structural(kind: ArtifactTreeFailureKind) -> Self {
        Self {
            kind,
            io_kind: None,
            access_limit: None,
        }
    }

    pub(in crate::filesystem_media) const fn limit(observed: u64, admitted: u64) -> Self {
        Self {
            kind: ArtifactTreeFailureKind::AccessLimitExceeded,
            io_kind: None,
            access_limit: Some(ArtifactTreeAccessLimit { observed, admitted }),
        }
    }
}
