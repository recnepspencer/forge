#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaQualificationDenial {
    UnmanagedWriterPosture {
        counters: Box<super::MediaCounterSnapshot>,
    },
    OwnerPreEffect {
        denial: super::FilesystemMediaOwnerAdmissionDenial,
        release: Option<super::OwnershipReleaseOutcome>,
        counters: Box<super::MediaCounterSnapshot>,
    },
    RemoteFilesystem {
        counters: Box<super::MediaCounterSnapshot>,
    },
    RemovableFilesystem {
        counters: Box<super::MediaCounterSnapshot>,
    },
    ReadOnlyFilesystem {
        counters: Box<super::MediaCounterSnapshot>,
    },
    UnknownFilesystem {
        counters: Box<super::MediaCounterSnapshot>,
    },
    UserspaceFilesystem {
        filesystem: Box<str>,
        counters: Box<super::MediaCounterSnapshot>,
    },
    DamagedIdentity {
        counters: Box<super::MediaCounterSnapshot>,
    },
    Capability {
        denial: crate::BackendCapabilityAdmissionDenial,
        counters: Box<super::MediaCounterSnapshot>,
    },
}

impl MediaQualificationDenial {
    pub const fn release(&self) -> Option<super::OwnershipReleaseOutcome> {
        match self {
            Self::OwnerPreEffect { release, .. } => *release,
            _ => None,
        }
    }

    pub const fn counters(&self) -> &super::MediaCounterSnapshot {
        match self {
            Self::UnmanagedWriterPosture { counters }
            | Self::OwnerPreEffect { counters, .. }
            | Self::RemoteFilesystem { counters }
            | Self::RemovableFilesystem { counters }
            | Self::ReadOnlyFilesystem { counters }
            | Self::UnknownFilesystem { counters }
            | Self::UserspaceFilesystem { counters, .. }
            | Self::DamagedIdentity { counters }
            | Self::Capability { counters, .. } => counters,
        }
    }

    pub(super) fn with_terminal_counters(mut self, terminal: super::MediaCounterSnapshot) -> Self {
        match &mut self {
            Self::UnmanagedWriterPosture { counters }
            | Self::OwnerPreEffect { counters, .. }
            | Self::RemoteFilesystem { counters }
            | Self::RemovableFilesystem { counters }
            | Self::ReadOnlyFilesystem { counters }
            | Self::UnknownFilesystem { counters }
            | Self::UserspaceFilesystem { counters, .. }
            | Self::DamagedIdentity { counters }
            | Self::Capability { counters, .. } => **counters = terminal,
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaQualificationDeferred {
    MutationOwnerContended {
        counters: super::MediaCounterSnapshot,
    },
}

impl MediaQualificationDeferred {
    pub const fn counters(self) -> super::MediaCounterSnapshot {
        match self {
            Self::MutationOwnerContended { counters } => counters,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaQualificationStale {
    RootUnavailable {
        kind: std::io::ErrorKind,
        counters: super::MediaCounterSnapshot,
    },
    RootIdentityChanged {
        observed: [u8; 32],
        counters: super::MediaCounterSnapshot,
    },
}

impl MediaQualificationStale {
    pub const fn counters(self) -> super::MediaCounterSnapshot {
        match self {
            Self::RootUnavailable { counters, .. } | Self::RootIdentityChanged { counters, .. } => {
                counters
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaQualificationRebindRequired {
    VolumeChanged {
        counters: super::MediaCounterSnapshot,
    },
    BackendProfileChanged {
        counters: super::MediaCounterSnapshot,
    },
    QualificationContractChanged {
        counters: super::MediaCounterSnapshot,
    },
}

impl MediaQualificationRebindRequired {
    pub const fn counters(self) -> super::MediaCounterSnapshot {
        match self {
            Self::VolumeChanged { counters }
            | Self::BackendProfileChanged { counters }
            | Self::QualificationContractChanged { counters } => counters,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaQualificationPostOwnershipCause {
    ProfileObservation {
        kind: std::io::ErrorKind,
    },
    RootIdentityChanged(super::NamespaceConfinementDenial),
    ProfileChanged {
        drift: super::MediaQualificationBasisDrift,
    },
    IdentityPublication,
    IdentityRead,
    QualificationTransaction,
    QualificationIdentityExhausted,
    Denied(Box<MediaQualificationDenial>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaQualificationFailure {
    OwnerAfterEffect {
        denial: super::FilesystemMediaOwnerAdmissionDenial,
        release: Option<super::OwnershipReleaseOutcome>,
        counters: Box<super::MediaCounterSnapshot>,
    },
    PostOwnership {
        cause: Box<MediaQualificationPostOwnershipCause>,
        release: super::OwnershipReleaseOutcome,
        counters: Box<super::MediaCounterSnapshot>,
    },
}

impl MediaQualificationFailure {
    pub const fn counters(&self) -> &super::MediaCounterSnapshot {
        match self {
            Self::OwnerAfterEffect { counters, .. } | Self::PostOwnership { counters, .. } => {
                counters
            }
        }
    }

    pub const fn release(&self) -> Option<super::OwnershipReleaseOutcome> {
        match self {
            Self::OwnerAfterEffect { release, .. } => *release,
            Self::PostOwnership { release, .. } => Some(*release),
        }
    }

    pub const fn post_ownership_cause(&self) -> Option<&MediaQualificationPostOwnershipCause> {
        match self {
            Self::OwnerAfterEffect { .. } => None,
            Self::PostOwnership { cause, .. } => Some(cause),
        }
    }
}
