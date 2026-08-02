use worth_store_physical_backend::{BackendCapabilityKind, CapabilityEvidenceClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoSchedulerBackendCapabilityRequirement {
    BufferedFile,
    DirectIo,
    Mmap,
    AsyncIo,
    Fsync,
    FilesystemAdmittedFsync,
    DirectorySync,
    FilesystemAdmittedDirectorySync,
    DurableRename,
    FilesystemAdmittedDurableRename,
    SecureFrameIo,
}

impl IoSchedulerBackendCapabilityRequirement {
    pub const fn capability_kind(self) -> BackendCapabilityKind {
        match self {
            Self::BufferedFile => BackendCapabilityKind::BufferedFile,
            Self::DirectIo => BackendCapabilityKind::DirectIo,
            Self::Mmap => BackendCapabilityKind::Mmap,
            Self::AsyncIo => BackendCapabilityKind::AsyncIo,
            Self::Fsync | Self::FilesystemAdmittedFsync => BackendCapabilityKind::Fsync,
            Self::DirectorySync | Self::FilesystemAdmittedDirectorySync => {
                BackendCapabilityKind::DirectorySync
            }
            Self::DurableRename | Self::FilesystemAdmittedDurableRename => {
                BackendCapabilityKind::DurableRename
            }
            Self::SecureFrameIo => BackendCapabilityKind::SecureFrameIo,
        }
    }

    pub const fn required_evidence(self) -> CapabilityEvidenceClass {
        match self {
            Self::BufferedFile => CapabilityEvidenceClass::DeclaredByConfig,
            Self::FilesystemAdmittedFsync
            | Self::FilesystemAdmittedDirectorySync
            | Self::FilesystemAdmittedDurableRename => {
                CapabilityEvidenceClass::EstablishedByFilesystemAdmission
            }
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
