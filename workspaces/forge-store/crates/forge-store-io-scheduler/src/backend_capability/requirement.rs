use forge_store_physical_backend::{BackendCapabilityKind, CapabilityEvidenceClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoSchedulerBackendCapabilityRequirement {
    BufferedFile,
    DirectIo,
    Mmap,
    AsyncIo,
    Fsync,
    DirectorySync,
    DurableRename,
    SecureFrameIo,
}

impl IoSchedulerBackendCapabilityRequirement {
    pub const fn capability_kind(self) -> BackendCapabilityKind {
        match self {
            Self::BufferedFile => BackendCapabilityKind::BufferedFile,
            Self::DirectIo => BackendCapabilityKind::DirectIo,
            Self::Mmap => BackendCapabilityKind::Mmap,
            Self::AsyncIo => BackendCapabilityKind::AsyncIo,
            Self::Fsync => BackendCapabilityKind::Fsync,
            Self::DirectorySync => BackendCapabilityKind::DirectorySync,
            Self::DurableRename => BackendCapabilityKind::DurableRename,
            Self::SecureFrameIo => BackendCapabilityKind::SecureFrameIo,
        }
    }

    pub const fn required_evidence(self) -> CapabilityEvidenceClass {
        match self {
            Self::BufferedFile => CapabilityEvidenceClass::DeclaredByConfig,
            Self::DirectIo
            | Self::Mmap
            | Self::AsyncIo
            | Self::Fsync
            | Self::DirectorySync
            | Self::DurableRename
            | Self::SecureFrameIo => CapabilityEvidenceClass::ExternallyGuaranteed,
        }
    }
}
