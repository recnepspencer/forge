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
