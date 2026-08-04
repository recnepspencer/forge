#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendCapabilityKind {
    BufferedFile,
    DirectIo,
    Mmap,
    AsyncIo,
    Fsync,
    DirectorySync,
    DurableRename,
    SecureFrameIo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendTargetProfile {
    SimulatedStrictDurable,
    PosixFileFsyncDirSync,
    WindowsFlushFileBuffers,
    MmapFlushNotDurabilityCertified,
    AdversarialLostFlush,
    AdversarialReorderedFlush,
}

impl BackendTargetProfile {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SimulatedStrictDurable => "simulated-strict-durable",
            Self::PosixFileFsyncDirSync => "posix-file-fsync-directory-sync",
            Self::WindowsFlushFileBuffers => "windows-flush-file-buffers",
            Self::MmapFlushNotDurabilityCertified => "mmap-flush-not-durability-certified",
            Self::AdversarialLostFlush => "controlled-lost-flush",
            Self::AdversarialReorderedFlush => "controlled-reordered-flush",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityEvidenceClass {
    DeclaredByConfig,
    ObservedByProbe,
    EstablishedByFilesystemAdmission,
    ExternallyGuaranteed,
    UnverifiableAssumption,
    CertifiedBackendProfile,
}

impl CapabilityEvidenceClass {
    pub const fn satisfies(self, required: Self) -> bool {
        matches!(
            (self, required),
            (Self::CertifiedBackendProfile, Self::CertifiedBackendProfile)
                | (Self::CertifiedBackendProfile, Self::ExternallyGuaranteed)
                | (
                    Self::CertifiedBackendProfile,
                    Self::EstablishedByFilesystemAdmission
                )
                | (Self::CertifiedBackendProfile, Self::ObservedByProbe)
                | (Self::CertifiedBackendProfile, Self::DeclaredByConfig)
                | (Self::ExternallyGuaranteed, Self::ExternallyGuaranteed)
                | (Self::ExternallyGuaranteed, Self::ObservedByProbe)
                | (Self::ExternallyGuaranteed, Self::DeclaredByConfig)
                | (Self::ObservedByProbe, Self::ObservedByProbe)
                | (Self::ObservedByProbe, Self::DeclaredByConfig)
                | (
                    Self::EstablishedByFilesystemAdmission,
                    Self::EstablishedByFilesystemAdmission
                )
                | (
                    Self::EstablishedByFilesystemAdmission,
                    Self::ObservedByProbe
                )
                | (
                    Self::EstablishedByFilesystemAdmission,
                    Self::DeclaredByConfig
                )
                | (Self::DeclaredByConfig, Self::DeclaredByConfig)
                | (Self::UnverifiableAssumption, Self::UnverifiableAssumption)
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendCapabilitySupportPosture {
    Supported,
    Unsupported,
    Unavailable,
    Unknown,
    Stale,
    RebindRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityConfidenceScope {
    BackendProfile,
    BackendAndMedia,
    CertificationOnly,
    UnboundedAssumption,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityResidualRisk {
    None,
    Bounded,
    Unverifiable,
}
