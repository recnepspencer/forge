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
}

impl ArtifactTreeFailure {
    pub const fn kind(self) -> ArtifactTreeFailureKind {
        self.kind
    }

    pub const fn io_kind(self) -> Option<std::io::ErrorKind> {
        self.io_kind
    }

    pub(in crate::filesystem_media) fn io(
        kind: ArtifactTreeFailureKind,
        error: &std::io::Error,
    ) -> Self {
        Self {
            kind,
            io_kind: Some(error.kind()),
        }
    }

    pub(in crate::filesystem_media) const fn structural(kind: ArtifactTreeFailureKind) -> Self {
        Self {
            kind,
            io_kind: None,
        }
    }
}
